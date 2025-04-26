use crate::application::dtos::{CreateStockDto, StockDto};
use crate::application::services::StockService;
use crate::domain::value_objects::Result as DomainResult;
use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::AppState;

#[derive(Clone)]
pub struct StockController {
    stock_service: Arc<StockService>,
}

impl StockController {
    pub fn new(stock_service: Arc<StockService>) -> Self {
        Self { stock_service }
    }

    pub async fn create_stock(&self, dto: CreateStockDto) -> DomainResult<StockDto> {
        self.stock_service.create_stock(dto).await
    }

    pub async fn get_stock_by_id(&self, id: &str) -> DomainResult<Option<StockDto>> {
        self.stock_service.get_stock_by_id(id).await
    }

    pub async fn get_stock_by_code(&self, code: &str) -> DomainResult<Option<StockDto>> {
        self.stock_service.get_stock_by_code(code).await
    }

    pub async fn get_all_stocks(&self) -> DomainResult<Vec<StockDto>> {
        self.stock_service.get_all_stocks().await
    }

    pub async fn delete_stock(&self, id: &str) -> DomainResult<()> {
        self.stock_service.delete_stock(id).await
    }
}

// Axum 路由處理器
pub async fn get_all_stocks(
    State(state): State<AppState>,
) -> std::result::Result<Json<Vec<StockDto>>, StatusCode> {
    match state.stock_controller.get_all_stocks().await {
        Ok(stocks) => Ok(Json(stocks)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_stock_by_code(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> std::result::Result<Json<Option<StockDto>>, StatusCode> {
    match state.stock_controller.get_stock_by_code(&code).await {
        Ok(stock) => Ok(Json(stock)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
