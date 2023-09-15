use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde_derive::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize)]
struct JsonResponse {
    data: String,
}

async fn insert_handler(body: web::Bytes) -> Result<HttpResponse> {
    let body_str = String::from_utf8_lossy(&body);
    let digest = md5::compute(body_str.as_bytes());
    let hash = format!("{:x}", digest);
    Ok(HttpResponse::Ok().json(JsonResponse { data: hash }))
}

async fn read_handler() -> Result<HttpResponse> {
    let content = fs::read_to_string("formbox_stress_test.json");
    match content {
        Ok(json) => Ok(HttpResponse::Ok().body(json)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error reading file")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/insert", web::post().to(insert_handler))
            .route("/read", web::get().to(read_handler))
    })
        .bind("localhost:6969")?
        .run()
        .await
}
