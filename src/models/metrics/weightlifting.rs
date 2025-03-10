use crate::traits::traits::Metric;
use axum::response::{IntoResponse, Json as AxumJson, Response};
use serde_json::json;
use std::any::Any;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Weight(pub f32);

impl Weight {
    pub fn weight(&self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct LiftedWeight(pub f32);

impl LiftedWeight {
    pub fn lifted_weight(&self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct WeightLifting {
    /// Own weight
    pub weight: Weight,
    /// Summary lifted weight
    pub lifted_weight: LiftedWeight,
}

impl WeightLifting {
    pub fn new(weight: Weight, lifted_weight: LiftedWeight) -> Self {
        Self {
            weight,
            lifted_weight,
        }
    }
}

impl IntoResponse for WeightLifting {
    fn into_response(self) -> Response {
        AxumJson(json!({
            "weight": self.weight.0,
            "lifted_weight": self.lifted_weight.0,
        }))
        .into_response()
    }
}

impl Metric for WeightLifting {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Metric> {
        Box::new(self.clone())
    }

    fn response_name(&self) -> &'static str {
        "WeightLifting"
    }
}
