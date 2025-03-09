use std::env;
use dotenv::dotenv;

use crate::service::core::{Service, URL};
use crate::traits::traits::SportPerformance;

mod models;
mod traits;
mod service;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let url = URL(env::var("SERVICE_URL").expect("SERVICE_URL not found in .env file"));
    let _ = Service::new(url)
        .await
        .start()
        .await;
}
