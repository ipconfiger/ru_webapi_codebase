use std::net::SocketAddr;
use std::time::Duration;
use axum::response::{Html, IntoResponse, Response};
use axum::{Json, Router};
use axum::body::to_bytes;
use axum::http::{header, Request, StatusCode};
use axum::middleware::Next;
use axum::routing::get;
//use http::{header, Request, StatusCode};
use log::{error, info};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use crate::errors::ErrResponse;
use crate::conf::Configuration;
use crate::{example, services};
use crate::redis::{RedisHolder, RedisSession};
use redis::AsyncCommands;

#[derive(Clone)]
pub struct AppContext {}

pub async fn init(cfg: &Configuration) {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&cfg.db_uri.clone())
        .await
        .expect("can't connect to database");
    let client = redis::Client::open(cfg.redis_uri.clone()).unwrap();
    let client_inner = client.clone();
    ru_di::Di::register(move |_| {
        RedisHolder{client: client_inner.clone()}
    });
    let pool_inner = pool.clone();
    ru_di::Di::register(move |_| {
        pool_inner.clone()
    });
    services::init().await;
}

pub async fn start_serve(cfg: &Configuration) {
    
    let state = AppContext{};

    let app = Router::new()
        .route("/api/js/rapidoc-min.js", get(redoc_js))
        .route("/api/docs", get(redoc_ui))
        .route("/api/docs/openapi.json", get(openapi))
        .route("/api/status", get(status))
        .route("/api/404", get(not_found))
        .nest("/example", example::handlers::router(state.clone()))
        .layer(axum::middleware::from_fn(log_request));
    info!("server will start at 0.0.0.0:{}", cfg.port);
    if let Ok(listener) = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", cfg.port).as_str()).await {
        if let Err(e) = axum::serve(listener, app).await {
            error!("server fault with err:{}", e);
        }else{
            info!("server stopped normally");
        }
    }
}

async fn not_found() -> Html<String> {
    Html(include_str!("../templates/404.html").to_string())
}
async fn status()->impl IntoResponse {
    Json(json!({"status": "it works!"}))
}

async fn log_request(req: Request<axum::body::Body>, next: Next) -> Response {
    info!("{} {}", req.method(), req.uri());
    next.run(req).await
}

async fn redoc_js() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/javascript")
        .body(axum::body::Body::new::<String>(include_str!("../templates/rapidoc-min.js").to_string())).unwrap()
}

async fn redoc_ui() -> Html<String> {
    let rapidoc = RapiDoc::new("/api/docs/openapi.json");
    let rapidoc = rapidoc.custom_html(CUSTOM_REDOC_HTML.to_string());
    Html(rapidoc.to_html())
}

const CUSTOM_REDOC_HTML: &str = r#"
<!doctype html> <!-- Important: must specify -->
<html>
  <head>
    <meta charset="utf-8"> <!-- Important: rapi-doc uses utf8 characters -->
    <script type="module" src="/api/js/rapidoc-min.js"></script>
  </head>
  <body>
    <rapi-doc spec-url = "/api/docs/openapi.json"> </rapi-doc>
  </body>
</html>
"#;

#[utoipa::path(
    get,
    path = "/api/docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[derive(OpenApi)]
#[openapi(
paths(
    openapi,
    example::handlers::it_works,
),
components(
    schemas(
        ErrResponse
    ),
),
)]
struct ApiDoc;