# Actix-diesel template

Boilerplate project with [Actix](https://actix.rs) and [Diesel](https://diesel.rs) ORM. A starting point for your web application. 



## Usage

Refer to [Diesel documentation](https://docs.diesel.rs/2.0.x/diesel/index.html) on database specific problems and [Actix documentation](https://actix.rs/docs) on web-specific stuff.

Source code for route handlers live inside `src/api/`. To begin creating new routes, choose a filename for the route you're going to create. It's recommended that the feature it's going to implement can be guessed from the filename. 

 After creating a new file or appending an existing file with your handlers, make sure that the file is included in `mod.rs` for it to be compiled and that the route is enabled in `main.rs`.



### Migrations

For managing migrations use the `diesel-cli`. (`cargo install diesel_cli` and make sure `~/.cargo/bin/` is in your `$PATH`).

For more advanced help with `diesel-cli` [see it's GitHub repository](https://github.com/diesel-rs/diesel/tree/HEAD/diesel_cli).



#### Create new migration

```sh
diesel migration generate \
	--database-url "postgres://derive:cloud@localhost:6969/derive-cloud" \
	some_migration
```



### Database-integrated routes

For every route an `actix_web::web::Data<crate::database::db::Database>` is given as an argument. To use it, you should be able to simply mention it as an argument in your function declaration as in `pub async fn statistics(db: Data<Database>)`.

The usage can be found from `src/database/db.rs`; basically you get a Database `struct`, which implements a `get`-method, which returns you a pooled database connection that works as the controller.

To create new functions for your controller, create a trait including your functions for your feature in `src/database/db.rs` and add the trait as an implementation requirement for `Controller`-trait. After that, create a file under `src/database`  that implements that trait. An implementation can look like the following code snippet.

```rust
/* ------------ src/database/db.rs ------------ */
//  .........
pub trait MyFeatureTrait {
    fn some_feature(&mut self) -> Result<SomeResult, UserError>;
}

pub trait Controller: SomeOtherFeatureTrait + MyFeatureTrait {}
// 	.........

/* ------------ src/database/myfeature.rs ------------ */
use crate::errors::UserError;

use super::db::MyFeatureTrait;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    QueryDsl, RunQueryDsl,
};

impl MyFeatureTrait for PooledConnection<ConnectionManager<PgConnection>> {
    fn some_feature(&mut self) -> Result<i64, UserError> {
        use crate::schema::users::dsl::*;
        users
            .count()
            .get_result::<i64>(self)
            .map_err(|_| UserError::InternalError)
    }
}
```

After implementation you need to make sure the file is included in `mod.rs`.  If everything went correctly, you should be able to call the function from your route with `database.get()?.my_method()`. 

You could use the database method by for example so:

```rust
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

#[get("/users")]
pub async fn statistics(db: Data<Database>) -> Result<Json<StatisticsResponse>, UserError> {
    let result = block(move || db.get()?.some_feature()).await??;

    Ok(Json(StatisticsResponse {
        user_count: result,
    }))
}
```



### Errors

The `src/database/*.rs` and `src/database/*.rs` should both return an `crate::errors::UserError` as the error value. This requirement makes them work seamlessly with each other, and you have to write less code on API handlers as you don't need to cast them to other error type first.

To change the way errors are returned you can change the implementation in `src/errors.rs ` at `impl ResponseError for UserError` -> `fn error_response(&self) -> HttpResponse` . 



### Database setup

The database is ran under docker and credentials are stored in `docker-compose-dev.yml` (at `POSTGRES_USER` and `POSTGRES_PASSWORD` environment variables). If you choose to change them you also need to change them to `setup.sh`. 

After database is setup you can run the project with release flags.

```sh
chmod +x setup.sh && ./setup.sh && cargo run --release
```



## License

MIT