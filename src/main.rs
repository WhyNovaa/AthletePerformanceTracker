use dotenv::dotenv;
use std::env;

use crate::service::core::{Service, URL};

mod models;
mod service;
mod traits;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let url = URL(env::var("SERVICE_URL").expect("SERVICE_URL not found in .env file"));
    let _ = Service::new(url).await.start().await;
}
