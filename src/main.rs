#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod api;
mod app;
mod auth;
mod db;
mod models;
mod schema;

use app::App;
use rocket_anyhow::Result;

#[rocket::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();

    let _rocket = rocket::build()
        .attach(App::new()?)
        .mount("/misc", api::misc::routes())
        .mount("/users", api::users::routes())
        .launch()
        .await?;

    Ok(())
}
