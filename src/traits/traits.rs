use crate::models::sportsman::Sportsman;
use axum::response::IntoResponse;
use std::any::Any;
use std::fmt::Debug;

pub trait Metric: Any + Debug + IntoResponse + Sync + Send {
    fn as_any(&self) -> &dyn Any;
    fn clone_box(&self) -> Box<dyn Metric>;
}

pub trait SportPerformance {
    async fn add_performance(&self, sportsman: Sportsman, metric: Box<dyn Metric>);
    async fn get_performance<T: Metric + Clone>(&self, sportsman: &Sportsman) -> Option<T>;
}
