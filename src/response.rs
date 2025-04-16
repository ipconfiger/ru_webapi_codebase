use axum::Json;
use axum::response::IntoResponse;
use sqlx::Error;
use serde_json::json;
use redis::RedisError;
use reqwest::Error as HttpError;
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::{Error as JsonError, Value};
use utoipa::ToSchema;


#[derive(Debug, Clone, ToSchema, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires: i32,
    
}

