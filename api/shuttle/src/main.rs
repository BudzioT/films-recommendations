use axum::{routing::get, Router};
use shuttle_runtime::CustomError;
use sqlx::Executor;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool
) -> shuttle_axum::ShuttleAxum {
    // Execute schema sql file
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    // Root router
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
