use crate::api::*;
use crate::common::AppState;
use crate::middleware::auth::Auth;
use actix_web::{
    http::{header, Method},
    middleware::{self, cors::Cors},
    App,
};

pub fn app_hato(app_state: AppState) -> App<AppState> {
    App::with_state(app_state)
        .middleware(middleware::Logger::default())
        .prefix("/api")
        .configure(|app| {
            Cors::for_app(app)
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![
                    header::ORIGIN,
                    header::AUTHORIZATION,
                    header::ACCEPT,
                    header::CONTENT_TYPE,
                ])
                .supports_credentials()
                .max_age(3600)
                .resource("/register", |r| r.method(Method::POST).with(user::register))
                .resource("/login", |r| r.method(Method::POST).with(user::login))
                .resource("/me", |r| {
                    r.middleware(Auth);
                    r.method(Method::GET).f(user::me)
                })
                .resource("/repo", |r| r.method(Method::POST).with(repo::create_repo))
                .register()
        })
}

pub fn app_common() -> App {
    App::new()
        .middleware(middleware::Logger::default())
        .resource("/", |r| r.f(index))
        .resource("/ping", |r| r.f(ping))
        .resource("/webhook", |r| {
            r.method(Method::POST).with(webhook::webhook)
        })
}
