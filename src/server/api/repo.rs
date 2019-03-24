use actix_web::{
    AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json, ResponseError, Result, State,
};
use futures::Future;

use crate::common::AppState;
use crate::handler::repo::CreateRepo;

pub fn list_repos(state: State<AppState>) -> FutureResponse<HttpResponse> {
    unimplemented!()
}

pub fn create_repo(rp: Json<CreateRepo>, state: State<AppState>) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(rp.0)
        .from_err()
        .and_then(|res| match res {
            Ok(new_rp) => Ok(HttpResponse::Ok().json(new_rp)),
            Err(api_error) => Ok(api_error.error_response()),
        })
        .responder()
}

pub fn update_repo(state: State<AppState>) -> FutureResponse<HttpResponse> {
    unimplemented!()
}

pub fn delete_repo(state: State<AppState>) -> FutureResponse<HttpResponse> {
    unimplemented!()
}
