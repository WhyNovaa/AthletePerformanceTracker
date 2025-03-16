use axum::http::StatusCode;
use axum::response::{IntoResponse, Json as AxumJson, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Display, Formatter};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub enum Error {
    SportsmanNotFound,
    SportsmanDoesntHasMetric,
    SaveError,
    RemoveError,
    NameTooLong,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SportsmanNotFound => write!(f, "Sportsman not found"),
            Error::SportsmanDoesntHasMetric => write!(f, "Metric wasn't found"),
            Error::SaveError => write!(f, "Something went wrong"),
            Error::RemoveError => write!(f, "Something went wrong"),
            Error::NameTooLong => write!(f, "Sportsman name is too long"),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::SportsmanNotFound => StatusCode::NOT_FOUND,
            Error::SportsmanDoesntHasMetric => StatusCode::NOT_FOUND,
            Error::SaveError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::RemoveError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NameTooLong => StatusCode::BAD_REQUEST,
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
