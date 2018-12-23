use actix_web::{
    AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json, ResponseError, State,
};
use futures::future::Future;

use crate::common::AppState;
use crate::handler::user::RegisterUser;

pub fn login(_: &HttpRequest) -> &'static str {
    "Hello world!"
}
pub fn logout(_: &HttpRequest) -> &'static str {
    "Hello world!"
}

pub fn register(
    (register_user, state): (Json<RegisterUser>, State<AppState>),
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
