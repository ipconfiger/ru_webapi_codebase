use crate::services::example_service::ExampleServices;
use crate::types::register_service;

pub mod example_service;

pub async fn init() {
    register_service::<ExampleServices>();
}