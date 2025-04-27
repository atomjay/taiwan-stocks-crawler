use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;
use bigdecimal::BigDecimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPrice {
    pub id: Uuid,
    pub stock_id: Uuid,
    pub date: Date,
    pub open: BigDecimal,
    pub high: BigDecimal,
    pub low: BigDecimal,
    pub close: BigDecimal,
    pub volume: u64,
    pub change: BigDecimal,           // 漲跌幅
    pub change_percent: BigDecimal,   // 漲跌百分比
    pub turnover: u64,         // 成交金額
    pub transactions: u64,     // 成交筆數
    pub pe_ratio: Option<BigDecimal>, // 本益比
    pub pb_ratio: Option<BigDecimal>, // 股價淨值比
    pub dividend_yield: Option<BigDecimal>, // 殖利率
    pub market_cap: Option<u64>,     // 市值
    pub foreign_buy: Option<i64>,    // 外資買賣超
    pub trust_buy: Option<i64>,      // 投信買賣超
    pub dealer_buy: Option<i64>,     // 自營商買賣超
}

impl Default for StockPrice {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            stock_id: Uuid::nil(),
            date: Date::from_calendar_date(2025, time::Month::January, 1).unwrap(),
            open: BigDecimal::from(0),
            high: BigDecimal::from(0),
            low: BigDecimal::from(0),
            close: BigDecimal::from(0),
            volume: 0,
            change: BigDecimal::from(0),
            change_percent: BigDecimal::from(0),
            turnover: 0,
            transactions: 0,
            pe_ratio: None,
            pb_ratio: None,
            dividend_yield: None,
            market_cap: None,
            foreign_buy: None,
            trust_buy: None,
            dealer_buy: None,
        }
    }
}

impl StockPrice {
    pub fn new(
        stock_id: Uuid,
        date: Date,
        open: BigDecimal,
        high: BigDecimal,
        low: BigDecimal,
        close: BigDecimal,
        volume: u64,
        turnover: u64,
        transactions: u64,
        pe_ratio: Option<BigDecimal>,
        pb_ratio: Option<BigDecimal>,
        dividend_yield: Option<BigDecimal>,
        market_cap: Option<u64>,
        foreign_buy: Option<i64>,
        trust_buy: Option<i64>,
        dealer_buy: Option<i64>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            stock_id,
            date,
            open,
            high,
            low,
            close,
            volume,
            change: BigDecimal::from(0),
            change_percent: BigDecimal::from(0),
            turnover,
            transactions,
            pe_ratio,
            pb_ratio,
            dividend_yield,
            market_cap,
            foreign_buy,
            trust_buy,
            dealer_buy,
        }
    }

    pub fn with_details(
        stock_id: Uuid,
        date: Date,
        open: BigDecimal,
        high: BigDecimal,
        low: BigDecimal,
        close: BigDecimal,
        volume: u64,
        change: BigDecimal,
        change_percent: BigDecimal,
        turnover: u64,
        transactions: u64,
        pe_ratio: Option<BigDecimal>,
        pb_ratio: Option<BigDecimal>,
        dividend_yield: Option<BigDecimal>,
        market_cap: Option<u64>,
        foreign_buy: Option<i64>,
        trust_buy: Option<i64>,
        dealer_buy: Option<i64>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            stock_id,
            date,
            open,
            high,
            low,
            close,
            volume,
            change,
            change_percent,
            turnover,
            transactions,
            pe_ratio,
            pb_ratio,
            dividend_yield,
            market_cap,
            foreign_buy,
            trust_buy,
            dealer_buy,
        }
    }

    pub fn calculate_change(&mut self, prev_close: BigDecimal) {
        if prev_close > BigDecimal::from(0) {
            self.change = self.close.clone() - prev_close.clone();
            self.change_percent = (self.change.clone() / prev_close) * BigDecimal::from(100);
        }
    }
}
