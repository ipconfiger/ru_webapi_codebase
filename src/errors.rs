use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlx::Error;
use reqwest::Error as HttpError;
use redis::RedisError;
use log::error;
use std::error::Error as StdError;
use serde::{Deserialize, Serialize};
use serde_json::{Error as JsonError, Value};
use utoipa::ToSchema;
use serde_json::json;
use uuid::Error as UuidError;

#[derive(Serialize, Deserialize, ToSchema)]
pub(crate) struct ErrResponse {
    info: String
}

#[derive(Debug)]
pub enum AppError {
    DbError(Error),
    RedisError(RedisError),
    LogicError(String),
    AuthError,
    HttpError(HttpError),
    JsonError(JsonError),
    UuidError(UuidError),
    NotFound,
    LoginFailure,
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        error!("数据库错误:{:?}", value);
        Self::DbError(value)
    }
}

impl From<String> for AppError {
    fn from(value: String) -> Self {
        Self::LogicError(value)
    }
}

impl From<RedisError> for AppError {
    fn from(value: RedisError) -> Self {
        Self::RedisError(value)
    }
}

impl From<HttpError> for AppError {
    fn from(value: HttpError) -> Self {
        Self::HttpError(value)
    }
}

impl From<JsonError> for AppError {
    fn from(value: JsonError) -> Self {
        Self::JsonError(value)
    }
}

impl From<UuidError> for AppError {
    fn from(value: UuidError) -> Self {
        error!("Uuid错误:{:?}", value);
        Self::UuidError(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let resp = match self {
            Self::DbError(de)=>{
                if let Error::RowNotFound = de {
                    error!("查询记录不存在：{:?}", de);
                    (
                        StatusCode::NOT_FOUND,
                        Json(json!(ErrResponse{info: format!("{de:?}")}))
                    )
                }else{
                    error!("数据库错误:{:?}", de);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!(ErrResponse{info: format!("{de:?}")}))
                    )
                   
                }
            },
            Self::LogicError(msg)=>{
                error!("业务错误:{}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(ErrResponse{info: msg}))
                )
            },
            Self::AuthError=>{
                (
                    StatusCode::FORBIDDEN,
                    Json(json!(ErrResponse{info: "没有认证权限".to_string()}))
                )
            },
            Self::LoginFailure=>{
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!(ErrResponse{info: "用户名密码错误".to_string()}))
                )
            },
            Self::RedisError(re)=>{
                error!("Redis错误:{:?}", re);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(ErrResponse{info: format!("{re:?}")}))
                )
            },
            Self::HttpError(he)=>{
                error!("HTTP请求错误:{:?}", he);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(ErrResponse{info: format!("{he:?}")}))
                )
            },
            Self::JsonError(je)=>{
                error!("Json反序列化错误:{:?}", je);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(ErrResponse{info: format!("{je:?}")}))
                )
            },
            Self::NotFound=>{
                error!("Not found error");
                (
                    StatusCode::NOT_FOUND,
                    Json(json!(ErrResponse{info: "请求的资源不存在".to_string()}))
                )
            },
            Self::UuidError(ue)=>{
                error!("Uuid错误:{:?}", ue);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(ErrResponse{info: format!("{ue:?}")}))
                )
            }
        };
        resp.into_response()
    }
}

pub type APIResult<T> = Result<T, AppError>;

