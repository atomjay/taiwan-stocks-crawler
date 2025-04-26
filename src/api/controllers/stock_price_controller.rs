use crate::application::dtos::{CreateStockPriceDto, StockPriceDto};
use crate::application::services::StockPriceService;
use crate::domain::value_objects::Result as DomainResult;
use std::sync::Arc;
use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    Json,
};
use crate::AppState;
use serde::Deserialize;

#[derive(Clone)]
pub struct StockPriceController {
    stock_price_service: Arc<StockPriceService>,
}

impl StockPriceController {
    pub fn new(stock_price_service: Arc<StockPriceService>) -> Self {
        Self { stock_price_service }
    }

    pub async fn create_stock_price(&self, dto: CreateStockPriceDto) -> DomainResult<StockPriceDto> {
        self.stock_price_service.create_stock_price(dto).await
    }

    pub async fn get_stock_price_by_id(&self, id: &str) -> DomainResult<Option<StockPriceDto>> {
        self.stock_price_service.get_stock_price_by_id(id).await
    }

    pub async fn get_stock_prices_by_stock_id(
        &self,
        stock_id: &str,
        start_date: Option<String>,
        end_date: Option<String>,
    ) -> DomainResult<Vec<StockPriceDto>> {
        self.stock_price_service
            .get_stock_prices_by_stock_id(stock_id, start_date, end_date)
            .await
    }

    pub async fn get_latest_price_by_stock_id(&self, stock_id: &str) -> DomainResult<Option<StockPriceDto>> {
        self.stock_price_service.get_latest_price_by_stock_id(stock_id).await
    }
}

// 查詢參數結構體
#[derive(Deserialize)]
pub struct StockPriceQuery {
    start_date: Option<String>,
    end_date: Option<String>,
}

// Axum 路由處理器
pub async fn get_stock_prices_by_stock_id(
    State(state): State<AppState>,
    Path(code): Path<String>,
    Query(query): Query<StockPriceQuery>,
) -> std::result::Result<Json<Vec<StockPriceDto>>, StatusCode> {
    match state.price_controller.get_stock_prices_by_stock_id(
        &code,
        query.start_date,
        query.end_date,
    ).await {
        Ok(prices) => Ok(Json(prices)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
