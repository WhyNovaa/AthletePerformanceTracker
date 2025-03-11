use axum::response::{IntoResponse, Json as AxumJson, Response};
use serde_json::json;
use std::fmt::{Display, Formatter};

pub enum Error {
    SportsmanNotFound,
    SportsmanDoesntHasMetric,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SportsmanNotFound => write!(f, "Sportsman not found"),
            Error::SportsmanDoesntHasMetric => write!(f, "Metric wasn't found"),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::SportsmanNotFound => "404",
            Error::SportsmanDoesntHasMetric => "404",
        };
        AxumJson(json!({
            "status": status,
            "message:": self.to_string(),
        }))
        .into_response()
    }
}
