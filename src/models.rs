use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub created_at: std::time::SystemTime,
    pub updated_at: std::time::SystemTime,
    pub username: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Selectable)]
#[diesel(table_name = users)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}
