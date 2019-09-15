use std::borrow::Cow;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum JMDictEntryId<'a> {
    Kanji(Cow<'a, str>),
}

impl<'a> JMDictEntryId<'a> {
    pub fn from_kanji<S: Into<Cow<'a, str>>>(s: S) -> Self {
        JMDictEntryId::Kanji(s.into())
    }
    pub fn from_kanjis<S: Into<Cow<'a, str>>, I: IntoIterator<Item = S>>(
        into_iter: I,
    ) -> impl Iterator<Item = JMDictEntryId<'a>> {
        into_iter.into_iter().map(Self::from_kanji)
    }
    pub fn match_kanji(&self, kanji_string: &str) -> bool {
        match self {
            JMDictEntryId::Kanji(ref my_kanji_string) => kanji_string == my_kanji_string,
            _ => false,
        }
    }
}
