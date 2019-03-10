use crate::errors::APIErrorKind;
use crate::utils::{decode_user_token, Extension};
use actix_web::middleware::{Middleware, Response, Started};
use actix_web::{
    http::{header, HttpTryFrom},
    App, HttpRequest, HttpResponse, Result,
};

pub struct Auth;

impl<S> Middleware<S> for Auth {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        if req.method() == "OPTIONS" {
            return Ok(Started::Done);
        }
        let token = req
            .headers()
            .get("authorization")
            .map(|t| t.to_str().ok())
            .ok_or(APIErrorKind::Unauthorized)?;

        match token {
            Some(t) => {
                let claims = decode_user_token(t)?;
                req.extensions_mut().insert(Extension(claims));
                return Ok(Started::Done);
            }
            None => Err(APIErrorKind::Unauthorized.into()),
        }
    }

    fn response(&self, req: &HttpRequest<S>, mut resp: HttpResponse) -> Result<Response> {
        resp.headers_mut().insert(
            header::HeaderName::try_from("X-VERSION").unwrap(),
            header::HeaderValue::from_static("0.1.0"),
        );
        Ok(Response::Done(resp))
    }
}
