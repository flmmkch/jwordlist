use clap::{Arg, App};
use std::fs::File;
mod config;
mod server;
mod update_dict;

fn main() -> Result<(), Error> {
    let matches = App::new("JWordList")
        .version("0.1.0")
        .author("Victor Nivet <victor@saumon.ninja>")
        .about("A Japanese vocabulary list web app")
        .arg(Arg::with_name("dict-update")
            .long("dict-update")
            .help("Update the dictionary file"))
        .arg(Arg::with_name("dict-url")
            .long("dict-url")
            .requires("dict-update")
            .takes_value(true)
            .help("Dictionary file update URL"))
        .arg(Arg::with_name("config")
            .long("configuration")
            .value_name("FILE")
            .default_value(config::CONFIG_FILENAME)
            .help("Sets a custom config file")
            .takes_value(true))
        .get_matches();
    let config_filename: std::path::PathBuf = matches.value_of_os("config").map(std::ffi::OsStr::to_os_string).unwrap_or_else(|| std::ffi::OsString::from(config::CONFIG_FILENAME)).into();
    let config: config::Config = serde_yaml::from_reader(File::open(&config_filename).expect(&format!(
            "Unable to open config file {} for reading",
            config_filename.display()
        )))
        .expect(&format!(
            "Error while deserializing config file {}",
            config_filename.display()
        ));
    if matches.is_present("dict-update") {
        let update_url: &str = match (matches.value_of("dict-url"), config.jmdict_filename.file_name().and_then(std::ffi::OsStr::to_str)) {
            (Some(dict_url), _) => dict_url,
            (None, Some(update_dict::DICT_FILENAME_ALL)) => update_dict::DICT_URL_ALL,
            (None, Some(update_dict::DICT_FILENAME_EN)) => update_dict::DICT_URL_ENGLISH,
            (None, other_filename) => panic!("Unable to determine dictionary URL for \"{}\": use a dictionary with \"JMDict.gz\" or \"JMDict_e.gz\" as filename, or add --dict-url argument", other_filename.unwrap_or_default()),
        };
        if let Err(e) = update_dict::update_dict(update_url, &config) {
            eprintln!("Failed to update dictionary: {}", e);
        }
    }
    // run the server
    server::run_server(config)?;
    Ok(())
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ActixPayloadError(actix_web::error::PayloadError),
    ActixClientSendRequestError(actix_web::client::SendRequestError),
    Other(Box<dyn std::error::Error + 'static>),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<actix_web::error::PayloadError> for Error {
    fn from(error: actix_web::error::PayloadError) -> Self {
        Error::ActixPayloadError(error)
    }
}

impl From<actix_web::client::SendRequestError> for Error {
    fn from(error: actix_web::client::SendRequestError) -> Self {
        Error::ActixClientSendRequestError(error)
    }
}

#[allow(dead_code)]
impl Error {
    pub fn from_other<E: std::error::Error + 'static>(error: E) -> Self {
        Self::Other(Box::new(error))
    }
    pub fn error(&self) -> Option<&dyn std::error::Error> {
        match self {
            Error::IoError(e) => Some(e),
            Error::ActixPayloadError(_) => None,
            Error::ActixClientSendRequestError(_) => None,
            Error::Other(e) => Some(e.as_ref()),
        }
    }
    pub fn display(&self) -> &dyn std::fmt::Display {
        match self {
            Error::IoError(e) => e,
            Error::ActixPayloadError(e) => e,
            Error::ActixClientSendRequestError(e) => e,
            Error::Other(ref e) => e,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Display::fmt(self.display(), f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.error().and_then(|e| e.source())
    }
}