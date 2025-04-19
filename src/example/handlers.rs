use axum::{Json, Router};
use axum::routing::get;
use serde_json::{json, Value};
use crate::app::AppContext;
use crate::services::example_service::ExampleServices;
use crate::types::{get_serve, APIResult};
use crate::errors::ErrResponse;
use crate::response::StatusResponse;

#[utoipa::path(
    get,
    path = "/example/",
    responses(
            (status = 200, description = "测试Handler返回Json", body = StatusResponse),
            (status = 500, description = "服务器错误", body = ErrResponse),
            (status = 401, description = "认证失败", body = ErrResponse),
            (status = 403, description = "没有权限", body = ErrResponse)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
async fn it_works() -> APIResult<StatusResponse> {
    let serv = get_serve::<ExampleServices>();
    let resp = serv.test().await?;
    Ok(Json(StatusResponse{status: resp}))
}

// 导出路由
pub fn router(state: AppContext) -> Router {
    Router::new()
        .route("/", get(it_works))
        .with_state(state)
}