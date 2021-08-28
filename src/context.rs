use crate::Result;
use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, Pool, Postgres};
use std::{env, sync::Arc};

pub type DbConnection = PoolConnection<Postgres>;

#[derive(Clone, Debug)]
pub struct Context {
    db_pool: Arc<Pool<Postgres>>,
}

impl Context {
    pub async fn new() -> Result<Self> {
        let db_pool = Arc::new(
            PgPoolOptions::new()
                .max_connections(5)
                .connect(&env::var("DATABASE_URL")?)
                .await?,
        );

        Ok(Context { db_pool })
    }

    pub async fn connection(&self) -> Result<DbConnection> {
        Ok(self.db_pool.acquire().await?)
    }
}

impl juniper::Context for Context {}
