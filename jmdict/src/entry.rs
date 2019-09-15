use super::JMDictEntryId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JMDictEntry {
    entry_id: JMDictEntryId<'static>,
    kanji_list: Vec<Kanji>,
    reading_list: Vec<Reading>,
    sense_list: Vec<Sense>,
}

impl JMDictEntry {
    pub fn new<'a>(
        entry_id: JMDictEntryId<'a>,
        kanji_list: Vec<Kanji>,
        reading_list: Vec<Reading>,
        sense_list: Vec<Sense>,
    ) -> Self {
        let my_entry_id: JMDictEntryId<'static> = match entry_id {
            JMDictEntryId::Kanji(kanji_cow) => JMDictEntryId::Kanji(kanji_cow.into_owned().into()),
        };
        Self {
            entry_id: my_entry_id,
            kanji_list,
            reading_list,
            sense_list,
        }
    }
    pub fn kanji(&self) -> &[Kanji] {
        &self.kanji_list
    }
    pub fn readings(&self) -> &[Reading] {
        &self.reading_list
    }
    pub fn senses(&self) -> &[Sense] {
        &self.sense_list
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Kanji(String);

impl Kanji {
    pub fn new(kanji_string: String) -> Self {
        Self(kanji_string)
    }
    pub fn string(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reading(String);

impl Reading {
    pub fn new(kana_string: String) -> Self {
        Self(kana_string)
    }
    pub fn string(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sense {
    gloss_list: Vec<Gloss>,
}

impl Sense {
    pub fn new() -> Self {
        Sense {
            gloss_list: Vec::new(),
        }
    }
    pub fn add_gloss(&mut self, gloss: Gloss) {
        self.gloss_list.push(gloss);
    }
    pub fn glosses(&self) -> &[Gloss] {
        &self.gloss_list
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Gloss(String, Option<String>);

impl Gloss {
    pub fn new(gloss_string: String) -> Self {
        Gloss(gloss_string, None)
    }
    pub fn new_with_lang(gloss_string: String, gloss_lang: String) -> Self {
        Gloss(gloss_string, Some(gloss_lang))
    }
    pub fn text(&self) -> &str {
        &self.0
    }
    pub fn lang(&self) -> Option<&str> {
        self.1.as_ref().map(|s| s as &str)
    }
}
