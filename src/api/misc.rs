use crate::api::errors::ApiError;
use rocket::{Catcher, Route};

pub fn routes() -> Vec<Route> {
    routes![ping]
}

pub fn catchers() -> Vec<Catcher> {
    catchers![not_found]
}

#[get("/ping")]
async fn ping() -> &'static str {
    "pong"
}

#[catch(404)]
async fn not_found() -> ApiError {
    ApiError::NotFound("Resource not found".into())
}
