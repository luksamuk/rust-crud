use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![ping]
}
#[get("/ping")]
async fn ping() -> &'static str {
    return "pong";
}
