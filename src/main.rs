use std::str::FromStr;

use actix_web::http::StatusCode;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};

const TEMPLATE: &str = include_str!("../html/template.html");

#[get("/{content}")]
async fn index(content: web::Path<String>) -> impl Responder {
    let content_str = content.into_inner();
    let status_code = StatusCode::from_str(&content_str).unwrap_or(StatusCode::OK);

    HttpResponse::build(status_code).body(TEMPLATE.replace("{{ content }}", &content_str))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    tracing::info!("starting HTTP server at http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
