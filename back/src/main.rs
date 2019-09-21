use clap::{Arg, App};
use std::fs::File;
mod config;
mod server;
mod update_dict;
mod error;
use error::Error;

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
