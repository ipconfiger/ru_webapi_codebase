use axum::{Json, Router};
use axum::routing::get;
use serde_json::{json, Value};
use crate::app::AppContext;
use crate::example::services;
use crate::types::{get_serve, APIResult};

#[utoipa::path(
    get,
    path = "/example/",
    responses(
            (status = 200, description = "获取文章列表有限条数", body = Response),
            (status = 500, description = "服务器错误", body = ErrResponse),
            (status = 401, description = "认证失败", body = ErrResponse),
            (status = 403, description = "没有权限", body = ErrResponse)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
async fn it_works() -> APIResult<Value> {
    let serv = get_serve::<services::ExampleServices>();
    serv.test().await?;
    Ok(Json(json!({"status": "it works".to_string()})))
}

// 导出路由
pub fn router(state: AppContext) -> Router {
    Router::new()
        .route("/", get(it_works))
        .with_state(state)
}