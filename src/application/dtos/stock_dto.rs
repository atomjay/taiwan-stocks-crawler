use crate::domain::models::Stock;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockDto {
    pub id: String,
    pub code: String,
    pub name: String,
    pub last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStockDto {
    pub code: String,
    pub name: String,
}

impl From<Stock> for StockDto {
    fn from(stock: Stock) -> Self {
        Self {
            id: stock.id.to_string(),
            code: stock.code,
            name: stock.name,
            last_updated: stock.last_updated.to_string(),
        }
    }
}

impl TryFrom<StockDto> for Stock {
    type Error = anyhow::Error;

    fn try_from(dto: StockDto) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&dto.id)?,
            code: dto.code,
            name: dto.name,
            last_updated: OffsetDateTime::parse(&dto.last_updated, &time::format_description::well_known::Rfc3339)?,
        })
    }
}
