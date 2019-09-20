use jmdict::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub trait WordStorage {
    type ErrorType;
    fn get_stored_entry_ids(&self) -> Result<Vec<JMDictEntryId<'static>>, Self::ErrorType>;
    fn set_stored_entry_ids<'a, I: IntoIterator<Item = JMDictEntryId<'a>>>(
        &self,
        entry_id_iter: I,
    ) -> Result<(), Self::ErrorType>;
}

pub struct WindowLocalStorage();

#[derive(Serialize, Deserialize)]
struct LocalWordStorageJson<'a> {
    entry_ids: Vec<JMDictEntryId<'a>>,
}

const WINDOW_LOCAL_STORAGE_KEY: &'static str = "WORD_ENTRY_IDS";

impl WordStorage for WindowLocalStorage {
    type ErrorType = JsValue;
    fn get_stored_entry_ids(&self) -> Result<Vec<JMDictEntryId<'static>>, Self::ErrorType> {
        let window = web_sys::window().unwrap();
        if let Some(local_storage) = window.local_storage()? {
            if let Some(json_string) = local_storage.get_item(WINDOW_LOCAL_STORAGE_KEY)? {
                let words_stored: LocalWordStorageJson =
                    serde_json::from_str(&json_string).unwrap();
                let owned_words = words_stored
                    .entry_ids
                    .into_iter()
                    .map(JMDictEntryId::into_owned)
                    .collect();
                Ok(owned_words)
            } else {
                Ok(Vec::new())
            }
        } else {
            Ok(Vec::new())
        }
    }
    fn set_stored_entry_ids<'a, I: IntoIterator<Item = JMDictEntryId<'a>>>(
        &self,
        entry_id_iter: I,
    ) -> Result<(), Self::ErrorType> {
        let window = web_sys::window().unwrap();
        let entry_ids: Vec<_> = entry_id_iter.into_iter().collect();
        let words_stored = LocalWordStorageJson { entry_ids };
        let local_storage: web_sys::Storage = window
            .local_storage()?
            .expect("No window local storage available");
        let json_string: String =
            serde_json::to_string(&words_stored).map_err(|e| e.to_string())?;
        local_storage.set_item(WINDOW_LOCAL_STORAGE_KEY, &json_string)?;
        Ok(())
    }
}
