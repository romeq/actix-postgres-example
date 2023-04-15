use crate::database::users::*;
use actix_web::{get, web::{Data, block}, Responder};
use crate::DbPool;

#[get("/")]
pub async fn statistics(db_pool: Data<DbPool>) -> impl Responder {
    let user_num: i64 = match block(move || {
        let mut f = db_pool.get().unwrap();
        get_number_of_users(&mut f).unwrap()
    }).await {
        Ok(result) => result,
        Err(err) => {
            println!("error: {}", err);
            0
        },
    };

    user_num.to_string()
}
