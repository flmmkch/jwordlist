use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum JMDictEntryId<'a> {
    Kanji(Cow<'a, str>),
}

impl<'a> JMDictEntryId<'a> {
    pub fn from_kanji<S: Into<Cow<'a, str>>>(s: S) -> Self {
        JMDictEntryId::Kanji(s.into())
    }
    pub fn match_kanji(&self, kanji_string: &str) -> bool {
        match self {
            JMDictEntryId::Kanji(ref my_kanji_string) => kanji_string == my_kanji_string,
            _ => false,
        }
    }
    pub fn into_owned(self) -> JMDictEntryId<'static> {
        match self {
            JMDictEntryId::Kanji(my_kanji_string) => {
                JMDictEntryId::Kanji(Cow::Owned(my_kanji_string.into_owned()))
            }
        }
    }
}
