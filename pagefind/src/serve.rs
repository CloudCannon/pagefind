use std::path::PathBuf;

use actix_files as fs;
use actix_web::{App, HttpServer};
use portpicker::{is_free_tcp, pick_unused_port};

pub async fn serve_dir(dir: PathBuf) {
    let port = if is_free_tcp(1414) {
        1414
    } else {
        match pick_unused_port() {
            Some(p) => p,
            None => {
                panic!("No free ports!")
            }
        }
    };

    println!("Serving {dir:?} at http://localhost:{port}");

    match HttpServer::new(move || {
        App::new().service(fs::Files::new("/", &dir).index_file("index.html"))
    })
    .bind(("127.0.0.1", port))
    {
        Ok(bound) => {
            let server = bound.run();
            if let Err(e) = server.await {
                panic!("Couldn't serve the directory: {e:?}");
            }
        }
        Err(e) => {
            panic!("Couldn't serve the directory: {e:?}");
        }
    }
}
