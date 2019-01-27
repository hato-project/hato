use actix_web::middleware::{Middleware, Response, Started};
use actix_web::{
    http::{header, HttpTryFrom},
    App, HttpRequest, HttpResponse, Result,
};

pub struct Auth;

impl<S> Middleware<S> for Auth {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        Ok(Started::Done)
    }

    fn response(&self, req: &HttpRequest<S>, mut resp: HttpResponse) -> Result<Response> {
        resp.headers_mut().insert(
            header::HeaderName::try_from("X-VERSION").unwrap(),
            header::HeaderValue::from_static(("0.1.0")),
        );
        Ok(Response::Done(resp))
    }
}
