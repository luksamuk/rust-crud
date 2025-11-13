use rocket::response::Responder;
use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Responder, Error, Debug)]
pub enum ApiError {
    #[response(status = 400, content_type = "json")]
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[response(status = 401, content_type = "json")]
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[response(status = 404, content_type = "json")]
    #[error("Not found: {0}")]
    NotFound(String),

    #[response(status = 422, content_type = "json")]
    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),

    #[response(status = 500, content_type = "json")]
    #[error("Internal server error (I/O): {0}")]
    Io(#[from] std::io::Error),

    #[response(status = 500, content_type = "json")]
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

pub trait IntoApiError {
    fn into_api_error(self) -> ApiError;
}

impl<E> IntoApiError for E
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn into_api_error(self) -> ApiError {
        ApiError::InternalServerError(self.to_string())
    }
}
