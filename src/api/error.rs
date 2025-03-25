use axum::http::StatusCode;
use axum::response::{IntoResponse, Json as AxumJson, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum Error {
    #[error("Sportsman not found")]
    SportsmanNotFound,
    #[error("Performance wasn't found")]
    SportsmanDoesntHasMetric,
    #[error("Something went wrong")]
    SaveError,
    #[error("Something went wrong")]
    RemoveError,
    #[error("Wrong input data: {0}")]
    ValidationError(#[from] validator::ValidationError),
    #[error("Wrong input data")]
    ValidationErrors(#[from] validator::ValidationErrors),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::SportsmanNotFound => StatusCode::NOT_FOUND,
            Error::SportsmanDoesntHasMetric => StatusCode::NOT_FOUND,
            Error::SaveError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::RemoveError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ValidationError(_) => StatusCode::BAD_REQUEST,
            Error::ValidationErrors(_) => StatusCode::BAD_REQUEST,
        };

        (
            status,
            AxumJson(json!({
                "message": self.to_string(),
            })),
        )
            .into_response()
    }
}
