use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Json, ResponseError, State};
use futures::future::Future;

use crate::common::AppState;

pub fn list_repos(state: State<AppState>) -> FutureResponse<HttpResponse> {
    unimplemented!()
}

pub fn create_repo(state: State<AppState>) -> FutureResponse<HttpResponse> {
    unimplemented!()
}

pub fn update_repo(state: State<AppState>) -> FutureResponse<HttpResponse> {
    unimplemented!()
}

pub fn delete_repo(state: State<AppState>) -> FutureResponse<HttpResponse> {
    unimplemented!()
}
