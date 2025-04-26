use crate::application::dtos::{CreateStockDto, StockDto};
use crate::domain::models::Stock;
use crate::domain::repositories::StockRepository;
use crate::domain::value_objects::Result;
use std::sync::Arc;
use uuid::Uuid;

pub struct StockService {
    stock_repository: Arc<dyn StockRepository>,
}

impl StockService {
    pub fn new(stock_repository: Arc<dyn StockRepository>) -> Self {
        Self { stock_repository }
    }

    pub async fn create_stock(&self, dto: CreateStockDto) -> Result<StockDto> {
        let stock = Stock::new(dto.code, dto.name);
        self.stock_repository.save(&stock).await?;
        Ok(StockDto::from(stock))
    }

    pub async fn get_stock_by_id(&self, id: &str) -> Result<Option<StockDto>> {
        let uuid = Uuid::parse_str(id)?;
        let stock = self.stock_repository.find_by_id(&uuid).await?;
        Ok(stock.map(StockDto::from))
    }

    pub async fn get_stock_by_code(&self, code: &str) -> Result<Option<StockDto>> {
        let stock = self.stock_repository.find_by_code(code).await?;
        Ok(stock.map(StockDto::from))
    }

    pub async fn get_all_stocks(&self) -> Result<Vec<StockDto>> {
        let stocks = self.stock_repository.find_all().await?;
        Ok(stocks.into_iter().map(StockDto::from).collect())
    }

    pub async fn delete_stock(&self, id: &str) -> Result<()> {
        let uuid = Uuid::parse_str(id)?;
        self.stock_repository.delete(&uuid).await
    }
}
