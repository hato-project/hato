use actix_web::{
    http::{header, Method},
    middleware::{self, cors::Cors},
    App,
};

use api::{index, ping};
use common::AppContext;
use db::init;

pub fn app_context() -> App<AppContext> {
    let addr = init();
    App::with_state(AppContext { db: addr.clone() })
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
                .register()
        })
}

pub fn app() -> App {
    App::new()
        .middleware(middleware::Logger::default())
        .resource("/", |r| r.f(index))
        .resource("/ping", |r| r.f(ping))
}
