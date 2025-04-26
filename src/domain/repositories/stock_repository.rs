use crate::domain::models::Stock;
use crate::domain::value_objects::Result;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait StockRepository: Send + Sync {
    async fn save(&self, stock: &Stock) -> Result<()>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Stock>>;
    async fn find_by_code(&self, code: &str) -> Result<Option<Stock>>;
    async fn find_all(&self) -> Result<Vec<Stock>>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}
