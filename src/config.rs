use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub jmdict_filename: PathBuf,
    pub listen_bind: String,
}
