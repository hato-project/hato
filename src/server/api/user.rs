use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Json, ResponseError, State};
use futures::future::Future;

use crate::common::AppState;
use crate::handler::user::{LoginUser, RegisterUser};

pub fn login(login_user: Json<LoginUser>, state: State<AppState>) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(login_user.0)
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(api_error) => Ok(api_error.error_response()),
        })
        .responder()
}

pub fn register(
    register_user: Json<RegisterUser>,
    state: State<AppState>,
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(register_user.0)
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(api_error) => Ok(api_error.error_response()),
        })
        .responder()
}
