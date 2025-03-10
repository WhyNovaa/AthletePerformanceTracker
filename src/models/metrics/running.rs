use crate::traits::traits::Metric;
use axum::response::{IntoResponse, Json as AxumJson, Response};
use serde::Serialize;
use serde_json::json;
use std::any::Any;
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize)]
pub struct Distance(pub f32);

impl Distance {
    pub fn dist(&self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Speed(pub f32);

impl Speed {
    pub fn speed(&self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone)]
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

/*impl Debug for Running {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Distance: {}, Speed: {}", self.distance.0, self.speed.0)?;
        Ok(())
    }
}*/
impl IntoResponse for Running {
    fn into_response(self) -> Response {
        AxumJson(json!({
            "distance": self.speed.0,
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
}
