use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;



#[derive(Debug, Clone, ToSchema, Deserialize, Serialize)]
pub struct StatusResponse {
    pub status: String,
    
}

#[derive(Debug, Clone, ToSchema, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires: i32,
    
}

