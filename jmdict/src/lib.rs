use std::io::prelude::*;

pub mod entry_id;

pub mod entry;

pub mod prelude {
    pub use super::entry::JMDictEntry;
    pub use super::entry_id::JMDictEntryId;
}

use self::prelude::*;

#[cfg(feature = "reader")]
pub fn with_jmdict_gz_entries<
    'a,
    R: Read,
    I: IntoIterator<Item = JMDictEntryId<'a>>,
    F: FnMut(JMDictEntry),
>(
    reader: R,
    entries_ids: I,
    on_entry: F,
) {
    use flate2::read::GzDecoder;
    use std::io::BufReader;
    let gz_reader = GzDecoder::new(reader);
    let buf_gz_reader = BufReader::new(gz_reader);
    with_jmdict_entries(buf_gz_reader, entries_ids, on_entry);
}

fn lookup_entry_id<'a, F: Fn(&JMDictEntryId<'a>) -> bool>(
    entries_ids: &mut Vec<JMDictEntryId<'a>>,
    f: F,
) -> Option<JMDictEntryId<'a>> {
    if let Some(index) = entries_ids
        .iter()
        .enumerate()
        .skip_while(|(_, entry_id)| !f(entry_id))
        .map(|(i, _)| i)
        .next()
    {
        Some(entries_ids.remove(index))
    } else {
        None
    }
}

#[cfg(feature = "reader")]
pub fn with_jmdict_entries<
    'a,
    R: BufRead,
    I: IntoIterator<Item = JMDictEntryId<'a>>,
    F: FnMut(JMDictEntry),
