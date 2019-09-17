#![recursion_limit="512"]
use wasm_bindgen::prelude::*;
use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen_futures::JsFuture;
use futures::future::Future;
use jmdict::prelude::*;
use wasm_bindgen::JsCast;
use std::panic;
mod loading;
use self::loading::JWordListLoading;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    initialize_app()?;
    Ok(())
}

pub fn initialize_app() -> Result<(), JsValue> {
    let js_future = get_words(vec!["言葉", "辞典", "映画館"])?
        .and_then(|entry_list| {
            use typed_html::{html, text};
            let window = web_sys::window().expect("no global window exists");
            let document = window.document().expect("no window document found");
            let word_list_container = document.get_element_by_id("word-list-container").expect("No \"word-list-container\" element found");
            word_list_container.class_list().remove_1("scale-out").unwrap();
            word_list_container.class_list().add_1("scale-in").unwrap();
            let collections = document.get_element_by_id("word-list").expect("No \"word-list\" element found");
            
            let entries_html: Vec<std::boxed::Box<typed_html::elements::li<String>>> =
                entry_list.iter().map(|entry| {
                    let main_kanji: &str = entry.kanji().first().map(jmdict::entry::Kanji::string).unwrap_or("");
                    let jisho_url = url::Url::parse("https://jisho.org/word/").and_then(|u| u.join(&main_kanji)).map(|u| String::from(u.as_str())).unwrap_or("#!".into());
                    let tangorin_url = url::Url::parse("https://tangorin.com/words/").and_then(|u| u.join(&main_kanji)).map(|u| String::from(u.as_str())).unwrap_or("#!".into());
                    html! {
                        <li class="collection-item">
                            <div class="row">
                                <div class="col s4 m2"><h5>{ text!( main_kanji ) }</h5><h6 class="grey-text">"READING"</h6></div>
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
                let word_count = document.get_element_by_id("word-count").expect("No \"word-count\" element found");
                word_count.set_text_content(Some(&entry_list.len().to_string()));
            }

            collections.set_inner_html("");
            let dom_parser = web_sys::DomParser::new().unwrap();
            for entry_html in entries_html {
                let entry_string: String = entry_html.to_string();
                if let Ok(new_document) = dom_parser.parse_from_string(&entry_string, web_sys::SupportedType::TextHtml) {
                    if let Some(new_element) = new_document.children().get_with_index(0) {
                        let _ = collections.append_child(&new_element);
                    }
                }
            };
            futures::future::ok(())
        }).map_err(|_| ());
    wasm_bindgen_futures::spawn_local(js_future);
    Ok(())
}

pub fn get_words<'a, S: Into<&'a str>, I: IntoIterator<Item = S>>(words_iterator: I) -> Result<impl Future<Item = Vec<JMDictEntry>, Error = JsValue>, JsValue>
{
    let mut _loading = JWordListLoading::lock();
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::SameOrigin);
    {
        let words: Vec<&str> = words_iterator.into_iter().map(Into::into).collect();
        let words_json: String = serde_json::to_string(&words).map_err(|e| e.to_string())?;
        opts.body(Some(&words_json.into()));
    }

    let request = Request::new_with_str_and_init(
        "/api/get_words",
        &opts,
    )?;

    request
        .headers()
        .set("Accept", "application/json")?;

    let window = web_sys::window().expect("no global `window` exists");
    let words_future = JsFuture::from(window.fetch_with_request(&request))
        .and_then(move |resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            _loading = JWordListLoading::lock();
            resp.json()
        })
        .and_then(|json_value: js_sys::Promise| {
            // Convert this other `Promise` into a rust `Future`.
            JsFuture::from(json_value)
        })
        .map(|json| -> Vec<JMDictEntry> {
            // Use serde to parse the JSON into a struct.
            json.into_serde().unwrap()
        });
    Ok(words_future)
}