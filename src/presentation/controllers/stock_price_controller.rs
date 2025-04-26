use crate::application::dtos::{CreateStockPriceDto, StockPriceDto};
use crate::application::services::StockPriceService;
use crate::domain::value_objects::Result;
use std::sync::Arc;

#[derive(Clone)]
pub struct StockPriceController {
    stock_price_service: Arc<StockPriceService>,
}

impl StockPriceController {
    pub fn new(stock_price_service: Arc<StockPriceService>) -> Self {
        Self { stock_price_service }
    }

    pub async fn create_stock_price(&self, dto: CreateStockPriceDto) -> Result<StockPriceDto> {
        self.stock_price_service.create_stock_price(dto).await
    }

    pub async fn get_stock_price_by_id(&self, id: &str) -> Result<Option<StockPriceDto>> {
        self.stock_price_service.get_stock_price_by_id(id).await
    }

    pub async fn get_stock_prices_by_stock_id(
        &self,
        stock_id: &str,
        start_date: Option<String>,
        end_date: Option<String>,
    ) -> Result<Vec<StockPriceDto>> {
        self.stock_price_service
            .get_stock_prices_by_stock_id(stock_id, start_date, end_date)
            .await
    }

    pub async fn get_latest_price_by_stock_id(&self, stock_id: &str) -> Result<Option<StockPriceDto>> {
        self.stock_price_service
            .get_latest_price_by_stock_id(stock_id)
            .await
    }
}
