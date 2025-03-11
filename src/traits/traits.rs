use crate::models::error::Error;
use crate::models::sportsman::Sportsman;
use axum::response::IntoResponse;
use std::any::Any;
use std::fmt::Debug;
pub trait Metric: Any + Debug + IntoResponse + Sync + Send {
    fn as_any(&self) -> &dyn Any;
    fn clone_box(&self) -> Box<dyn Metric>;
    fn response_name(&self) -> &'static str;
}

pub trait SportPerformance {
    async fn add_performance(&self, sportsman: Sportsman, metric: Box<dyn Metric>);
    async fn get_performance<T: Metric + Clone>(&self, sportsman: &Sportsman) -> Result<T, Error>;
    async fn remove_performance<T: Metric>(&self, sportsman: Sportsman) -> Result<(), Error>;
}
