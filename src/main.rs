use actix_web::http::StatusCode;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::str::FromStr;

const TEMPLATE: &str = include_str!("../asset/template.html");
#[cfg(debug_assertions)]
const FAVICON_LIGHT: &str = "asset/ghost-light.ico";
#[cfg(not(debug_assertions))]
const FAVICON_LIGHT: &str = "/etc/error-pages/ghost-light.ico";
#[cfg(debug_assertions)]
const FAVICON_DARK: &str = "asset/ghost-dark.ico";
#[cfg(not(debug_assertions))]
const FAVICON_DARK: &str = "/etc/error-pages/ghost-dark.ico";

macro_rules! favicon {
    ($path:expr) => {
        match fs::read($path) {
            Ok(v) => HttpResponse::Ok().content_type("image/x-icon").body(v),
            Err(_) => HttpResponse::NotFound().finish(),
        }
    };
}

#[get("/ghost-light.ico")]
async fn favicon_light() -> impl Responder {
    favicon!(FAVICON_LIGHT)
}

#[get("/ghost-dark.ico")]
async fn favicon_dark() -> impl Responder {
    favicon!(FAVICON_DARK)
}

#[get("/{content}")]
async fn index(content: web::Path<String>) -> impl Responder {
    let content_str = content.into_inner();
    let status_code = StatusCode::from_str(&content_str).unwrap_or(StatusCode::OK);
    let content = TEMPLATE.replace("content-str", &content_str);
    HttpResponse::with_body(status_code, content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    HttpServer::new(|| {
        App::new()
            .service(favicon_light)
            .service(favicon_dark)
            .service(index)
            .wrap(middleware::Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
