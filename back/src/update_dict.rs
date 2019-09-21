use super::config::Config;
use futures::future::Future;
use futures::stream::Stream;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;

#[allow(dead_code)]
pub const DICT_FILENAME_ALL: &'static str = "JMdict.gz";
#[allow(dead_code)]
pub const DICT_FILENAME_EN: &'static str = "JMdict_e.gz";
#[allow(dead_code)]
pub const DICT_URL_ALL: &'static str = "http://ftp.monash.edu/pub/nihongo/JMdict.gz";
#[allow(dead_code)]
pub const DICT_URL_ENGLISH: &'static str = "http://ftp.monash.edu/pub/nihongo/JMdict_e.gz";

pub fn update_dict(update_url: &str, config: &Config) -> Result<(), crate::Error> {
    if config.jmdict_filename.exists() {
        if !config.jmdict_filename.is_file() {
            panic!("{} is not a file", config.jmdict_filename.display());
        }
        let mut extension = config
            .jmdict_filename
            .extension()
            .map(OsString::from)
            .unwrap_or(OsString::new());
        extension.push(".bak");
        let mut backup_filename = config.jmdict_filename.clone();
        backup_filename.set_extension(&extension);
        std::fs::rename(&config.jmdict_filename, &backup_filename)?;
        eprintln!(
            "{} renamed to {}",
            config.jmdict_filename.display(),
            backup_filename.display()
        );
    }
    eprintln!(
        "Downloading dictionary data from {} as {}",
        update_url,
        config.jmdict_filename.display()
    );
    let mut file = File::create(&config.jmdict_filename)?;
    let progress_bar = indicatif::ProgressBar::new(1);
    progress_bar.set_style(indicatif::ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));
    actix_rt::System::new("dict-update").block_on(futures::lazy(|| {
        actix_web::client::Client::new()
            .get(update_url)
            .send()
            .map_err(crate::Error::from)
            .and_then(|response| {
                let content_length: u64 = response
                    .headers()
                    .get(actix_web::http::header::CONTENT_LENGTH)
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1);
                progress_bar.set_length(content_length);
                response
                    .for_each(|bytes| {
                        file.write(bytes.as_ref())?;
                        progress_bar.inc(bytes.len() as u64);
                        Ok(())
                    })
                    .map_err(crate::Error::from)
            })
    }))?;
    progress_bar.finish();
    Ok(())
}
