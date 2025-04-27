use crate::domain::models::StockPrice;
use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;
use bigdecimal::BigDecimal;
use std::str::FromStr;

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
            open: price.open.to_string().parse::<f64>().unwrap_or(0.0),
            high: price.high.to_string().parse::<f64>().unwrap_or(0.0),
            low: price.low.to_string().parse::<f64>().unwrap_or(0.0),
            close: price.close.to_string().parse::<f64>().unwrap_or(0.0),
            volume: price.volume,
            change: price.change.to_string().parse::<f64>().unwrap_or(0.0),
            change_percent: price.change_percent.to_string().parse::<f64>().unwrap_or(0.0),
            turnover: price.turnover,
            transactions: price.transactions,
            pe_ratio: price.pe_ratio.map(|v| v.to_string().parse::<f64>().unwrap_or(0.0)),
            pb_ratio: price.pb_ratio.map(|v| v.to_string().parse::<f64>().unwrap_or(0.0)),
            dividend_yield: price.dividend_yield.map(|v| v.to_string().parse::<f64>().unwrap_or(0.0)),
            market_cap: price.market_cap,
            foreign_buy: price.foreign_buy,
            trust_buy: price.trust_buy,
            dealer_buy: price.dealer_buy,
        }
    }
}

impl From<StockPrice> for CreateStockPriceDto {
    fn from(price: StockPrice) -> Self {
        Self {
            stock_id: price.stock_id.to_string(),
            date: price.date.to_string(),
            open: price.open.to_string().parse::<f64>().unwrap_or(0.0),
            high: price.high.to_string().parse::<f64>().unwrap_or(0.0),
            low: price.low.to_string().parse::<f64>().unwrap_or(0.0),
            close: price.close.to_string().parse::<f64>().unwrap_or(0.0),
            volume: price.volume,
            change: price.change.to_string().parse::<f64>().unwrap_or(0.0),
            change_percent: price.change_percent.to_string().parse::<f64>().unwrap_or(0.0),
            turnover: price.turnover,
            transactions: price.transactions,
            pe_ratio: price.pe_ratio.map(|v| v.to_string().parse::<f64>().unwrap_or(0.0)),
            pb_ratio: price.pb_ratio.map(|v| v.to_string().parse::<f64>().unwrap_or(0.0)),
            dividend_yield: price.dividend_yield.map(|v| v.to_string().parse::<f64>().unwrap_or(0.0)),
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
        let id = Uuid::parse_str(&dto.id)?;
        let stock_id = Uuid::parse_str(&dto.stock_id)?;
        let date = Date::parse(&dto.date, &time::format_description::well_known::Iso8601::DATE)?;
        
        // 將 f64 轉換為 BigDecimal
        let open = BigDecimal::from_str(&dto.open.to_string())?;
        let high = BigDecimal::from_str(&dto.high.to_string())?;
        let low = BigDecimal::from_str(&dto.low.to_string())?;
        let close = BigDecimal::from_str(&dto.close.to_string())?;
        let change = BigDecimal::from_str(&dto.change.to_string())?;
        let change_percent = BigDecimal::from_str(&dto.change_percent.to_string())?;
        
        // 轉換可選值
        let pe_ratio = dto.pe_ratio.map(|v| BigDecimal::from_str(&v.to_string()).unwrap_or_else(|_| BigDecimal::from(0)));
        let pb_ratio = dto.pb_ratio.map(|v| BigDecimal::from_str(&v.to_string()).unwrap_or_else(|_| BigDecimal::from(0)));
        let dividend_yield = dto.dividend_yield.map(|v| BigDecimal::from_str(&v.to_string()).unwrap_or_else(|_| BigDecimal::from(0)));

        Ok(StockPrice {
            id,
            stock_id,
            date,
            open,
            high,
            low,
            close,
            volume: dto.volume,
            change,
            change_percent,
            turnover: dto.turnover,
            transactions: dto.transactions,
            pe_ratio,
            pb_ratio,
            dividend_yield,
            market_cap: dto.market_cap,
            foreign_buy: dto.foreign_buy,
            trust_buy: dto.trust_buy,
            dealer_buy: dto.dealer_buy,
        })
    }
}
