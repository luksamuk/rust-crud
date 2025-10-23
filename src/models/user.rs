use crate::auth::password::generate_hash;
use crate::schema::users;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use rocket_anyhow::Result;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    #[diesel(sql_type = Bytea)]
    pub password_hash: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl TryFrom<NewUser> for User {
    type Error = rocket_anyhow::Error;

    fn try_from(new_user: NewUser) -> Result<Self> {
        Ok(User {
            id: Uuid::now_v7(),
            username: new_user.username.to_owned(),
            email: new_user.email.to_owned(),
            password_hash: generate_hash(&new_user.password)?,
            created_at: Utc::now(),
            updated_at: None,
        })
    }
}
