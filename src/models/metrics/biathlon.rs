use crate::traits::traits::Metric;
use axum::response::{IntoResponse, Json as AxumJson, Response};
use serde_json::json;
use std::any::Any;
use std::fmt::Debug;
use utoipa::ToSchema;

#[derive(Debug, Clone, ToSchema)]
pub struct Accuracy(pub f32);

#[derive(Debug, Clone, ToSchema)]
pub struct Distance(pub f32);

#[derive(Debug, Clone, ToSchema)]
pub struct Speed(pub f32);

#[derive(Debug, Clone, ToSchema)]
pub struct Biathlon {
    /// Shooting accuracy
    pub accuracy: Accuracy,
    /// Distance in km
    pub distance: Distance,
    /// Speed in km per hour
    pub speed: Speed,
}

impl Biathlon {
    pub fn new(accuracy: Accuracy, distance: Distance, speed: Speed) -> Self {
        Self {
            accuracy,
            distance,
            speed,
        }
    }
}

impl IntoResponse for Biathlon {
    fn into_response(self) -> Response {
        AxumJson(json!({
            "accuracy": self.accuracy.0,
            "distance": self.distance.0,
            "speed:": self.speed.0,
        }))
        .into_response()
    }
}

impl Metric for Biathlon {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Metric> {
        Box::new(self.clone())
    }

    fn response_name(&self) -> &'static str {
        "Biathlon"
    }
}
