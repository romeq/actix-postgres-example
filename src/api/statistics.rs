use crate::{database::db::Database, errors::*};
use actix_web::{
    get,
    web::{block, Data, Json},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct StatisticsResponse {
    pub user_count: i64,
}

#[get("/")]
pub async fn statistics(db: Data<Database>) -> Result<Json<StatisticsResponse>, UserError> {
    let result = block(move || db.get()?.get_total_users()).await??;

    Ok(Json(StatisticsResponse {
        user_count: result as i64,
    }))
}
