use dotenv::dotenv;
use crate::models::sportsman::Sportsman;
use crate::service::core::Service;
use crate::service::postgres::postgres_pool::DBPool;

mod models;
mod service;
mod traits;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();
    env_logger::init();

    let pool = DBPool::new().await;
    pool.add_sportsman(&Sportsman::new(String::from("Aboba"))).await.unwrap();
    let a = pool.get_sportsman_id(&Sportsman::new(String::from("Aboba"))).await.unwrap();
    println!("{}", a);

    //let _ = Service::new().await.start().await;

    Ok(())
}
