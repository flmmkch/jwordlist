use actix_web::web;
use futures::future::Future;
use futures::stream::Stream;
use jmdict::prelude::*;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
mod config;
use self::config::Config;
mod error;
use self::error::*;

const CONFIG_FILENAME: &'static str = "jwordlist.yaml";

fn main() -> std::io::Result<()> {
    let app = Arc::new(JWordListApp::initialize());
    let app_data = web::Data::new(Arc::clone(&app));
    println!("Listening on http://{}", &app.config.listen_bind);
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .register_data(app_data.clone())
            .service(web::resource("/api/get_words").route(web::post().to_async(get_words)))
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(&app.config.listen_bind)?
    .run()
}

struct JWordListApp {
    config: Config,
    jmdict_memory: Vec<u8>,
}

impl JWordListApp {
    fn initialize() -> Self {
        let config: Config = serde_yaml::from_reader(File::open(CONFIG_FILENAME).expect(&format!(
            "Unable to open config file \"{}\" for reading",
            CONFIG_FILENAME
        )))
        .expect(&format!(
            "Error while deserializing config file \"{}\"",
            CONFIG_FILENAME
        ));
        let jmdict_filename = Path::new(&config.jmdict_filename);
        let jmdict_memory = Self::load_jmdict_to_memory(jmdict_filename).expect(&format!(
            "Unable to read JMDict file {}",
            jmdict_filename.display()
        ));
        JWordListApp {
            config,
            jmdict_memory,
        }
    }
    fn load_jmdict_to_memory(jmdict_path: &Path) -> Result<Vec<u8>, std::io::Error> {
        use std::io::Cursor;
        use std::io::{Seek, SeekFrom};
        let mut jmdict_file = File::open(jmdict_path)?;
        let total_size = jmdict_file.seek(SeekFrom::End(0))?;
        jmdict_file.seek(SeekFrom::Start(0))?;
        let mut jmdict_memory: Vec<u8> = Vec::with_capacity(total_size as usize);
        let mut jmdict_memory_cursor = Cursor::new(&mut jmdict_memory);
        std::io::copy(&mut jmdict_file, &mut jmdict_memory_cursor)?;
        Ok(jmdict_memory)
    }
    fn with_entries<'a, I: IntoIterator<Item = JMDictEntryId<'a>>, F: FnMut(JMDictEntry)>(
        &self,
        entries_iter: I,
        f: F,
    ) -> usize {
        jmdict::with_jmdict_gz_entries(std::io::Cursor::new(&self.jmdict_memory), entries_iter, f)
    }
}

fn get_words(
    state: web::Data<Arc<JWordListApp>>,
    payload: web::Payload,
) -> impl Future<Item = actix_web::HttpResponse, Error = actix_web::error::Error> {
    payload
        .from_err()
        .fold(web::BytesMut::new(), |mut body, chunk| {
            body.extend_from_slice(&chunk);
            Result::<web::BytesMut, actix_web::error::Error>::Ok(body)
        })
        .and_then(move |body| {
            use std::collections::HashMap;
            let entries_id_string_total: &str =
                std::str::from_utf8(&body).map_err(JWordListErrorResponse::from)?;
            let entry_ids: Vec<JMDictEntryId> = serde_json::from_str(&entries_id_string_total)
                .map_err(JWordListErrorResponse::from)?;
            let jwordlistapp: &JWordListApp = &state;
            let mut entries_map: HashMap<JMDictEntryId, JMDictEntry> = Default::default();
            jwordlistapp.with_entries(entry_ids.iter().cloned(), |e| {
                entries_map.insert(e.entry_id().clone().into_owned(), e.clone());
            });
            // reorder according to the original order
            let entry_list: Vec<&JMDictEntry> = entry_ids
                .iter()
                .filter_map(|id| entries_map.get(id))
                .collect();
            let json_string =
                serde_json::to_string(&entry_list).map_err(JWordListErrorResponse::from)?;
            Ok(actix_web::HttpResponse::Ok()
                .header(actix_web::http::header::CONTENT_TYPE, "application/json")
                .body(json_string))
        })
}
