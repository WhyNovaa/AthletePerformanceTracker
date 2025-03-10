use crate::models::sportsman::Sportsman;
use crate::traits::traits::{Metric, SportPerformance};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type Metrics = Vec<Box<dyn Metric>>;
type Performances = Arc<RwLock<HashMap<Sportsman, Metrics>>>;

#[derive(Debug)]
pub struct PerformanceTracker {
    performances: Performances,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        Self {
            performances: Performances::new(Default::default()),
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

    async fn get_performance<T: Metric + Clone>(&self, sportsman: &Sportsman) -> Option<T> {
        let perf_guard = self.performances.read().await;
        perf_guard
            .get(sportsman)?
            .iter()
            .find_map(|m| m.as_any().downcast_ref::<T>())
            .cloned()
    }
}
