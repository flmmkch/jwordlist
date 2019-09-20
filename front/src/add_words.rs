use futures::future::Future;
use jmdict::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn add_word_form_init() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    // set add words action
    {
        let add_words_submit_collection = document.get_elements_by_name("add-words-action");
        let submit_closure =
            Closure::wrap(Box::new(|| action_submit().unwrap()) as Box<dyn FnMut() -> bool>);
        for add_words_submit in super::js_util::node_list_iter(add_words_submit_collection) {
            configure_submit(add_words_submit, &submit_closure)?;
        }
        submit_closure.forget();
    }
    // add the fields to the form
    add_word_fields_reset()?;
    Ok(())
}

pub fn add_word_fields_reset() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let add_words_fields = document.get_element_by_id("add-words-fields").unwrap();
    add_words_fields.set_inner_html("");
    add_new_field(&add_words_fields)?;
    Ok(())
}

const FIELD_STRING_START: &'static str = "wordid";

fn add_field_name(index: u32) -> String {
    format!("{}{}", FIELD_STRING_START, index + 1)
}

fn add_new_field(add_words_field: &web_sys::Element) -> Result<web_sys::Element, JsValue> {
    use typed_html::html;
    let new_field_name =
        typed_html::types::Id::new(add_field_name(add_words_field.children().length()));
    let my_html: std::boxed::Box<typed_html::elements::p<String>> = html!(
                <p><input type="text" name=new_field_name style="width: 250px;" placeholder="Type a new word here..."/></p>
    );
    let new_element = super::js_util::parse_html_element(&my_html.to_string())?.unwrap();
    add_words_field.append_child(&new_element)?;
    Ok(new_element)
}

fn iter_add_word_fields() -> Result<impl Iterator<Item = web_sys::HtmlInputElement>, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let add_words_fields = document.get_element_by_id("add-words-fields").unwrap();
    let fields_collection = add_words_fields.get_elements_by_tag_name("input");
    let iterator = super::js_util::html_collection_iter(fields_collection)
        .filter_map(|html_element| html_element.dyn_into::<web_sys::HtmlInputElement>().ok())
        .filter(|e| e.name().starts_with(FIELD_STRING_START));
    Ok(iterator)
}

pub fn focus_next_add_word_field() -> Result<(), JsValue> {
    let field_iter = iter_add_word_fields()?;
    for field in field_iter {
        if field.value() == "" {
            field.focus()?;
            break;
        }
    }
    Ok(())
}

pub fn setup_add_words() -> Result<(), JsValue> {
    Ok(())
}

fn configure_submit<T: ?Sized>(node: web_sys::Node, closure: &Closure<T>) -> Result<(), JsValue> {
    if let Some(html_element) = node.dyn_ref::<web_sys::HtmlElement>() {
        html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
    }
    Ok(())
}

fn action_submit() -> Result<bool, JsValue> {
    use crate::storage::WordStorage;
    let mut all_dict_entry_ids: Vec<JMDictEntryId<'static>> =
        super::storage::WindowLocalStorage().get_stored_entry_ids()?;
    let fields: Vec<_> = iter_add_word_fields()?.collect();
    all_dict_entry_ids.reserve(fields.len());
    let field_value_iterator = fields.iter().map(|e| e.value());
    for field_value in field_value_iterator {
        all_dict_entry_ids.push(JMDictEntryId::from_kanji(field_value));
    }
    super::storage::WindowLocalStorage()
        .set_stored_entry_ids(all_dict_entry_ids.iter().cloned())?;
    let js_future = super::get_words(all_dict_entry_ids)?
        .and_then(super::display_word_list)
        .and_then(|_| {
            add_word_fields_reset()?;
            focus_next_add_word_field()?;
            Ok(())
        })
        .map_err(super::js_util::map_js_err_to_unit);
    wasm_bindgen_futures::spawn_local(js_future);
    // do not submit
    Ok(false)
}
