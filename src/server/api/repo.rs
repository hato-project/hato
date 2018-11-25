use actix_web::{AsyncResponder, FutureResponse, HttpRequest, HttpResponse};
use futures::future::Future;

use common::AppState;
use model::*;

pub fn repo(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let repo_id = req.match_info().get("repo_id").unwrap().to_string();
    req.state()
        .db
        .send(RepoID { repo_id })
        .from_err()
        .and_then(|res| match res {
            Ok(repo) => {
                if let Some(repo) = repo {
                    return Ok(HttpResponse::Ok().json(repo));
                }
                Ok(HttpResponse::NotFound().into())
            }
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
