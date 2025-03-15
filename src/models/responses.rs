use axum::http::StatusCode;
use crate::models::error::Error;
use axum::response::{IntoResponse, Json as AxumJson, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub enum Responses {
    PerformanceAdded(&'static str),
    PerformanceRemoved,
    PerformanceNotFound,
    Errors(Error),
}

impl IntoResponse for Responses {
    fn into_response(self) -> Response {
        let status = match self {
            Responses::PerformanceAdded(_) => StatusCode::OK,
            Responses::PerformanceRemoved => StatusCode::OK,
            Responses::PerformanceNotFound => StatusCode::OK,
            Responses::Errors(_) => StatusCode::NOT_FOUND,
        };
        match self {
            Responses::PerformanceAdded(name) => {
                let json = json!({
                    "message": format!("{} performance added successfully", name),
                });
                (status, AxumJson(json)).into_response()
            }
            Responses::PerformanceRemoved => {
                let json = json!({
                    "message": "performance removed successfully",
                });
                (status, AxumJson(json)).into_response()
            }
            Responses::PerformanceNotFound => {
                let json = json!({
                    "message": "performance not found",
                });
                (status, AxumJson(json)).into_response()
            }
            Responses::Errors(e) => e.into_response(),
        }
    }
}