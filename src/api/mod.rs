use actix_web::{
    AsyncResponder, FutureResponse, HttpMessage, HttpRequest, HttpResponse, Json, State,
};
use futures::future::Future;

use chrono::prelude::*;

use common::AppState;
use model::*;

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

pub fn repo(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let repo_id = req.match_info().get("repo_id").unwrap().to_string();
    req.state()
        .db
        .send(RepoID { repo_id })
        .from_err()
        .and_then(|res| match res {
            Ok(repo) => Ok(HttpResponse::Ok().json(repo)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
