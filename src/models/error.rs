use axum::response::{IntoResponse, Json as AxumJson, Response};
use serde_json::json;
use std::fmt::{Display, Formatter};

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
            Error::SportsmanNotFound => "404",
            Error::SportsmanDoesntHasMetric => "404",
            Error::SaveError => "500",
            Error::RemoveError => "500",
            Error::NameTooLong => "400",
        };
        AxumJson(json!({
            "status": status,
            "message:": self.to_string(),
        }))
        .into_response()
    }
}
