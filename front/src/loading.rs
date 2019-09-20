use std::marker::PhantomData;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/loading.js")]
extern "C" {
    #[wasm_bindgen]
    fn loadingStart();
    #[wasm_bindgen]
    fn loadingEnd();
}

pub struct JWordListLoading(PhantomData<()>);

impl JWordListLoading {
    pub fn lock() -> Self {
        loadingStart();
        Self(PhantomData)
    }
}

impl Drop for JWordListLoading {
    fn drop(&mut self) {
        loadingEnd();
    }
}
