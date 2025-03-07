use tokio::net::TcpListener;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use axum::Router;
use crate::models::performance_tracker::PerformanceTracker;

pub struct URL(String);

impl ToString for URL {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
pub struct Service {
    router: Router,
    tcp_listener: TcpListener,
    tracker: PerformanceTracker,
}

impl Service {
    async fn new(url: URL) -> Self {
        let router = Router::new();
        let tcp_listener = retry_to_bind(url).await;
        let tracker = PerformanceTracker::new();
        Self {
            router,
            tcp_listener,
            tracker,
        }
    }

    async fn start(self) {
        axum::serve(self.tcp_listener, self.router).await.unwrap();
    }
}

async fn retry_to_bind(url: URL) -> TcpListener {
    loop {
        log::info!("Retrying to bind service at {}", url.to_string());
        match TcpListener::bind(url.to_string()).await {
            Ok(listener) => {
                log::info!("Tcp listener bound successfully");
                return listener;
            },
            Err(e) => {
                log::error!("Tcp listener bind error: {}\n Retrying...", e);
            }
        }
        sleep(Duration::from_secs(1));
    }
}
