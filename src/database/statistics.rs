use crate::errors::UserError;

use super::db::Statistics;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    QueryDsl, RunQueryDsl,
};

impl Statistics for PooledConnection<ConnectionManager<PgConnection>> {
    fn get_total_users(&mut self) -> Result<i64, UserError> {
        use crate::schema::users::dsl::*;
        users
            .count()
            .first::<i64>(self)
            .map_err(|_| UserError::InternalError)
    }
}
