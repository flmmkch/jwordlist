#![recursion_limit="512"]
use wasm_bindgen::prelude::*;
use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen_futures::JsFuture;
use futures::future::Future;
use jmdict::prelude::*;
use js_sys::Promise;
use wasm_bindgen::JsCast;
use std::panic;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    Ok(())
}

#[wasm_bindgen]
pub fn get_all_kanji() -> Result<Promise, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::SameOrigin);

    let request = Request::new_with_str_and_init(
        "/api/get_all_kanji",
        &opts,
    )?;

    request
        .headers()
        .set("Accept", "application/json")?;
    let window = web_sys::window().expect("no global `window` exists");
    let request_future = JsFuture::from(window.fetch_with_request(&request));
    let js_future = request_future
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            resp.json()
        })
        .and_then(|json_value: js_sys::Promise| {
            web_sys::console::log_1(&"GET KANJI 3".into());
            // Convert this other `Promise` into a rust `Future`.
            JsFuture::from(json_value)
        })
        .and_then(|json| {
            use typed_html::{html, text};
            // Use serde to parse the JSON into a struct.
            let entry_list: Vec<JMDictEntry> = json.into_serde().unwrap();
            let window = web_sys::window().expect("no global window exists");
            let document = window.document().expect("no window document found");
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
                                <div class="col s12 m8"><div class="flow-text">
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
                                            <p> { text!(sense_string) } </p>
                                        ))
                                    }
                                </div></div>
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
            }
            futures::future::ok(wasm_bindgen::JsValue::NULL)
        });
    Ok(wasm_bindgen_futures::future_to_promise(js_future))
}

