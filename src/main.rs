use crate::models::metrics::running::Running;
use crate::models::sportsman::Sportsman;
use crate::service::postgres::postgres_pool::DBPool;
use dotenv::dotenv;

mod models;
mod service;
mod traits;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let pool = DBPool::new().await;
    //pool.add_sportsman(&Sportsman::new(String::from("Aboba"))).await?;
    //pool.add_sportsman(&Sportsman::new(String::from("Aboba1"))).await?;
    //pool.add_sportsman(&Sportsman::new(String::from("Aboba2"))).await?;
    let sportsmen = pool.get_all_sportsmen().await?;
    println!("{:?}", sportsmen);
    let id = pool
        .get_sportsman_id(&Sportsman::new(String::from("Aboba")))
        .await?;
    //pool.add_metric(id, Running::new(Distance(1_f32), Speed(2_f32))).await?;
    //pool.add_metric(id, Biathlon::new(biathlon::Accuracy(1_f32), biathlon::Distance(2_f32), biathlon::Speed(3_f32))).await?;
    //pool.add_metric(id, WeightLifting::new(weight_lifting::Weight(1_f32), weight_lifting::LiftedWeight(2_f32))).await?;
    println!("{:?}", pool.remove_metric_if_exists::<Running>(id).await?);
    println!("{:?}", pool.remove_metric_if_exists::<Running>(id).await?);
    println!("{}", id);

    //let _ = Service::new().await.start().await;

    Ok(())
}
