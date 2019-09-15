use jmdict::prelude::*;
use std::fs::File;
use std::path::Path;
mod config;
use std::sync::Arc;
use self::config::Config;

const CONFIG_FILENAME: &'static str = "jwordlist.yaml";

fn main() -> std::io::Result<()> {
    let app = Arc::new(JWordListApp::initialize());
    let app_data = actix_web::web::Data::new(Arc::clone(&app));
    println!("Listening on http://{}", &app.config.listen_bind);
    actix_web::HttpServer::new(
        move || actix_web::App::new()
            .register_data(app_data.clone())
            .service(actix_web::web::resource("/api/get_all_kanji").to(get_all_kanji))
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
        )
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
}

fn get_all_kanji(state: actix_web::web::Data<Arc<JWordListApp>>, _req: actix_web::HttpRequest) -> actix_web::Result<actix_web::HttpResponse> {
    let jwordlistapp: &JWordListApp = &state;
    let mut entry_list = Vec::new();
    jmdict::with_jmdict_gz_entries(
        std::io::Cursor::new(&jwordlistapp.jmdict_memory),
        JMDictEntryId::from_kanjis(vec!["言葉", "辞典"]),
        |e| {
            entry_list.push(e);
        },
    );
    let json_string = serde_json::to_string(&entry_list)?;
    Ok(actix_web::HttpResponse::Ok()
        .header(actix_web::http::header::CONTENT_TYPE, "application/json")
        .body(json_string))
}