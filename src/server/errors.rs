use actix_web::{HttpResponse, ResponseError};

#[derive(Fail, Debug)]
pub enum APIError {
    #[fail(display = "internal error")]
    InternalError,

    #[fail(display = "bad request")]
    BadRequest,
}

impl ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            APIError::InternalError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            APIError::BadRequest => HttpResponse::BadRequest().json("Bad Request"),
        }
    }
}
