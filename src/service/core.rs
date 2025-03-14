use crate::models::metrics::biathlon::Biathlon;
use crate::models::metrics::running::Running;
use crate::models::metrics::weight_lifting::WeightLifting;
use crate::models::performance_tracker::PerformanceTracker;
use crate::models::responses::Responses;
use crate::models::sportsman::Sportsman;
use crate::traits::traits::{Metric, Pool, SportPerformance};
use axum::extract::Path;
use axum::response::IntoResponse;
use std::env;

use crate::models::error::Error;
use crate::service::models::{BiathlonPerformance, RunningPerformance, WeightLiftingPerformance};
use crate::service::postgres::postgres_pool::DBPool;
use axum::routing::{delete, get, post};
use axum::{Extension, Json, Router};
use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;

pub struct Url(pub String);

impl Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

pub struct Service {
    router: Router,
    tcp_listener: TcpListener,
    tracker: Arc<PerformanceTracker>,
    pool: Arc<DBPool>,
}

impl Service {
    pub async fn new() -> Self {
        let pool = Arc::new(DBPool::new().await);

        let tracker = Arc::new(
            pool.get_performance_tracker()
                .await
                .expect("Couldn't load tracker"),
        );

        let url = Url(env::var("SERVICE_URL").expect("SERVICE_URL not found in .env file"));
        let tcp_listener = retry_to_bind(&url)
            .await
            .expect("Error in binding tcp_listener");

        let router = Router::new()
            .merge(routes_get_performance(Arc::clone(&tracker)))
            .merge(routes_add_performance(
                Arc::clone(&tracker),
                Arc::clone(&pool),
            ))
            .merge(routes_remove_performance(
                Arc::clone(&tracker),
                Arc::clone(&pool),
            ));

        Self {
            router,
            tcp_listener,
            tracker,
            pool,
        }
    }

    pub async fn start(self) {
        axum::serve(self.tcp_listener, self.router)
            .await
            .expect("Couldn't serve tcp listener and router");
    }
}

async fn retry_to_bind(url: &Url) -> Result<TcpListener, ()> {
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
        .route(
            "/weight_lifting/{name}",
            get(get_performance::<WeightLifting>),
        )
        .layer(Extension(tracker))
}

fn routes_add_performance(tracker: Arc<PerformanceTracker>, pool: Arc<DBPool>) -> Router {
    Router::new()
        .route(
            "/running/{name}",
            post(add_performance::<Running, RunningPerformance>),
        )
        .route(
            "/biathlon/{name}",
            post(add_performance::<Biathlon, BiathlonPerformance>),
        )
        .route(
            "/weight_lifting/{name}",
            post(add_performance::<WeightLifting, WeightLiftingPerformance>),
        )
        .layer(Extension((tracker, pool)))
}

fn routes_remove_performance(tracker: Arc<PerformanceTracker>, pool: Arc<DBPool>) -> Router {
    Router::new()
        .route("/running/{name}", delete(remove_performance::<Running>))
        .route("/biathlon/{name}", delete(remove_performance::<Biathlon>))
        .route(
            "/weight_lifting/{name}",
            delete(remove_performance::<WeightLifting>),
        )
        .layer(Extension((tracker, pool)))
}

async fn get_performance<T: Metric + Clone>(
    Extension(tracker): Extension<Arc<PerformanceTracker>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let sportsman = match Sportsman::new(name) {
        Ok(s) => s,
        Err(e) => return e.into_response(),
    };

    match tracker.get_performance::<T>(&sportsman).await {
        Ok(performance) => {
            log::info!("Performance: {:?}", performance);
            performance.into_response()
        }
        Err(e) => {
            log::info!("{}", e);
            Responses::Errors(e).into_response()
        }
    }
}

async fn add_performance<T, P>(
    Extension((tracker, pool)): Extension<(Arc<PerformanceTracker>, Arc<DBPool>)>,
    Path(name): Path<String>,
    Json(performance): Json<P>,
) -> impl IntoResponse
where
    T: Metric,
    P: Into<T>,
{
    let sportsman = match Sportsman::new(name) {
        Ok(s) => s,
        Err(e) => return e.into_response(),
    };
    let metric: T = performance.into();
    let response_name = metric.response_name();

    if let Err(e) = pool.add_sportsman(&sportsman).await {
        log::error!("Error while saving sportsman: {e}");
        return Responses::Errors(Error::SaveError).into_response();
    }

    if let Err(e) = pool.add_performance(&sportsman, metric.clone_box()).await {
        log::error!("Error while saving performance{e}");
        return Responses::Errors(Error::SaveError).into_response();
    }

    tracker.add_performance(sportsman, metric.clone_box()).await;
    log::info!("Performance was added successfully");

    Responses::PerformanceAdded(response_name).into_response()
}

async fn remove_performance<T: Metric>(
    Extension((tracker, pool)): Extension<(Arc<PerformanceTracker>, Arc<DBPool>)>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let sportsman = match Sportsman::new(name) {
        Ok(s) => s,
        Err(e) => return e.into_response(),
    };

    match pool.remove_performance::<T>(&sportsman).await {
        Ok(removed) => {
            if !removed {
                return Responses::PerformanceNotFound.into_response();
            }
        }
        Err(e) => {
            log::error!("Error while removing performance: {e}");
            return Responses::Errors(Error::RemoveError).into_response();
        }
    }

    match tracker.remove_performance::<T>(sportsman).await {
        Ok(_) => {
            log::info!("Performance was removed successfully");
            Responses::PerformanceRemoved.into_response()
        }
        Err(e) => {
            log::info!("{}", e);
            Responses::Errors(e).into_response()
        }
    }
}