>(
    buf_reader: R,
    entries_ids: I,
    mut on_entry: F,
) -> usize {
    use quick_xml::events::Event;
    let mut entries_to_lookup: Vec<JMDictEntryId<'a>> = entries_ids.into_iter().collect();
    let mut xml_reader = quick_xml::Reader::from_reader(buf_reader);
    let mut xml_buf = Vec::new();
    let mut total_entry_count = 0usize;
    let mut current_entry_id: Option<JMDictEntryId<'a>> = None;
    let mut reading_entry = false;
    let mut entry_kanji: Vec<entry::Kanji> = Vec::new();
    let mut entry_reading: Vec<entry::Reading> = Vec::new();
    let mut entry_sense: Vec<entry::Sense> = Vec::new();
    let mut _keywords = Vec::new();
    loop {
        const ELEM_ENTRY: &'static [u8] = b"entry";
        const ELEM_KANJI: &'static [u8] = b"k_ele";
        const ELEM_READING: &'static [u8] = b"r_ele";
        const ELEM_SENSE: &'static [u8] = b"sense";
        match xml_reader.read_event(&mut xml_buf) {
            Ok(Event::Start(ref e)) if (!reading_entry && e.name() == ELEM_ENTRY) => {
                reading_entry = true;
                entry_kanji.clear();
                entry_reading.clear();
                entry_sense.clear();
            }
            Ok(Event::Start(ref e)) if reading_entry => match e.name() {
                // kanji element
                ELEM_KANJI => {
                    const ELEM_KEB: &'static [u8] = b"keb";
                    'kanji_inner: loop {
                        match xml_reader.read_event(&mut xml_buf) {
                            Ok(Event::Start(ref e1)) => match e1.name() {
                                ELEM_KEB => {
                                    if let Ok(kanji_text) =
                                        xml_reader.read_text(ELEM_KEB, &mut xml_buf)
                                    {
                                        entry_kanji.push(entry::Kanji::new(kanji_text.clone()));
                                        if current_entry_id.is_none() {
                                            current_entry_id = lookup_entry_id(
                                                &mut entries_to_lookup,
                                                |entry_id| entry_id.match_kanji(&kanji_text),
                                            );
                                        }
                                    }
                                }
                                _ => (),
                            },
                            Ok(Event::Eof) => break 'kanji_inner,
                            Ok(Event::End(ref e1)) if e1.name() == ELEM_KANJI => break 'kanji_inner,
                            Err(e) => panic!(
                                "Error at position {}: {:?}",
                                xml_reader.buffer_position(),
                                e
                            ),
                            _ => (),
                        }
                    }
                }
                // reading element
                ELEM_READING => {
                    const ELEM_REB: &'static [u8] = b"reb";
                    'reading_inner: loop {
                        match xml_reader.read_event(&mut xml_buf) {
                            Ok(Event::Start(ref e1)) => match e1.name() {
                                ELEM_REB => {
                                    if let Ok(reading_text) =
                                        xml_reader.read_text(ELEM_REB, &mut xml_buf)
                                    {
                                        entry_reading
                                            .push(entry::Reading::new(reading_text.clone()));
                                    }
                                }
                                _ => (),
                            },
                            Ok(Event::Eof) => break 'reading_inner,
                            Ok(Event::End(ref e1)) if e1.name() == ELEM_READING => {
                                break 'reading_inner
                            }
                            Err(e) => panic!(
                                "Error at position {}: {:?}",
                                xml_reader.buffer_position(),
                                e
                            ),
                            _ => (),
                        }
                    }
                }
                // sense element
                ELEM_SENSE => {
                    const ELEM_GLOSS: &'static [u8] = b"gloss";
                    const ATTR_GLOSS_LANG: &'static [u8] = b"xml:lang";
                    let mut current_sense = entry::Sense::new();
                    'sense_inner: loop {
                        match xml_reader.read_event(&mut xml_buf) {
                            Ok(Event::Start(ref e1)) => match e1.name() {
                                ELEM_GLOSS => {
                                    let lang_attribute_opt = e1
                                        .attributes()
                                        .filter_map(Result::ok)
                                        .filter(|a| a.key == ATTR_GLOSS_LANG)
                                        .filter_map(|a| std::str::from_utf8(&a.value).map(String::from).ok())
                                        .next();
                                    if let Ok(gloss_text) =
                                        xml_reader.read_text(ELEM_GLOSS, &mut xml_buf)
                                    {
                                        let gloss = if let Some(lang_attribute) = lang_attribute_opt {
                                            entry::Gloss::new_with_lang(gloss_text, lang_attribute)
                                        }
                                        else {
                                            entry::Gloss::new(gloss_text)
                                        };
                                        current_sense.add_gloss(gloss);
                                    }
                                }
                                _ => (),
                            },
                            Ok(Event::End(ref e1)) if e1.name() == ELEM_SENSE => break 'sense_inner,
                            Ok(Event::Eof) => break 'sense_inner,
                            Err(e) => panic!(
                                "Error at position {}: {:?}",
                                xml_reader.buffer_position(),
                                e
                            ),
                            _ => (),
                        }
                    }
                    entry_sense.push(current_sense);
                }
                _ => (),
            },
            Ok(Event::End(ref e)) if reading_entry && (e.name() == ELEM_ENTRY) => {
                if let Some(entry_id) = current_entry_id {
                    let current_entry =
                        JMDictEntry::new(entry_id, entry_kanji, entry_reading, entry_sense);
                    on_entry(current_entry);
                }
                reading_entry = false;
                current_entry_id = None;
                total_entry_count += 1;
                entry_kanji = Vec::new();
                entry_reading = Vec::new();
                entry_sense = Vec::new();
            }
            Ok(Event::DocType(ref doctype_buffer)) => {
                _keywords = read_keywords(doctype_buffer);
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!(
                "Error at position {}: {:?}",
                xml_reader.buffer_position(),
                e
            ),
            _ => (),
        }
    }
    xml_buf.clear();
    total_entry_count
}

struct KeywordDefinition {
    keyword: String,
    definition: String,
}

fn read_keywords(doctype_buffer: &[u8]) -> Vec<KeywordDefinition> {
    use std::io::{BufReader, Cursor};
    let cursor = Cursor::new(doctype_buffer);
    let buf_reader = BufReader::new(cursor);
    let keywords = Vec::new();
    for line_res in buf_reader.lines() {
        match line_res {
            Ok(line) => {
                // TODO
                /*
                if line.starts_with(ENTITY_START_STRING) {
                    let keyword: String;
                    let definition: String;
                    keyword = line.substr(ENTITY_START_STRING.len())
                    keywords.push(KeywordDefinition {
                        keyword,
                        definition,
                    });
                }
                */
            }
            Err(e) => panic!("Error reading keyword: {}", e),
        }
    }
    keywords
}
