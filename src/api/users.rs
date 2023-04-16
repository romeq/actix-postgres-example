use crate::{database::db::Database, errors::*};
use actix_web::{
    get,
    web::{block, Data, Json},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct StatisticsResponse {
    pub user_count: u64,
}

#[get("/users")]
pub async fn statistics(db_pool: Data<Database>) -> Result<Json<StatisticsResponse>, UserError> {
    let result = block(move || db_pool.get()?.get_total_users()).await;

    if let Ok(Ok(user_count)) = result {
        Ok(Json(StatisticsResponse {
            user_count: user_count as u64,
        }))
    } else {
        Err(UserError::InternalError)
    }
}
