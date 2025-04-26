use crate::application::dtos::{CreateStockDto, CreateStockPriceDto};
use crate::presentation::controllers::{StockController, StockPriceController};
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, delete},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct DateRangeQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

pub fn create_api_router(
    stock_controller: Arc<StockController>,
    stock_price_controller: Arc<StockPriceController>,
) -> Router {
    Router::new()
        .nest("/api/stocks", stock_routes(stock_controller))
        .nest("/api/stock-prices", stock_price_routes(stock_price_controller))
}

fn stock_routes(stock_controller: Arc<StockController>) -> Router {
    Router::new()
        .route("/", get(get_all_stocks))
        .route("/", post(create_stock))
        .route("/:id", get(get_stock_by_id))
        .route("/code/:code", get(get_stock_by_code))
        .route("/:id", delete(delete_stock))
        .layer(Extension(stock_controller))
}

fn stock_price_routes(stock_price_controller: Arc<StockPriceController>) -> Router {
    Router::new()
        .route("/:id", get(get_stock_price_by_id))
        .route("/stock/:stock_id", get(get_stock_prices_by_stock_id))
        .route("/stock/:stock_id/latest", get(get_latest_price_by_stock_id))
        .route("/", post(create_stock_price))
        .layer(Extension(stock_price_controller))
}

async fn get_all_stocks(
    Extension(controller): Extension<Arc<StockController>>,
) -> impl IntoResponse {
    match controller.get_all_stocks().await {
        Ok(stocks) => (StatusCode::OK, Json(stocks)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get stocks: {}", e),
        )
            .into_response(),
    }
}

async fn create_stock(
    Extension(controller): Extension<Arc<StockController>>,
    Json(dto): Json<CreateStockDto>,
) -> impl IntoResponse {
    match controller.create_stock(dto).await {
        Ok(stock) => (StatusCode::CREATED, Json(stock)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create stock: {}", e),
        )
            .into_response(),
    }
}

async fn get_stock_by_id(
    Path(id): Path<String>,
    Extension(controller): Extension<Arc<StockController>>,
) -> impl IntoResponse {
    match controller.get_stock_by_id(&id).await {
        Ok(Some(stock)) => (StatusCode::OK, Json(stock)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Stock not found").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get stock: {}", e),
        )
            .into_response(),
    }
}

async fn get_stock_by_code(
    Path(code): Path<String>,
    Extension(controller): Extension<Arc<StockController>>,
) -> impl IntoResponse {
    match controller.get_stock_by_code(&code).await {
        Ok(Some(stock)) => (StatusCode::OK, Json(stock)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Stock not found").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get stock: {}", e),
        )
            .into_response(),
    }
}

async fn delete_stock(
    Path(id): Path<String>,
    Extension(controller): Extension<Arc<StockController>>,
) -> impl IntoResponse {
    match controller.delete_stock(&id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete stock: {}", e),
        )
            .into_response(),
    }
}

async fn get_stock_price_by_id(
    Path(id): Path<String>,
    Extension(controller): Extension<Arc<StockPriceController>>,
) -> impl IntoResponse {
    match controller.get_stock_price_by_id(&id).await {
        Ok(Some(price)) => (StatusCode::OK, Json(price)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Stock price not found").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get stock price: {}", e),
        )
            .into_response(),
    }
}

async fn get_stock_prices_by_stock_id(
    Path(stock_id): Path<String>,
    Query(query): Query<DateRangeQuery>,
    Extension(controller): Extension<Arc<StockPriceController>>,
) -> impl IntoResponse {
    match controller
        .get_stock_prices_by_stock_id(&stock_id, query.start_date, query.end_date)
        .await
    {
        Ok(prices) => (StatusCode::OK, Json(prices)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get stock prices: {}", e),
        )
            .into_response(),
    }
}

async fn get_latest_price_by_stock_id(
    Path(stock_id): Path<String>,
    Extension(controller): Extension<Arc<StockPriceController>>,
) -> impl IntoResponse {
    match controller.get_latest_price_by_stock_id(&stock_id).await {
        Ok(Some(price)) => (StatusCode::OK, Json(price)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Stock price not found").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get latest stock price: {}", e),
        )
            .into_response(),
    }
}

async fn create_stock_price(
    Extension(controller): Extension<Arc<StockPriceController>>,
    Json(dto): Json<CreateStockPriceDto>,
) -> impl IntoResponse {
    match controller.create_stock_price(dto).await {
        Ok(price) => (StatusCode::CREATED, Json(price)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create stock price: {}", e),
        )
            .into_response(),
    }
}
