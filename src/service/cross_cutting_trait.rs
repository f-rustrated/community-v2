use async_trait::async_trait;
use crate::service::response::BaseError;

#[async_trait]
pub trait TransactionUnitOfWork {
    async fn begin(&mut self) -> Result<(), BaseError>;
    async fn commit(&mut self) -> Result<(), BaseError>;
    async fn rollback(&mut self) -> Result<(), BaseError>;
}