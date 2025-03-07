use std::any::Any;
use std::fmt::{Debug, Display};
use crate::models::sportsman::Sportsman;

pub trait Metric: Any + Debug {
    fn as_any(&self) -> &dyn Any;
    fn clone_box(&self) -> Box<dyn Metric>;
}

pub trait SportPerformance {
    async fn add_performance(&mut self, sportsman: Sportsman, metric: Box<dyn Metric>);
    async fn get_performance<T: Metric + Clone>(&self, sportsman: &Sportsman) -> Option<T>;
}