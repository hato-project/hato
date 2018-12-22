pub fn ping(_: &HttpRequest) -> String {
    json!({
        "name": "hato",
        "time": Utc::now()
    })
    .to_string()
}
