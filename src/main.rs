use crate::models::metrics::running::{Distance, Running, Speed};
use crate::models::metrics::weightlifting::{LiftedWeight, Weight, WeightLifting};
use crate::models::performance_tracker::PerformanceTracker;
use crate::models::sportsman::Sportsman;
use crate::traits::traits::SportPerformance;

mod models;
mod traits;

#[tokio::main]
async fn main() {
    let name = String::from("John");
    let s1 = Sportsman::new(name.clone());
    let s2 = Sportsman::new(name);

    let mut tracker = PerformanceTracker::new();
    tracker.add_performance(s1.clone(), Box::new( Running::new(Distance(12_f32), Speed(15_f32) ))).await;

    println!("{:?}", tracker);

    tracker.add_performance(
        s1,
        Box::new(
            Running::new(
                Distance(100_f32),
                Speed(1000_f32)
            )
    )).await;

    println!("{:?}", tracker);

    tracker.add_performance(s2.clone(), Box::new( WeightLifting::new(Weight(123_f32), LiftedWeight(123_f32)))).await;

    println!("{:?}", tracker);

    tracker.add_performance(s2, Box::new( WeightLifting::new(Weight(123123_f32), LiftedWeight(123123_f32)))).await;

    println!("{:?}", tracker);
}
