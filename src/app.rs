use crate::db;
use rocket::{
    Build, Rocket,
    fairing::{Fairing, Info, Kind},
};
use rocket_anyhow::Result;

#[derive(Clone)]
pub struct App {
    pub connections: db::AsyncPool,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            connections: db::build_connection_pool()?,
        })
    }
}

#[rocket::async_trait]
impl Fairing for App {
    fn info(&self) -> Info {
        Info {
            name: "Application State",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        Ok(rocket.manage(self.clone()))
    }
}
