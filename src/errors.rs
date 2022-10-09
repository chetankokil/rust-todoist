use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    MissingCredential,
    InternalServerError,
    UserAlreadyExists,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, response) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an internal server error occured",
            ),
            Self::UserAlreadyExists => (StatusCode::BAD_REQUEST, "User already Exists"),
            Self::MissingCredential => (StatusCode::BAD_REQUEST, "Missing Credentials"),
        };
        (status, Json(json!({ "error": response }))).into_response()
    }
}
