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
use std::backtrace::Backtrace;
use std::fmt;
use clap::ArgAction::Append;
use clap::parser::ValueSource;

#[derive(Serialize, Deserialize, ToSchema)]
pub(crate) struct ErrResponse {
    info: String
}

#[derive(Debug)]
pub enum AppError {
    DbError {
        source:Error,
        backtrace: Backtrace,
    },
    RedisError { 
        source: RedisError,
        backtrace: Backtrace,
    },
    LogicError(String),
    AuthError,
    HttpError{ 
        source:HttpError,
        backtrace: Backtrace,
    },
    JsonError { 
        source: JsonError,
        backtrace: Backtrace,
    },
    UuidError { 
        source: UuidError, 
        backtrace: Backtrace,},
    NotFound,
    LoginFailure,
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        error!("数据库错误:{:?}", value);
        Self::DbError{ source: value, backtrace: Backtrace::capture()}
    }
}

impl From<String> for AppError {
    fn from(value: String) -> Self {
        Self::LogicError(value)
    }
}

impl From<RedisError> for AppError {
    fn from(value: RedisError) -> Self {
        Self::RedisError{ source: value, backtrace: Backtrace::capture() }
    }
}

impl From<HttpError> for AppError {
    fn from(value: HttpError) -> Self {
        Self::HttpError{ source: value, backtrace: Backtrace::capture() }
    }
}

impl From<JsonError> for AppError {
    fn from(value: JsonError) -> Self {
        Self::JsonError{ source: value, backtrace: Backtrace::capture() }
    }
}

impl From<UuidError> for AppError {
    fn from(value: UuidError) -> Self {
        error!("Uuid错误:{:?}", value);
        Self::UuidError{ source: value, backtrace: Backtrace::capture() }
    }
}


impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DbError { source, backtrace } => {
                write!(f, "Database Error: {} \nBacktrace:\n{}", source, backtrace)
            },
            AppError::RedisError { source, backtrace } => {
                write!(f, "Redis Error: {} \nBacktrace:\n{}", source, backtrace)
            }
            AppError::LogicError(msg) => {
                write!(f, "Logic Error: {msg}")
            }
            AppError::AuthError => {
                write!(f, "Auth Error")
            }
            AppError::HttpError { source, backtrace } => {
                write!(f, "Http Error: {} \nBacktrace:\n{}", source, backtrace)
            }
            AppError::JsonError { source, backtrace } => {
                write!(f, "Json Parse Error: {} \nBacktrace:\n{}", source, backtrace)
            }
            AppError::UuidError { source, backtrace } => {
                write!(f, "Uuid Parse Error: {} \nBacktrace:\n{}", source, backtrace)
            }
            AppError::NotFound => {
                write!(f, "Not Found")
            }
            AppError::LoginFailure => {
                write!(f, "Login Failure")
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        eprintln!("Error: {}", self);
        let resp = match self {
            Self::DbError { source, backtrace}=>{
                if let Error::RowNotFound = source {
                    //error!("查询记录不存在：{:?} {:?}", source, backtrace);
                    (
                        StatusCode::NOT_FOUND,
                        Json(json!(ErrResponse{info: format!("{source:?}")}))
                    )
                }else{
                    //error!("数据库错误:{:?} {:?}", source, backtrace);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!(ErrResponse{info: format!("{source:?}")}))
                    )
                   
                }
            },
            Self::LogicError(msg)=>{
                //error!("业务错误:{}", msg);
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
            Self::RedisError { source, backtrace}=>{
                //error!("Redis错误:{:?}, {:?}", source, backtrace);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(ErrResponse{info: format!("{source:?}")}))
                )
            },
            Self::HttpError { source, backtrace}=>{
                //error!("HTTP请求错误:{:?} {:?}", source, backtrace);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(ErrResponse{info: format!("{source:?}")}))
                )
            },
            Self::JsonError { source, backtrace}=>{
                //error!("Json反序列化错误:{:?} {:?}", source, backtrace);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(ErrResponse{info: format!("{source:?}")}))
                )
            },
            Self::NotFound=>{
                //error!("Not found error");
                (
                    StatusCode::NOT_FOUND,
                    Json(json!(ErrResponse{info: "请求的资源不存在".to_string()}))
                )
            },
            Self::UuidError { source, backtrace}=>{
                //error!("Uuid错误:{:?} {:?}", source, backtrace);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(ErrResponse{info: format!("{source:?}")}))
                )
            }
        };
        resp.into_response()
    }
}

pub type APIResult<T> = Result<T, AppError>;

