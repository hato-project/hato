use actix_web::{HttpResponse, ResponseError};
use failure::{Backtrace, Context, Fail};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct APIError {
    inner: Context<APIErrorKind>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum APIErrorKind {
    #[fail(display = "Internal Error")]
    InternalError,

    #[fail(display = "Bad Request")]
    BadRequest,

    #[fail(display = "Unauthorized")]
    Unauthorized,

    #[fail(display = "Forbidden")]
    Forbidden,

    #[fail(display = "Not Found")]
    NotFound,

    #[fail(display = "User not Found")]
    UserNotFound,

    #[fail(display = "Conflict")]
    Conflict,
}

impl Fail for APIError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&self.inner, f)
    }
}

impl APIError {
    pub fn kind(&self) -> APIErrorKind {
        *self.inner.get_context()
    }
}

impl From<APIErrorKind> for APIError {
    fn from(kind: APIErrorKind) -> APIError {
        APIError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<APIErrorKind>> for APIError {
    fn from(inner: Context<APIErrorKind>) -> APIError {
        APIError { inner: inner }
    }
}

impl ResponseError for APIErrorKind {
    fn error_response(&self) -> HttpResponse {
        match *self {
            e @ APIErrorKind::BadRequest => HttpResponse::BadRequest().json(e),
            e @ APIErrorKind::Unauthorized => HttpResponse::Unauthorized().json(e),
            e @ APIErrorKind::NotFound => HttpResponse::NotFound().json(e),
            e @ APIErrorKind::UserNotFound => HttpResponse::NotFound().json(e),
            e @ APIErrorKind::Forbidden => HttpResponse::Forbidden().json(e),
            e @ APIErrorKind::Conflict => HttpResponse::Conflict().json(e),
            e => HttpResponse::InternalServerError().json(e),
        }
    }
}
