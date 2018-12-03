use crate::api::*;
use crate::common::AppState;
use actix_web::{
    http::{header, Method},
    middleware::{self, cors::Cors},
    App,
};

pub fn app_hato(app_state: AppState) -> App<AppState> {
    App::with_state(app_state)
        .middleware(middleware::Logger::default())
        .prefix("/hato")
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
                .resource("/repo/{repo_id}", |r| {
                    r.method(Method::GET).with(repo::repo);
                })
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
