use super::db::Users;
use crate::errors::UserError::*;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    result::Error as DieselError,
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
use rand::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, diesel::prelude::Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUserRequest {
    username: String,
    password: String,
}

#[derive(Deserialize, diesel::prelude::Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct LoginRequest {
    username: String,
    password: String,
}

impl Users for PooledConnection<ConnectionManager<PgConnection>> {
    fn create_user(
        &mut self,
        user: super::users::NewUserRequest,
    ) -> Result<(), crate::errors::UserError> {
        use crate::schema::users::dsl::*;

        let salt = &random::<[u8; 32]>();
        let hash =
            argon2::hash_encoded(user.password.as_bytes(), salt, &Default::default()).unwrap();

        diesel::insert_into(users)
            .values(NewUserRequest {
                username: user.username,
                password: hash,
            })
            .execute(self)
            .map(|_| ())
            .map_err(|err| match err {
                DieselError::DatabaseError(..) => UserAlreadyExistsOrRequestInvalid,
                _ => crate::errors::UserError::InternalError,
            })
    }

    fn login(&mut self, user: super::users::LoginRequest) -> Result<(), crate::errors::UserError> {
        use crate::schema::users::dsl::*;

        let password_hashed = users
            .select(password)
            .filter(username.eq(user.username))
            .first::<String>(self)
            .map_err(|_| InvalidRequest)?;

        argon2::verify_encoded(&password_hashed.as_str(), user.password.as_bytes())
            .map(|_| ())
            .map_err(|_| PermissionDenied)
    }
}
