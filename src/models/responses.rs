use axum::response::{IntoResponse, Json as AxumJson, Response};
use serde_json::json;

pub enum Responses {
    NoSuchSportsman,
    PerformanceAdded(&'static str),
}

impl IntoResponse for Responses {
    fn into_response(self) -> Response {
        match self {
            Responses::NoSuchSportsman => {
                AxumJson(json!({
                "status": "404",
                "message": "sportsman wasn't found",
                })).into_response()
            },
            Responses::PerformanceAdded(name) => {
                AxumJson(json!({
                "status": "200",
                "message": format!("{} performance added successfully", name),
                })).into_response()
            },
        }
    }
}
