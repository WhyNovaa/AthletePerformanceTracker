use crate::models::error::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json as AxumJson, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub enum Responses {
    PerformanceAdded(&'static str),
    PerformanceRemoved,
    PerformanceNotFound,
    SportsmanNotFound,
    InvalidPerformanceFormat(&'static str),
    Errors(Error),
}

impl IntoResponse for Responses {
    fn into_response(self) -> Response {
        let status = match self {
            Responses::PerformanceAdded(_) => StatusCode::OK,
            Responses::PerformanceRemoved => StatusCode::OK,
            Responses::PerformanceNotFound => StatusCode::NOT_FOUND,
            Responses::SportsmanNotFound => StatusCode::NOT_FOUND,
            Responses::InvalidPerformanceFormat(_) => StatusCode::BAD_REQUEST,
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
                    "message": "Performance removed successfully",
                });
                (status, AxumJson(json)).into_response()
            }
            Responses::PerformanceNotFound => {
                let json = json!({
                    "message": "Performance not found",
                });
                (status, AxumJson(json)).into_response()
            }
            Responses::SportsmanNotFound => {
                let json = json!({
                    "message": "Sportsman not found",
                });
                (status, AxumJson(json)).into_response()
            }
            Responses::InvalidPerformanceFormat(name) => {
                let json = json!({
                    "message": format!("Invalid {} format", name),
                });
                (status, AxumJson(json)).into_response()
            }
            Responses::Errors(e) => e.into_response(),
        }
    }
}
