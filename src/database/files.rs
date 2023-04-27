use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection, RunQueryDsl,
};

use super::db::Files;
use crate::errors::UserError;
use serde::Deserialize;

#[derive(Deserialize, diesel::prelude::Insertable)]
#[diesel(table_name = crate::schema::files)]
pub struct File {
    pub owner_id: uuid::Uuid,
    pub original_filename: String,
    pub was_encrypted: bool,
    pub uploaded_at: Option<std::time::SystemTime>,
}

impl Files for PooledConnection<ConnectionManager<PgConnection>> {
    fn create_file(&mut self, file: File) -> Result<(), UserError> {
        use crate::schema::files::dsl::*;
        diesel::insert_into(files)
            .values(file)
            .execute(self)
            .map(|_| ())
            .map_err(|_| UserError::InternalError)
    }
}
