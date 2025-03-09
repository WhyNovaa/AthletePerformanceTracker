use std::fmt::Display;
use tokio::net::TcpListener;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use axum::Router;
use axum::routing::get;
use crate::models::error::Error;
use crate::models::performance_tracker::PerformanceTracker;

pub struct URL(pub String);

impl Display for URL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

pub struct Service {
    router: Router,
    tcp_listener: TcpListener,
    tracker: PerformanceTracker,
}

impl Service {
    pub async fn new(url: URL) -> Self {
        let router = Router::new()
            .route("/", get(|| async {
                "Hello world"
            }));
        let tcp_listener = retry_to_bind(&url).await.expect("Error in binding tcp_listener");
        let tracker = PerformanceTracker::new();

        Self {
            router,
            tcp_listener,
            tracker,
        }
    }

    pub async fn start(self) {
        axum::serve(self.tcp_listener, self.router).await.expect("Couldn't serve tcp listener and router");
    }
}

async fn retry_to_bind(url: &URL) -> Result<TcpListener, ()> {
    for _ in 0..5 {
        log::info!("Trying to bind service at {}", url);
        match TcpListener::bind(url.to_string()).await {
            Ok(listener) => {
                log::info!("Tcp listener bound successfully");
                return Ok(listener);
            },
            Err(e) => {
                log::error!("Tcp listener bind error: {}\n Retrying...", e);
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Err(())
}
