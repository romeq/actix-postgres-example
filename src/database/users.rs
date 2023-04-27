use super::db::Users;
use crate::errors::UserError::{self, *};
use crate::models::{NewUser, User, UserLogin};
use diesel::SelectableHelper;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    result::Error as DieselError,
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
use rand::prelude::*;

impl Users for PooledConnection<ConnectionManager<PgConnection>> {
    fn register(&mut self, user: NewUser) -> Result<uuid::Uuid, UserError> {
        use crate::schema::users::dsl::*;

        let salt = &random::<[u8; 32]>();
        let hash =
            argon2::hash_encoded(user.password.as_bytes(), salt, &Default::default()).unwrap();

        diesel::insert_into(users)
            .values(NewUser {
                username: user.username,
                password: hash,
            })
            .returning(user_id)
            .get_result::<uuid::Uuid>(self)
            .map_err(|err| match err {
                DieselError::DatabaseError(..) => UserAlreadyExistsOrRequestInvalid,
                _ => crate::errors::UserError::InternalError,
            })
    }

    fn login(&mut self, user: UserLogin) -> Result<uuid::Uuid, UserError> {
        use crate::schema::users::dsl::*;

        let db_user = users
            .filter(username.eq(user.username))
            .select((user_id, password))
            .first::<(uuid::Uuid, String)>(self)
            .map_err(|_| InvalidRequest)?;

        argon2::verify_encoded(db_user.1.as_str(), user.password.as_bytes())
            .map(|_| db_user.0)
            .map_err(|_| PermissionDenied)
    }

    fn profile(&mut self, id: uuid::Uuid) -> Result<User, UserError> {
        use crate::schema::users::dsl::*;

        users
            .filter(user_id.eq(id))
            .select(User::as_select())
            .get_result::<User>(self)
            .map_err(|_| UserError::InvalidRequest)
    }
}
