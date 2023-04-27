use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

use crate::{
    errors::UserError,
    models::{NewUser, User, UserLogin},
    DbPool,
};

pub struct Database {
    pub db: DbPool,
}
impl Database {
    pub fn get(&self) -> Result<Box<dyn Controller>, UserError> {
        match self.db.get() {
            Ok(res) => Ok(Box::new(res)),
            Err(..) => Err(UserError::InternalError),
        }
    }
}

pub trait Statistics {
    fn get_total_users(&mut self) -> Result<i64, UserError>;
}
pub trait Users {
    fn register(&mut self, user: NewUser) -> Result<uuid::Uuid, UserError>;
    fn login(&mut self, user: UserLogin) -> Result<uuid::Uuid, UserError>;
    fn profile(&mut self, id: uuid::Uuid) -> Result<User, UserError>;
}

pub trait Controller: Statistics + Users {}

impl Controller for PooledConnection<ConnectionManager<PgConnection>> {}
