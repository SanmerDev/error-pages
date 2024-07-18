use std::str::FromStr;

use actix_web::http::StatusCode;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use maud::{html, Markup, DOCTYPE};

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    HttpResponse::NoContent()
}

fn style() -> Markup {
    html! {
        style {
            "body { font-family: system-ui; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; padding: 0 20px; background-color: rgb(251, 252, 253); color: rgb(10, 12, 16); text-align: center; word-break: break-all; } \
             h1 { font-size: 5em; } \
             @media (prefers-color-scheme: dark) { body { background-color: rgb(10, 12, 16); color: rgb(251, 252, 253); } }"
        }
    }
}

#[get("/{content}")]
async fn index(content: web::Path<String>) -> impl Responder {
    let content_str = content.into_inner();
    let status_code = StatusCode::from_str(&content_str).unwrap_or(StatusCode::OK);

    let content = html! {
        (DOCTYPE)
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1.0";
        (style())
        title { (content_str) }
        h1 { (content_str) }
    };

    HttpResponse::with_body(status_code, content.into_string())
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
            .wrap(middleware::Logger::new("%{r}a %r %s %T %{User-Agent}i"))
            .service(favicon)
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
