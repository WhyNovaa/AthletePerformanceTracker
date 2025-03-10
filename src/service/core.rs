use axum::response::IntoResponse;
use crate::models::error::Responses;
use crate::models::metrics::biathlon::Biathlon;
use crate::models::metrics::running::Running;
use crate::models::metrics::weightlifting::WeightLifting;
use crate::models::performance_tracker::PerformanceTracker;
use crate::models::sportsman::Sportsman;
use crate::traits::traits::{Metric, SportPerformance};
use axum::extract::Path;

use axum::routing::{get, post};
use axum::{Extension, Json, Router};

use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use crate::service::models::{BiathlonPerformance, RunningPerformance, WeightLiftingPerformance};

pub struct URL(pub String);

impl Display for URL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

pub struct Service {
    router: Router,
    tcp_listener: TcpListener,
    tracker: Arc<PerformanceTracker>,
}

impl Service {
    pub async fn new(url: URL) -> Self {
        let tcp_listener = retry_to_bind(&url)
            .await
            .expect("Error in binding tcp_listener");
        let tracker = Arc::new(PerformanceTracker::new());
        let router = Router::new()
            .merge(routes_get_performance(Arc::clone(&tracker)))
            .merge(routes_add_performance(Arc::clone(&tracker)));

        Self {
            router,
            tcp_listener,
            tracker,
        }
    }

    pub async fn start(self) {
        axum::serve(self.tcp_listener, self.router)
            .await
            .expect("Couldn't serve tcp listener and router");
    }
}

async fn retry_to_bind(url: &URL) -> Result<TcpListener, ()> {
    for _ in 0..5 {
        log::info!("Trying to bind service at {}", url);
        match TcpListener::bind(url.to_string()).await {
            Ok(listener) => {
                log::info!("Tcp listener bound successfully");
                return Ok(listener);
            }
            Err(e) => {
                log::error!("Tcp listener bind error: {}\n Retrying...", e);
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Err(())
}

fn routes_get_performance(tracker: Arc<PerformanceTracker>) -> Router {
    Router::new()
        .route("/running/{name}", get(get_performance::<Running>))
        .route("/biathlon/{name}", get(get_performance::<Biathlon>))
        .route("/weight_lifting/{name}", get(get_performance::<WeightLifting>))
        .layer(Extension(tracker))
}

fn routes_add_performance(tracker: Arc<PerformanceTracker>) -> Router {
    Router::new()
        .route("/running/{name}", post(add_performance::<Running, RunningPerformance>))
        .route("/biathlon/{name}", post(add_performance::<Biathlon, BiathlonPerformance>))
        .route("/weight_lifting/{name}", post(add_performance::<WeightLifting, WeightLiftingPerformance>))
        .layer(Extension(tracker))
}


async fn get_performance<T: Metric + Clone>(
    Extension(tracker): Extension<Arc<PerformanceTracker>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let sportsman = Sportsman::new(name);

    if let Some(performance) = tracker.get_performance::<T>(&sportsman).await {
        log::info!("Performance: {:?}", performance);
        performance.into_response()
    } else {
        log::error!("Sportsman wasn't found");
        Responses::NoSuchSportsman.into_response()
    }
}

async fn add_performance<T, P>(
    Extension(tracker): Extension<Arc<PerformanceTracker>>,
    Path(name): Path<String>,
    Json(performance): Json<P>,
) -> impl IntoResponse
where
    T: Metric,
    P: Into<T>,
{
    let sportsman = Sportsman::new(name);
    let metric: T = performance.into();
    let response_name = metric.response_name();

    tracker.add_performance(sportsman, Box::new(metric)).await;

    Responses::PerformanceAdded(response_name).into_response()
}