use dotenv::dotenv;
use std::env;
use sqlx::postgres;
use crate::service::core::{Service, URL};

mod models;
mod service;
mod traits;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let user = env::var("POSTGRES_USER").expect("POSTGRES_USER not found in .env file");
    let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not found in .env file");
    let db = env::var("POSTGRES_DB").expect("POSTGRES_DB not found in .env file");

    let database_url = format!("postgres://{}:{}@pg:5432/{}", user, password, db);

    let connection = postgres::PgPool::connect(database_url.as_str()).await.expect("Connection error");

    let url = URL(env::var("SERVICE_URL").expect("SERVICE_URL not found in .env file"));
    let _ = Service::new(url).await.start().await;
}
