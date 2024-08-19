use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

lazy_static::lazy_static! {
    static ref STORAGE_BASE_PATH: String = env::var("STORAGE_DIR").unwrap_or_else(|_| "data".to_string());
}

async fn handle_file_upload(mut payload: web::Payload) -> impl Responder {
    let file_path = format!("{}/{}", *STORAGE_BASE_PATH, "your_filename_here");

    let mut destination_file = File::create(&file_path)
        .map_err(|e| HttpResponse::InternalServerError().body(format!("Failed to create file: {}", e)))?;

    while let Some(chunk) = payload.next().await {
        let data = chunk.map_err(|e| HttpResponse::InternalServerError().body(format!("Failed reading chunk: {}", e)))?;
        if let Err(e) = destination_file.write_all(&data) {
            return HttpResponse::InternalServerError().body(format!("Failed to write data: {}", e));
        }
    }

    HttpResponse::Ok().body("File uploaded successfully")
}

async fn serve_file_download() -> impl Responder {
    let file_path = format!("{}/{}", *STORAGE_BASE_PATH, "file_name_to_download");
    let mut file_to_serve = File::open(&file_path).map_err(|_| HttpResponse::NotFound().body("File not found"))?;
    let mut file_contents = Vec::new();
    file_to_serve.read_to_end(&mut file_contents).map_err(|_| HttpResponse::InternalServerError().body("Failed to read the file"))?;

    HttpResponse::Ok().content_type("application/octet-stream").body(file_contents)
}

async fn handle_file_deletion() -> impl Responder {
    let file_path = format!("{}/{}", *STORAGE_BASE_PATH, "file_name_to_delete");
    fs::remove_file(&file_path).map_err(|_| HttpResponse::InternalServerError().body("Failed to delete the file"))?;

    HttpResponse::Ok().body("File deleted successfully")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(handle_file_upload))
            .route("/download", web::get().to(serve_file_download))
            .route("/delete", web::delete().to(handle_file_deletion))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}