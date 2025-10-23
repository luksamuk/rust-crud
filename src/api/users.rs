use crate::app::App;
use crate::models::user::{NewUser, User};
use crate::schema::users;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use paginator_rocket::{PaginatedJson, Pagination};
use rocket::{Route, State, get, serde::json::Json};
use rocket_anyhow::Result;

pub fn routes() -> Vec<Route> {
    routes![list_users, create_user]
}

#[get("/")]
async fn list_users(app: &State<App>, pagination: Pagination) -> Result<PaginatedJson<User>> {
    let mut conn = app.connections.get().await?;
    let count = users::table.count().get_result::<i64>(&mut conn).await? as u32;
    let data: Vec<User> = users::table
        .select(User::as_select())
        .order_by(users::id.asc())
        .load(&mut conn)
        .await?;

    Ok(PaginatedJson::new(data, &pagination.params, count))
}

#[post("/", format = "json", data = "<new_user>")]
async fn create_user(app: &State<App>, new_user: Json<NewUser>) -> Result<Json<User>> {
    let user = User::try_from(new_user.into_inner())?;

    let mut conn = app.connections.get().await?;

    Ok(Json(
        diesel::insert_into(users::table)
            .values(&user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await?,
    ))
}
