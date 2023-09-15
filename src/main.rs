#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket::Data;
use rocket::response::content;
use rocket::config::{Config, Environment};
use rocket_contrib::json::Json;
use std::fs;
use std::io::Read;

#[derive(Serialize)]
struct JsonResponse {
    data: String,
}

#[post("/insert", format = "application/json", data = "<input>")]
fn insert_handler(input: Data) -> Json<JsonResponse> {
    let mut body_str = String::new();
    input.open().read_to_string(&mut body_str).expect("Failed to read data");

    let digest = md5::compute(body_str.as_bytes());
    let hash = format!("{:x}", digest);
    Json(JsonResponse { data: hash })
}

#[get("/read")]
fn read_handler() -> content::Json<String> {
    let json = fs::read_to_string("formbox_stress_test.json").expect("Failed to read file");
    content::Json(json)
}

fn main() {
    let config = Config::build(Environment::Staging)
        .address("localhost")
        .port(6969)
        .finalize().unwrap();

    rocket::custom(config)
        .mount("/", routes![insert_handler, read_handler])
        .launch();
}
