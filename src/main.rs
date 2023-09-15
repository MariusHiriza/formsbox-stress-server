use warp::{Filter, Rejection, Reply};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use warp::hyper::body::Bytes;
use warp::reply::Response;
use std::sync::Arc;

#[derive(Debug)]
enum CustomError {
    FileReadError,
}

impl warp::reject::Reject for CustomError {}

#[derive(Deserialize, Serialize)]
struct JsonResponse {
    data: String,
}

async fn insert_handler(body: Bytes) -> Result<impl Reply, Rejection> {
    let body_str = String::from_utf8_lossy(&body);
    let digest = md5::compute(body_str.as_bytes());
    let hash = format!("{:x}", digest);
    Ok(warp::reply::json(&JsonResponse { data: hash }))
}

async fn read_handler(json_content: Arc<String>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&json_content.as_str()))
}

#[tokio::main]
async fn main() {
    let json_content = Arc::new(fs::read_to_string("formbox_stress_test.json").expect("Failed to read the JSON file"));

    let insert_route = warp::post()
        .and(warp::path("insert"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::bytes())
        .and_then(insert_handler);

    let read_route = warp::get()
        .and(warp::path("read"))
        .and(warp::any().map(move || json_content.clone()))
        .and_then(read_handler);

    let routes = insert_route.or(read_route);

    warp::serve(routes).run(([127, 0, 0, 1], 6969)).await;
}
