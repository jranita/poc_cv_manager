use once_cell::sync::OnceCell;
use sqlx::postgres::PgPool;
use std::{env, fmt};

static PG: OnceCell<PgPool> = OnceCell::new();

pub async fn create_pg_pool() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not available");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool.");

    PG.set(pool).expect("Postgresql pool must be set")
}

#[inline]
pub fn get_postgres() -> &'static PgPool {
    // Safety: tt is already set when the program is initialized
    unsafe { PG.get_unchecked() }
}