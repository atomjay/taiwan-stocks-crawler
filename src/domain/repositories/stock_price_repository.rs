use crate::domain::models::StockPrice;
use crate::domain::value_objects::Result;
use async_trait::async_trait;
use time::Date;
use uuid::Uuid;

#[async_trait]
pub trait StockPriceRepository: Send + Sync {
    async fn create(&self, stock_price: &StockPrice) -> Result<()>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<StockPrice>>;
    async fn find_by_stock_id(&self, stock_id: &Uuid) -> Result<Vec<StockPrice>>;
    async fn find_by_stock_id_and_date_range(
        &self,
        stock_id: &Uuid,
        start_date: Option<Date>,
        end_date: Option<Date>,
    ) -> Result<Vec<StockPrice>>;
    async fn find_latest_by_stock_id(&self, stock_id: &Uuid) -> Result<Option<StockPrice>>;
}
