use crate::traits::traits::Metric;
use axum::response::{IntoResponse, Json as AxumJson, Response};
use serde_json::json;
use std::any::Any;
use std::fmt::Debug;
use utoipa::ToSchema;

#[derive(Debug, Clone, ToSchema)]
pub struct Distance(pub f32);

#[derive(Debug, Clone, ToSchema)]
pub struct Speed(pub f32);

#[derive(Debug, Clone, ToSchema)]
pub struct Running {
    /// distance in km
    pub distance: Distance,
    /// sportsman's speed in km per hour_
    pub speed: Speed,
}

impl Running {
    pub fn new(distance: Distance, speed: Speed) -> Self {
        Self { distance, speed }
    }
}

impl IntoResponse for Running {
    fn into_response(self) -> Response {
        AxumJson(json!({
            "distance": self.distance.0,
            "speed:": self.speed.0,
        }))
        .into_response()
    }
}

impl Metric for Running {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Metric> {
        Box::new(self.clone())
    }

    fn response_name(&self) -> &'static str {
        "Running"
    }
}
