use crate::application::dtos::{CreateStockDto, StockDto};
use crate::application::services::StockService;
use crate::domain::value_objects::Result;
use std::sync::Arc;

#[derive(Clone)]
pub struct StockController {
    stock_service: Arc<StockService>,
}

impl StockController {
    pub fn new(stock_service: Arc<StockService>) -> Self {
        Self { stock_service }
    }

    pub async fn create_stock(&self, dto: CreateStockDto) -> Result<StockDto> {
        self.stock_service.create_stock(dto).await
    }

    pub async fn get_stock_by_id(&self, id: &str) -> Result<Option<StockDto>> {
        self.stock_service.get_stock_by_id(id).await
    }

    pub async fn get_stock_by_code(&self, code: &str) -> Result<Option<StockDto>> {
        self.stock_service.get_stock_by_code(code).await
    }

    pub async fn get_all_stocks(&self) -> Result<Vec<StockDto>> {
        self.stock_service.get_all_stocks().await
    }

    pub async fn delete_stock(&self, id: &str) -> Result<()> {
        self.stock_service.delete_stock(id).await
    }
}
