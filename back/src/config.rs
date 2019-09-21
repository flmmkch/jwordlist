use serde::Deserialize;
use std::path::PathBuf;

pub const CONFIG_FILENAME: &'static str = "jwordlist.yaml";

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub jmdict_filename: PathBuf,
    pub listen_bind: String,
}
