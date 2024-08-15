use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::StatusCode};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::env;
use futures::StreamExt; // Required for payload.stream()

lazy_static::lazy_static! {
    static ref STORAGE_DIR: String = env::var("STORAGE_DIR").unwrap_or_else(|_| "data".to_string());
}

async fn upload_file(mut payload: web::Payload) -> impl Responder {
    let filepath = format!("{}/{}", *STORAGE_DIR, "your_filename_here");
    let mut file = match File::create(&filepath) {
        Ok(file) => file,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to create file: {}", e)),
    };

    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(data) => {
                if let Err(e) = file.write_all(&data) {
                    return HttpResponse::InternalServerError().body(format!("Failed to write data: {}", e));
                }
            },
            Err(e) => return HttpResponse::InternalServerError().body(format!("Failed reading chunk: {}", e)),
        }
    }

    HttpResponse::Ok().body("File uploaded successfully")
}

async fn download_file() -> impl Responder {
    let filepath = format!("{}/{}", *STORAGE_DIR, "your_filename_to_download");
    let mut file = match File::open(&filepath) {
        Ok(file) => file,
        Err(_) => return HttpResponse::NotFound().body("File not found"),
    };
    let mut contents = Vec::new();
    if let Err(_) = file.read_to_end(&mut contents) {
        return HttpResponse::InternalServerError().body("Failed to read the file");
    }

    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(contents)
}

async fn delete_file() -> impl Responder {
    let filepath = format!("{}/{}", *STORAGE_DIR, "your_filename_to_delete");
    match fs::remove_file(&filepath) {
        Ok(_) => HttpResponse::Ok().body("File deleted successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete the file"),
    }
}

#[actix_web::main]
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