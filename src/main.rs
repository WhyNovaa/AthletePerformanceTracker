use crate::service::core::Service;
use dotenv::dotenv;

mod models;
mod service;
mod traits;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let _ = Service::new().await.start().await;

    Ok(())
}
