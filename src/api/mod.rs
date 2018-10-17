pub mod repo;
pub mod webhook;

use actix_web::HttpRequest;
use chrono::prelude::*;

pub fn index(_: &HttpRequest) -> &'static str {
    "Hello Hato!"
}

pub fn ping(_: &HttpRequest) -> String {
    json!({
        "name": "hato",
        "time": Utc::now()
    })
    .to_string()
}
