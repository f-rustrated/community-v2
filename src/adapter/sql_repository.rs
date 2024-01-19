use std::sync::OnceLock;

use async_trait::async_trait;
use sqlx::{PgConnection, PgPool, Postgres, Transaction};
use sqlx::postgres::PgPoolOptions;

use crate::config::config;
use crate::service::cross_cutting_trait::TransactionUnitOfWork;
use crate::service::response::BaseError;

pub struct SqlRepository {
    pub(crate) pool: &'static PgPool,
    pub(crate) transaction: Option<Transaction<'static, Postgres>>,
}

impl SqlRepository {
    pub async fn new() -> Self {
        Self {
            pool: pool().await,
            transaction: Default::default(),
        }
    }
}

impl SqlRepository {
    pub fn transaction(&mut self) -> Result<&mut PgConnection, BaseError> {
        match self.transaction.as_mut() {
            Some(trx) => Ok(trx),
            None => {
                tracing::error!("Transaction has not begun!");
                Err(BaseError::TransactionError)
            }
        }
    }
}

#[async_trait]
impl TransactionUnitOfWork for SqlRepository {
    async fn begin(&mut self) -> Result<(), BaseError> {
        match self.transaction {
            None => {
                self.transaction = Some(self.pool.begin().await?);
            }
            Some(_) => Err(BaseError::TransactionError)?
        }
        Ok(())
    }

    async fn commit(&mut self) -> Result<(), BaseError> {
        match self.transaction.take() {
            Some(trx) => trx.commit().await.map_err(|err| {
                tracing::error!("Transaction begun but failed to commit: {}", err);
                BaseError::TransactionError
            }),
            None => {
                tracing::error!("Transaction hasn't begun!");
                Err(BaseError::TransactionError)
            }
        }
    }

    async fn rollback(&mut self) -> Result<(), BaseError> {
        match self.transaction.take() {
            Some(trx) => trx.rollback().await.map_err(|err| {
                tracing::error!("Transaction begun but failed to rollback: {}", err);
                BaseError::TransactionError
            }),
            None => {
                tracing::error!("Transaction hasn't begun!");
                Err(BaseError::TransactionError)
            }
        }
    }
}

pub async fn pool() -> &'static PgPool {
    static SQLX_CONNECTION_POOL: OnceLock<PgPool> = OnceLock::new();
    if SQLX_CONNECTION_POOL.get().is_none() {
        let url = config().postgresql_url.to_owned();
        let _ = SQLX_CONNECTION_POOL.set(
            PgPoolOptions::new()
                .max_connections(5)
                .connect(&url)
                .await
                .unwrap()
        );
    }
    SQLX_CONNECTION_POOL.get().unwrap()
}

impl From<sqlx::Error> for BaseError {
    fn from(value: sqlx::Error) -> Self {
        tracing::error!("Database Error! {}", value);
        Self::DatabaseError
    }
}