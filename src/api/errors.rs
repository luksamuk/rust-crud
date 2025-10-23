use anyhow::anyhow;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct AppError {
    pub detail: String,
}

impl AppError {
    pub fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }
}

impl From<AppError> for rocket_anyhow::Error {
    fn from(error: AppError) -> Self {
        anyhow!(error.detail).into()
    }
}
