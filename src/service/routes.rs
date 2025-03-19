use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::{delete, get, post};
use crate::api::handlers::{add_performance_by_sport, get_performance_by_sport, remove_performance_by_sport};
use crate::db::postgres_pool::DBPool;
use crate::models::performance_tracker::PerformanceTracker;

pub fn routes_get_performance(tracker: Arc<PerformanceTracker>) -> Router {
    Router::new()
        .route("/{sport}/{name}", get(get_performance_by_sport))
        .layer(Extension(tracker))
}

pub fn routes_add_performance(tracker: Arc<PerformanceTracker>, pool: Arc<DBPool>) -> Router {
    Router::new()
        .route("/{sport}/{name}", post(add_performance_by_sport))
        .layer(Extension((tracker, pool)))
}

pub fn routes_remove_performance(tracker: Arc<PerformanceTracker>, pool: Arc<DBPool>) -> Router {
    Router::new()
        .route("/{sport}/{name}", delete(remove_performance_by_sport))
        .layer(Extension((tracker, pool)))
}