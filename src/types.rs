use axum::Json;
use sea_orm::DatabaseConnection;
use crate::errors::AppError;
use crate::redis::RedisHolder;

pub type APIResult<T> = Result<Json<T>, AppError>;

pub fn get_serve<T: 'static>() -> T {
    ru_di::Di::get::<T>().unwrap()
}

pub trait Service {
    fn init(db: DatabaseConnection, redis: RedisHolder) -> Self;
}

pub fn register_service<T: Service + 'static + Send + Sync>() {
    ru_di::Di::register(move |di| {
        let db = di.get_inner::<DatabaseConnection>().unwrap();
        let redis = di.get_inner::<RedisHolder>().unwrap();
        T::init(db, redis)
    });
}