use actix_web::HttpRequest;
use chrono::prelude::*;

pub fn index(_: &HttpRequest) -> &'static str {
    "Hello Hato!"
}

pub fn ping(_: &HttpRequest) -> String {
    json!({
        "name": "hato",
        "time": Local::now(),
    })
    .to_string()
}
