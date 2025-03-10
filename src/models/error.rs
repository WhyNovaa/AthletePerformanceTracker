use axum::response::{IntoResponse, Json as AxumJson, Response};
use serde_json::json;

pub enum Error {
    NoSuchSportsman,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::NoSuchSportsman => {
                AxumJson(json!({
                "status": "404",
                "message": "sportsman wasn't found",
                }))
            }
            .into_response(),
        }
    }
}
