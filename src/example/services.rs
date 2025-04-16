use log::info;
use sqlx_struct_enhanced::EnhancedCrud;
use crate::errors::APIResult;
use crate::models::UserLogin;
use crate::redis::RedisHolder;
use crate::types::Service;
use crate::utilities::current_ts;

pub struct ExampleServices {
    pub(crate) db: sqlx::Pool<sqlx::Postgres>,
    pub(crate) redis: RedisHolder,
}

impl Service for ExampleServices {
    fn init(db: sqlx::Pool<sqlx::Postgres>, redis: RedisHolder) -> Self {
        Self{db, redis}
    }
}

impl ExampleServices {
    pub(crate) async fn test(&self) -> APIResult<()> {
        UserLogin{
            username: "Alex".to_string(),
            password: "123".to_string(),
            ts: current_ts(),
        }.insert_bind().execute(&self.db).await?;
        info!("Running test");
        Ok(())
    }

}