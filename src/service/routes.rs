use crate::api::handlers::{
    add_performance_by_sport, get_performance_by_sport, remove_performance_by_sport,
};
use crate::models::performance_tracker::PerformanceTracker;
use crate::traits::traits::Pool;
use axum::routing::{delete, get, post};
use axum::{Extension, Router};
use std::sync::Arc;

pub fn routes_get_performance(tracker: Arc<PerformanceTracker>) -> Router {
    Router::new()
        .route("/{sport}/{name}", get(get_performance_by_sport))
        .layer(Extension(tracker))
}

pub fn routes_add_performance<P: Pool>(tracker: Arc<PerformanceTracker>, pool: Arc<P>) -> Router {
    Router::new()
        .route("/{sport}/{name}", post(add_performance_by_sport))
        .layer(Extension((tracker, Arc::clone(&pool))))
}

pub fn routes_remove_performance<P: Pool>(
    tracker: Arc<PerformanceTracker>,
    pool: Arc<P>,
) -> Router {
    Router::new()
        .route("/{sport}/{name}", delete(remove_performance_by_sport))
        .layer(Extension((tracker, Arc::clone(&pool))))
}
