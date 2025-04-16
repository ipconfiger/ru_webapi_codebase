use crate::types::register_service;

pub mod handlers;
pub mod services;

pub async fn init() {
    register_service::<services::ExampleServices>();

}