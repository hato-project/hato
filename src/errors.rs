use actix_web::{error, http, HttpResponse};

#[derive(Fail, Debug)]
enum APIError {
    #[fail(display = "internal error")]
    InternalError,
    #[fail(display = "bad request")]
    BadRequest,
}

impl error::ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            APIError::InternalError => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
            APIError::BadRequest => HttpResponse::new(http::StatusCode::BAD_REQUEST),
        }
    }
}
