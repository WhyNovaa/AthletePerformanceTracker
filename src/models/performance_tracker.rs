use crate::api::error::Error;
use crate::models::sportsman::Sportsman;
use crate::traits::traits::{Metric, SportPerformance};
use std::any::TypeId;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub type Metrics = Vec<Box<dyn Metric>>;
type Performances = RwLock<HashMap<Sportsman, Metrics>>;

#[derive(Debug)]
pub struct PerformanceTracker {
    performances: Performances,
}

impl PerformanceTracker {
    pub fn new(sportsmen_to_metrics: HashMap<Sportsman, Metrics>) -> Self {
        Self {
            performances: RwLock::new(sportsmen_to_metrics),
        }
    }
}

impl SportPerformance for PerformanceTracker {
    /// if you add performance it can replace "old" performance that has the same type
    async fn add_performance(&self, sportsman: Sportsman, metric: Box<dyn Metric>) {
        let mut perf_guard = self.performances.write().await;

        let existing_metrics = perf_guard.entry(sportsman).or_insert_with(Vec::new);

        for existing_metric in existing_metrics.iter_mut() {
            if existing_metric.as_any().type_id() == metric.as_any().type_id() {
                *existing_metric = metric;
                return;
            }
        }

        existing_metrics.push(metric);
    }

    async fn get_performance<T: Metric + Clone>(&self, sportsman: &Sportsman) -> Result<T, Error> {
        let perf_guard = self.performances.read().await;

        if let Some(metrics) = perf_guard.get(sportsman) {
            for metric in metrics.iter() {
                if let Some(down_casted) = metric.as_any().downcast_ref::<T>() {
                    return Ok(down_casted.to_owned());
                }
            }
            Err(Error::SportsmanDoesntHasMetric)
        } else {
            Err(Error::SportsmanNotFound)
        }
    }

    async fn remove_performance<T: Metric>(&self, sportsman: Sportsman) -> Result<(), Error> {
        let mut perf_guard = self.performances.write().await;

        if let Some(existing_metrics) = perf_guard.get_mut(&sportsman) {
            if let Some(ind) = existing_metrics
                .iter()
                .position(|m| m.as_any().type_id() == TypeId::of::<T>())
            {
                existing_metrics.swap_remove(ind);
                return Ok(());
            }
            Err(Error::SportsmanDoesntHasMetric)
        } else {
            Err(Error::SportsmanNotFound)
        }
    }
}
