use jmdict::prelude::*;
use wasm_bindgen::prelude::*;

pub fn display_word_list(entry_list: &[JMDictEntry]) -> Result<(), JsValue> {
    use typed_html::{html, text};
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let word_list_container = document.get_element_by_id("word-list-container").unwrap();
    word_list_container.class_list().remove_1("scale-out")?;
    word_list_container.class_list().add_1("scale-in")?;
    let collections = document.get_element_by_id("word-list").unwrap();

    let entries_html: Vec<std::boxed::Box<typed_html::elements::li<String>>> =
        entry_list.iter().map(|entry| {
            let main_kanji: &str = entry.kanji().first().map(jmdict::entry::Kanji::string).unwrap_or("");
            let main_reading: &str = entry.readings().first().map(jmdict::entry::Reading::string).unwrap_or("");
            let jisho_url = url::Url::parse("https://jisho.org/word/").and_then(|u| u.join(&main_kanji)).map(|u| String::from(u.as_str())).unwrap_or("#!".into());
            let tangorin_url = url::Url::parse("https://tangorin.com/words/").and_then(|u| u.join(&main_kanji)).map(|u| String::from(u.as_str())).unwrap_or("#!".into());
            html! {
                <li class="collection-item">
                    <div class="row">
                        <div class="col s4 m2"><h5>{ text!( main_kanji ) }</h5><h6 class="grey-text">{ text!( main_reading ) }</h6></div>
                        <div class="col s12 m8"><ol>
                            {
                                entry.senses().iter().filter_map(|sense| {
                                    let mut sense_string = String::new();
                                    let mut separator = "";
                                    let mut has_english = false;
                                    for gloss in sense.glosses() {
                                        sense_string.push_str(separator);
                                        sense_string.push_str(gloss.text());
                                        separator = "; ";
                                        has_english = has_english || gloss.lang().is_none();
                                    }
                                    if has_english {
                                        Some(sense_string)
                                    }
                                    else {
                                        None
                                    }
                                })
                                .map(|sense_string| html!(
                                    <li class="flow-text"> { text!(sense_string) } </li>
                                ))
                            }
                        </ol></div>
                        <div class="col s12 m2">
                            <div class="col s12 m2"><span class="badge">"tag1"</span><span class="badge">"tag2"</span></div>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col s4"><a class="waves-effect waves-light btn-small teal" target="_blank" href={ &jisho_url }>"Jisho"</a></div>
                        <div class="col s4"><a class="waves-effect waves-light btn-small teal" target="_blank" href={ &tangorin_url }>"Tangorin"</a></div>
                    </div>
                </li>
            }
        })
        .collect();

    {
        let word_count = document.get_element_by_id("word-count").unwrap();
        word_count.set_text_content(Some(&entry_list.len().to_string()));
    }

    collections.set_inner_html("");
    for entry_html in entries_html {
        let entry_string: String = entry_html.to_string();
        if let Some(new_element) = super::js_util::parse_html_element(&entry_string)? {
            collections.append_child(&new_element)?;
        }
    }

    Ok(())
}
