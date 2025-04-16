use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};


#[derive(Debug, Clone, IntoParams, ToSchema, Deserialize, Serialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
    
}

