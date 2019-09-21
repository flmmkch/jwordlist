use std::fs::File;
use std::ffi::OsString;
use super::config::Config;
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
        let mut extension = config.jmdict_filename.extension().map(OsString::from).unwrap_or(OsString::new());
        extension.push(".bak");
        let mut backup_filename = config.jmdict_filename.clone();
        backup_filename.set_extension(&extension);
        std::fs::rename(&config.jmdict_filename, &backup_filename)?;
        eprintln!("{} renamed to {}", config.jmdict_filename.display(), backup_filename.display());
    }
    eprintln!("Downloading dictionary data from {} as {}", update_url, config.jmdict_filename.display());
    let file = File::create(&config.jmdict_filename)?;
    let request = reqwest::get(update_url)?;
    let mut downloader = TerminalDownloader::get(file, request.content_length().unwrap_or(1));
    reqwest::get(update_url)?
        .copy_to(&mut downloader)?;
    Ok(())
}

struct TerminalDownloader<T: Write> {
    write: T,
    progress_bar: indicatif::ProgressBar,
}

impl<T: Write> TerminalDownloader<T> {
    fn get(write: T, len: u64) -> Self {
        let progress_bar = indicatif::ProgressBar::new(len);
        TerminalDownloader {
            write,
            progress_bar,
        }
    }
}

impl<T: Write> Write for TerminalDownloader<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>
    {
        let res = self.write.write(buf);
        self.progress_bar.inc(buf.len() as u64);
        res
    }
    fn flush(&mut self) -> std::io::Result<()>
    {
        self.write.flush()
    }
}

impl<T: Write> Drop for TerminalDownloader<T> {
    fn drop(&mut self) {
        self.progress_bar.finish();
    }
}
