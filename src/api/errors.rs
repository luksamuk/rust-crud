use deadpool::managed::PoolError;
use diesel::result::Error as DieselError;
use rocket::response::Responder;
use rocket::serde::json::Json;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, serde::Serialize, Responder)]
pub struct ErrorPayload {
    pub details: String,
}

impl<S: Into<String>> From<S> for ErrorPayload {
    fn from(value: S) -> Self {
        Self {
            details: value.into(),
        }
    }
}

// impl std::fmt::Display for ErrorPayload {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let json = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
//         write!(f, "{}", json)
//     }
// }

impl std::fmt::Display for ErrorPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"details\": \"{}\"}}", self.details)
    }
}

#[derive(Debug, thiserror::Error, Responder)]
#[response(status = 500, content_type = "json")]
pub enum ApiError {
    #[response(status = 400, content_type = "json")]
    #[error("Bad request: {0}")]
    BadRequest(ErrorPayload),

    #[response(status = 401, content_type = "json")]
    #[error("Unauthorized: {0}")]
    Unauthorized(ErrorPayload),

    #[response(status = 404, content_type = "json")]
    #[error("Not found: {0}")]
    NotFound(ErrorPayload),

    #[response(status = 422, content_type = "json")]
    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(ErrorPayload),

    #[response(status = 500, content_type = "json")]
    #[error("Internal server error (I/O): {0}")]
    Io(#[from] std::io::Error),

    #[response(status = 500, content_type = "json")]
    #[error("Internal server error: {0}")]
    InternalServerError(ErrorPayload),
}

impl From<deadpool::managed::PoolError<diesel_async::pooled_connection::PoolError>> for ApiError {
    fn from(error: PoolError<diesel_async::pooled_connection::PoolError>) -> Self {
        ApiError::InternalServerError(error.to_string().into())
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(error: DieselError) -> Self {
        ApiError::InternalServerError(error.to_string().into())
    }
}
