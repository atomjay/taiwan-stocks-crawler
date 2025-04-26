use crate::application::dtos::{CreateStockPriceDto, StockPriceDto};
use crate::domain::models::StockPrice;
use crate::domain::repositories::StockPriceRepository;
use crate::domain::value_objects::Result;
use std::sync::Arc;
use time::Date;
use uuid::Uuid;

pub struct StockPriceService {
    stock_price_repository: Arc<dyn StockPriceRepository>,
}

impl StockPriceService {
    pub fn new(stock_price_repository: Arc<dyn StockPriceRepository>) -> Self {
        Self {
            stock_price_repository,
        }
    }

    pub async fn create_stock_price(&self, dto: CreateStockPriceDto) -> Result<StockPriceDto> {
        let stock_id = Uuid::parse_str(&dto.stock_id)?;
        let date = Date::parse(&dto.date, &time::format_description::well_known::Iso8601::DATE)?;

        let stock_price = StockPrice::new(
            stock_id,
            date,
            dto.open,
            dto.high,
            dto.low,
            dto.close,
            dto.volume,
            dto.turnover,
            dto.transactions,
            dto.pe_ratio,
            dto.pb_ratio,
            dto.dividend_yield,
            dto.market_cap,
            dto.foreign_buy,
            dto.trust_buy,
            dto.dealer_buy,
        );
        
        // 設置變化值
        let stock_price = if dto.change != 0.0 || dto.change_percent != 0.0 {
            let mut sp = stock_price;
            sp.change = dto.change;
            sp.change_percent = dto.change_percent;
            sp
        } else {
            stock_price
        };

        self.stock_price_repository.create(&stock_price).await?;
        Ok(StockPriceDto::from(stock_price))
    }

    pub async fn get_stock_price_by_id(&self, id: &str) -> Result<Option<StockPriceDto>> {
        let uuid = Uuid::parse_str(id)?;
        let stock_price = self.stock_price_repository.find_by_id(&uuid).await?;
        Ok(stock_price.map(StockPriceDto::from))
    }

    pub async fn get_stock_prices_by_stock_id(
        &self,
        stock_id: &str,
        start_date_str: Option<String>,
        end_date_str: Option<String>,
    ) -> Result<Vec<StockPriceDto>> {
        let stock_id = Uuid::parse_str(stock_id)?;
        
        let start_date = if let Some(date_str) = start_date_str {
            Some(Date::parse(&date_str, &time::format_description::well_known::Iso8601::DATE)?)
        } else {
            None
        };
        
        let end_date = if let Some(date_str) = end_date_str {
            Some(Date::parse(&date_str, &time::format_description::well_known::Iso8601::DATE)?)
        } else {
            None
        };

        let stock_prices = self
            .stock_price_repository
            .find_by_stock_id_and_date_range(&stock_id, start_date, end_date)
            .await?;

        Ok(stock_prices.into_iter().map(StockPriceDto::from).collect())
    }

    pub async fn get_latest_price_by_stock_id(&self, stock_id: &str) -> Result<Option<StockPriceDto>> {
        let stock_id = Uuid::parse_str(stock_id)?;
        let stock_price = self
            .stock_price_repository
            .find_latest_by_stock_id(&stock_id)
            .await?;

        Ok(stock_price.map(StockPriceDto::from))
    }
}
