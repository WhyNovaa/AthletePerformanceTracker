use crate::models::error::Error;
use axum::response::{IntoResponse, Json as AxumJson, Response};
use serde_json::json;

pub enum Responses {
    PerformanceAdded(&'static str),
    PerformanceRemoved,
    Errors(Error),
}

impl IntoResponse for Responses {
    fn into_response(self) -> Response {
        match self {
            Responses::PerformanceAdded(name) => AxumJson(json!({
            "status": "200",
            "message": format!("{} performance added successfully", name),
            }))
            .into_response(),
            Responses::PerformanceRemoved => AxumJson(json!({
            "status": "200",
            "message": "performance removed successfully",
            }))
            .into_response(),
            Responses::Errors(e) => e.into_response(),
        }
    }
}
