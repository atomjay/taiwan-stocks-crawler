use crate::domain::entities::StockPrice;
use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPriceDto {
    pub id: String,
    pub stock_id: String,
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub change: f64,
    pub change_percent: f64,
    pub turnover: u64,
    pub transactions: u64,
    pub pe_ratio: Option<f64>,
    pub pb_ratio: Option<f64>,
    pub dividend_yield: Option<f64>,
    pub market_cap: Option<u64>,
    pub foreign_buy: Option<i64>,
    pub trust_buy: Option<i64>,
    pub dealer_buy: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStockPriceDto {
    pub stock_id: String,
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub change: f64,
    pub change_percent: f64,
    pub turnover: u64,
    pub transactions: u64,
    pub pe_ratio: Option<f64>,
    pub pb_ratio: Option<f64>,
    pub dividend_yield: Option<f64>,
    pub market_cap: Option<u64>,
    pub foreign_buy: Option<i64>,
    pub trust_buy: Option<i64>,
    pub dealer_buy: Option<i64>,
}

impl From<StockPrice> for StockPriceDto {
    fn from(price: StockPrice) -> Self {
        Self {
            id: price.id.to_string(),
            stock_id: price.stock_id.to_string(),
            date: price.date.to_string(),
            open: price.open,
            high: price.high,
            low: price.low,
            close: price.close,
            volume: price.volume,
            change: price.change,
            change_percent: price.change_percent,
            turnover: price.turnover,
            transactions: price.transactions,
            pe_ratio: price.pe_ratio,
            pb_ratio: price.pb_ratio,
            dividend_yield: price.dividend_yield,
            market_cap: price.market_cap,
            foreign_buy: price.foreign_buy,
            trust_buy: price.trust_buy,
            dealer_buy: price.dealer_buy,
        }
    }
}

impl TryFrom<StockPriceDto> for StockPrice {
    type Error = anyhow::Error;

    fn try_from(dto: StockPriceDto) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&dto.id)?,
            stock_id: Uuid::parse_str(&dto.stock_id)?,
            date: Date::parse(&dto.date, &time::format_description::well_known::Iso8601::DATE)?,
            open: dto.open,
            high: dto.high,
            low: dto.low,
            close: dto.close,
            volume: dto.volume,
            change: dto.change,
            change_percent: dto.change_percent,
            turnover: dto.turnover,
            transactions: dto.transactions,
            pe_ratio: dto.pe_ratio,
            pb_ratio: dto.pb_ratio,
            dividend_yield: dto.dividend_yield,
            market_cap: dto.market_cap,
            foreign_buy: dto.foreign_buy,
            trust_buy: dto.trust_buy,
            dealer_buy: dto.dealer_buy,
        })
    }
}
