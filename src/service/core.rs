use crate::traits::traits::Pool;
use std::env;

use crate::api::api_doc::ApiDoc;
use crate::api::handlers::retry_to_bind;
use crate::db::postgres_pool::DBPool;
use crate::models::performance_tracker::PerformanceTracker;
use crate::service::routes::{
    routes_add_performance, routes_get_performance, routes_remove_performance,
};
use axum::Router;
use std::fmt::Display;
use std::sync::Arc;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct Url(pub String);

impl Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
            .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
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
