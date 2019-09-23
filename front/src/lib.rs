#![recursion_limit = "512"]
use futures::future::Future;
use jmdict::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
mod loading;
use self::loading::JWordListLoading;
mod add_words;
mod display_word_list;
mod js_util;
mod storage;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    initialize_app()?;
    Ok(())
}

pub fn initialize_app() -> Result<(), JsValue> {
    use storage::WordStorage;
    // initial loading bar
    add_words::setup_add_words()?;
    let stored_words = storage::WindowLocalStorage().get_stored_entry_ids()?;
    add_words::add_word_form_init()?;
    let _ = JWordListLoading::lock();
    if stored_words.is_empty() {
        // ask for new words
        display_word_list::display_word_list(&[])?;
        add_words::focus_next_add_word_field()?;
    } else {
        // display initial words
        let js_future = get_words(stored_words)?
            .and_then(display_word_list)
            .map_err(js_util::map_js_err_to_unit);
        wasm_bindgen_futures::spawn_local(js_future);
    }
    Ok(())
}

pub fn get_words<'a, I: IntoIterator<Item = JMDictEntryId<'a>>>(
    words_iterator: I,
) -> Result<impl Future<Item = Vec<JMDictEntry>, Error = JsValue>, JsValue> {
    let mut _loading = JWordListLoading::lock();
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::SameOrigin);
    {
        let words: Vec<_> = words_iterator.into_iter().collect();
        let words_json: String = serde_json::to_string(&words).map_err(|e| e.to_string())?;
        opts.body(Some(&words_json.into()));
    }

    let request = Request::new_with_str_and_init("api/get_words", &opts)?;

    request.headers().set("Accept", "application/json")?;

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

pub fn display_word_list<S: AsRef<[JMDictEntry]>>(
    entry_list: S,
) -> impl Future<Item = (), Error = JsValue> {
    match display_word_list::display_word_list(entry_list.as_ref()) {
        Ok(()) => futures::future::ok(()),
        Err(e) => futures::future::err(e),
    }
}
