use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

lazy_static::lazy_static! {
    static ref STORAGE_DIR: String = std::env::var("STORAGE_DIR").unwrap_or_else(|_| "data".to_string());
}

async fn upload_file(mut payload: web::Payload) -> impl Responder {
    let filepath = format!("{}/{}", *STORAGE_DIR, "your_filename_here");
    let mut file = File::create(filepath).unwrap();

    while let Some(chunk) = payload.next().await {
        let data = chunk.unwrap();
        file.write_all(&data).unwrap();
    }

    HttpResponse::Ok().body("File uploaded successfully")
}

async fn download_file() -> impl Responder {
    let filepath = format!("{}/{}", *STORAGE_DIR, "your_filename_to_download");
    let mut file = File::open(filepath).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(contents)
}

async fn delete_file() -> impl Responder {
    let filepath = format!("{}/{}", *STORAGE_DIR, "your_filename_to_delete");
    fs::remove_file(filepath).unwrap();

    HttpResponse::Ok().body("File deleted successfully")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(upload_file))
            .route("/download", web::get().to(download_file))
            .route("/delete", web::delete().to(delete_file))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}