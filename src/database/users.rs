use diesel::{PgConnection, RunQueryDsl, prelude::*};

pub fn get_number_of_users(conn: &mut PgConnection) -> Result<i64, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    users.select(diesel::dsl::count(username)).first::<i64>(conn)
}
