use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;
use serde::Serialize;
use std::fmt;
use utoipa::ToSchema;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
    // Domain errors
    NotFound(String),
    BadRequest(String),
    ValidationFailed(ValidationErrors),

    // Infrastructure errors
    DatabaseError(DbErr),
    ConnectionError(DbErr),

    // Catch-all
    InternalServerError(String),
}

impl From<DbErr> for AppError {
    fn from(e: DbErr) -> Self {
        match e {
            // Record not found
            DbErr::RecordNotFound(msg) => AppError::NotFound(msg),

            // Connection-level issues
            DbErr::Conn(err) => AppError::ConnectionError(DbErr::Conn(err)),

            // Query issues (bad SQL, constraint violations, etc.)
            DbErr::Query(err) => AppError::DatabaseError(DbErr::Query(err)),

            // Everything else
            other => AppError::InternalServerError(other.to_string()),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "{}", msg),
            AppError::ValidationFailed(errors) => write!(f, "{}", errors),
            AppError::BadRequest(msg) => write!(f, "{}", msg),
            AppError::InternalServerError(msg) => write!(f, "{}", msg),
            AppError::DatabaseError(db_err) => write!(f, "{}", db_err.to_string()),
            AppError::ConnectionError(db_err) => write!(f, "{}", db_err.to_string()),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound(msg) => {
                ErrorResponse::new(StatusCode::NOT_FOUND, msg).into_response()
            }

            AppError::BadRequest(msg) => {
                ErrorResponse::new(StatusCode::BAD_REQUEST, msg).into_response()
            }

            AppError::ValidationFailed(errors) => {
                let details = serde_json::to_value(&errors).ok();
                ErrorResponse::new(StatusCode::BAD_REQUEST, "Validation failed")
                    .with_details(details.unwrap_or_default())
                    .into_response()
            }

            AppError::DatabaseError(e) => {
                // Log the internal detail, return a safe message to the client
                eprintln!("[DB ERROR] {:?}", e);
                ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error")
                    .into_response()
            }

            AppError::ConnectionError(e) => {
                eprintln!("[CONNECTION ERROR] {:?}", e);
                ErrorResponse::new(StatusCode::SERVICE_UNAVAILABLE, "Database unavailable")
                    .into_response()
            }

            AppError::InternalServerError(msg) => {
                eprintln!("[INTERNAL ERROR] {}", msg);
                ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
                    .into_response()
            }
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code: code.as_u16(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}
