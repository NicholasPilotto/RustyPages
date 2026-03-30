use axum::{
    Json,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

pub enum ValidationError {
    JsonError(axum::extract::rejection::JsonRejection),
    InvalidPayload(validator::ValidationErrors),
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        match self {
            ValidationError::JsonError(e) => {
                (StatusCode::BAD_REQUEST, e.to_string()).into_response()
            }
            ValidationError::InvalidPayload(errors) => {
                (StatusCode::BAD_REQUEST, Json(errors)).into_response()
            }
        }
    }
}

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ValidationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(ValidationError::JsonError)?;

        value.validate().map_err(ValidationError::InvalidPayload)?;

        Ok(ValidatedJson(value))
    }
}
