use std::path::PathBuf;

use actix_files::{self as fs};
use actix_web::{http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use portpicker::{is_free_tcp, pick_unused_port};

use crate::playground::{PLAYGROUND_CSS, PLAYGROUND_HTML, PLAYGROUND_JS};

async fn pg_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(PLAYGROUND_HTML)
}

async fn pg_js() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(PLAYGROUND_JS)
}

async fn pg_css() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/css")
        .body(PLAYGROUND_CSS)
}

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

    let rel_dir = dir
        .strip_prefix(std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")))
        .unwrap_or(&dir);

    println!("\nServing the Pagefind Playground at http://localhost:{port}/pagefind/playground/");
    println!("Serving {rel_dir:?} at http://localhost:{port}");

    match HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/pagefind/playground")
                    .route("/", web::get().to(pg_index))
                    .route("/index.html", web::get().to(pg_index))
                    .route("/pagefind-playground.js", web::get().to(pg_js))
                    .route("/pagefind-playground.css", web::get().to(pg_css)),
            )
            .service(fs::Files::new("/", &dir).index_file("index.html"))
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
