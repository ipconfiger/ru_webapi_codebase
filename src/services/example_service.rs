use log::info;
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::errors::{APIResult, AppError};
use crate::redis::RedisHolder;
use crate::types::Service;
use crate::utilities::current_ts;
use crate::models::UserLogin::{ActiveModel, Entity};

pub struct ExampleServices {
    pub(crate) db: DatabaseConnection,
    pub(crate) redis: RedisHolder,
}

impl Service for ExampleServices {
    fn init(db: DatabaseConnection, redis: RedisHolder) -> Self {
        Self{db, redis}
    }
}

impl ExampleServices {
    pub(crate) async fn test(&self) -> APIResult<String> {
        if let Some(entity) = Entity::find_by_id::<String>("Alex".to_string()).one(&self.db).await? {
            Ok(entity.username.to_string())
        } else {
            Err(AppError::NotFound)
        }
    }
}