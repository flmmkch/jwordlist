use wasm_bindgen::prelude::*;

struct HtmlCollectionIterator {
    html_collection: web_sys::HtmlCollection,
    current_index: u32,
}

impl std::iter::Iterator for HtmlCollectionIterator {
    type Item = web_sys::Element;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.html_collection.length() {
            let result_item = self.html_collection.item(self.current_index);
            self.current_index += 1;
            result_item
        } else {
            None
        }
    }
}

pub fn html_collection_iter(
    html_collection: web_sys::HtmlCollection,
) -> impl Iterator<Item = web_sys::Element> {
    HtmlCollectionIterator {
        html_collection,
        current_index: 0,
    }
}

struct NodeListIterator {
    node_list: web_sys::NodeList,
    current_index: u32,
}

impl std::iter::Iterator for NodeListIterator {
    type Item = web_sys::Node;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.node_list.length() {
            let result_item = self.node_list.item(self.current_index);
            self.current_index += 1;
            result_item
        } else {
            None
        }
    }
}

pub fn node_list_iter(node_list: web_sys::NodeList) -> impl Iterator<Item = web_sys::Node> {
    NodeListIterator {
        node_list,
        current_index: 0,
    }
}

pub fn map_js_err_to_unit(js_err: JsValue) -> () {
    web_sys::console::log_2(&"Error executing future: ".into(), &js_err);
}

pub fn parse_html_element(html_string: &str) -> Result<Option<web_sys::Element>, JsValue> {
    let range = web_sys::Range::new()?;
    let document_fragment = range.create_contextual_fragment(html_string)?;
    Ok(document_fragment.first_element_child())
}
