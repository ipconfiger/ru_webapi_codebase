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
        let user = UserLogin::where_query("name=$1").bind("test").fetch_one(&self.db).await?;
        info!("Running test:{:?}", user);
        Ok(())
    }

}