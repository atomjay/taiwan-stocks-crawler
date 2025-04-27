use crate::domain::models::StockPrice;
use crate::domain::repositories::StockPriceRepository;
use crate::domain::value_objects::Result;
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use time::Date;
use uuid::Uuid;
use bigdecimal::BigDecimal;
use std::str::FromStr;

pub struct PostgresStockPriceRepository {
    pool: PgPool,
}

impl PostgresStockPriceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // 安全地將 i64 轉換為 u64
    fn i64_to_u64(value: i64) -> u64 {
        if value < 0 {
            0
        } else {
            value as u64
        }
    }
}

#[async_trait]
impl StockPriceRepository for PostgresStockPriceRepository {
    async fn create(&self, stock_price: &StockPrice) -> Result<()> {
        let query = format!(
            r#"
            INSERT INTO stock_prices (
                id, stock_id, date, open, high, low, close, volume, 
                change, change_percent, turnover, transactions, 
                pe_ratio, pb_ratio, dividend_yield, market_cap,
                foreign_buy, trust_buy, dealer_buy
            ) VALUES (
                $1, $2, $3, $4::numeric, $5::numeric, $6::numeric, $7::numeric, $8::numeric, 
                $9::numeric, $10::numeric, $11::numeric, $12::numeric, 
                $13::numeric, $14::numeric, $15::numeric, $16::numeric,
                $17::numeric, $18::numeric, $19::numeric
            )
            ON CONFLICT (stock_id, date) 
            DO UPDATE SET
                open = $4::numeric,
                high = $5::numeric,
                low = $6::numeric,
                close = $7::numeric,
                volume = $8::numeric,
                change = $9::numeric,
                change_percent = $10::numeric,
                turnover = $11::numeric,
                transactions = $12::numeric,
                pe_ratio = $13::numeric,
                pb_ratio = $14::numeric,
                dividend_yield = $15::numeric,
                market_cap = $16::numeric,
                foreign_buy = $17::numeric,
                trust_buy = $18::numeric,
                dealer_buy = $19::numeric
            "#
        );

        let open_str = stock_price.open.to_string();
        let high_str = stock_price.high.to_string();
        let low_str = stock_price.low.to_string();
        let close_str = stock_price.close.to_string();
        let volume_str = stock_price.volume.to_string();
        let change_str = stock_price.change.to_string();
        let change_percent_str = stock_price.change_percent.to_string();
        let turnover_str = stock_price.turnover.to_string();
        let transactions_str = stock_price.transactions.to_string();
        let pe_ratio_str = stock_price.pe_ratio.as_ref().map(|v| v.to_string());
        let pb_ratio_str = stock_price.pb_ratio.as_ref().map(|v| v.to_string());
        let dividend_yield_str = stock_price.dividend_yield.as_ref().map(|v| v.to_string());
        let market_cap_str = stock_price.market_cap.map(|v| v.to_string());
        let foreign_buy_str = stock_price.foreign_buy.map(|v| v.to_string());
        let trust_buy_str = stock_price.trust_buy.map(|v| v.to_string());
        let dealer_buy_str = stock_price.dealer_buy.map(|v| v.to_string());

        sqlx::query(&query)
            .bind(stock_price.id)
            .bind(stock_price.stock_id)
            .bind(stock_price.date)
            .bind(open_str)
            .bind(high_str)
            .bind(low_str)
            .bind(close_str)
            .bind(volume_str)
            .bind(change_str)
            .bind(change_percent_str)
            .bind(turnover_str)
            .bind(transactions_str)
            .bind(pe_ratio_str)
            .bind(pb_ratio_str)
            .bind(dividend_yield_str)
            .bind(market_cap_str)
            .bind(foreign_buy_str)
            .bind(trust_buy_str)
            .bind(dealer_buy_str)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<StockPrice>> {
        let query = format!(
            r#"
            SELECT 
                id, stock_id, date, 
                open::text as open, high::text as high, low::text as low, close::text as close,
                volume::text as volume, 
                change::text as change, change_percent::text as change_percent,
                turnover::text as turnover, transactions::text as transactions, 
                pe_ratio::text as pe_ratio, pb_ratio::text as pb_ratio, dividend_yield::text as dividend_yield,
                market_cap::text as market_cap,
                foreign_buy::text as foreign_buy, trust_buy::text as trust_buy, dealer_buy::text as dealer_buy
            FROM stock_prices
            WHERE id = $1
            "#
        );

        let row_result = sqlx::query(&query)
            .bind(id)
            .fetch_one(&self.pool)
            .await;

        match row_result {
            Ok(r) => {
                let id: Uuid = r.get("id");
                let stock_id: Uuid = r.get("stock_id");
                let date: Date = r.get("date");
                let open: String = r.get("open");
                let high: String = r.get("high");
                let low: String = r.get("low");
                let close: String = r.get("close");
                let volume: String = r.get("volume");
                let change: String = r.get("change");
                let change_percent: String = r.get("change_percent");
                let turnover: String = r.get("turnover");
                let transactions: String = r.get("transactions");
                let pe_ratio: Option<String> = r.get("pe_ratio");
                let pb_ratio: Option<String> = r.get("pb_ratio");
                let dividend_yield: Option<String> = r.get("dividend_yield");
                let market_cap: Option<String> = r.get("market_cap");
                let foreign_buy: Option<String> = r.get("foreign_buy");
                let trust_buy: Option<String> = r.get("trust_buy");
                let dealer_buy: Option<String> = r.get("dealer_buy");

                let stock_price = StockPrice {
                    id,
                    stock_id,
                    date,
                    open: BigDecimal::from_str(&open).unwrap_or_else(|_| BigDecimal::from(0)),
                    high: BigDecimal::from_str(&high).unwrap_or_else(|_| BigDecimal::from(0)),
                    low: BigDecimal::from_str(&low).unwrap_or_else(|_| BigDecimal::from(0)),
                    close: BigDecimal::from_str(&close).unwrap_or_else(|_| BigDecimal::from(0)),
                    volume: u64::from_str(&volume).unwrap_or(0),
                    change: BigDecimal::from_str(&change).unwrap_or_else(|_| BigDecimal::from(0)),
                    change_percent: BigDecimal::from_str(&change_percent).unwrap_or_else(|_| BigDecimal::from(0)),
                    turnover: u64::from_str(&turnover).unwrap_or(0),
                    transactions: u64::from_str(&transactions).unwrap_or(0),
                    pe_ratio: pe_ratio.map(|v| BigDecimal::from_str(&v).unwrap_or_else(|_| BigDecimal::from(0))),
                    pb_ratio: pb_ratio.map(|v| BigDecimal::from_str(&v).unwrap_or_else(|_| BigDecimal::from(0))),
                    dividend_yield: dividend_yield.map(|v| BigDecimal::from_str(&v).unwrap_or_else(|_| BigDecimal::from(0))),
                    market_cap: market_cap.map(|v| u64::from_str(&v).unwrap_or(0)),
                    foreign_buy: foreign_buy.map(|v| i64::from_str(&v).unwrap_or(0)),
                    trust_buy: trust_buy.map(|v| i64::from_str(&v).unwrap_or(0)),
                    dealer_buy: dealer_buy.map(|v| i64::from_str(&v).unwrap_or(0)),
                };
                Ok(Some(stock_price))
            }
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    async fn find_by_stock_id(&self, stock_id: &Uuid) -> Result<Vec<StockPrice>> {
        let query = format!(
            r#"
            SELECT 
                id, stock_id, date, 
                open::text as open, high::text as high, low::text as low, close::text as close,
                volume::text as volume, 
                change::text as change, change_percent::text as change_percent,
                turnover::text as turnover, transactions::text as transactions, 
                pe_ratio::text as pe_ratio, pb_ratio::text as pb_ratio, dividend_yield::text as dividend_yield,
                market_cap::text as market_cap,
                foreign_buy::text as foreign_buy, trust_buy::text as trust_buy, dealer_buy::text as dealer_buy
            FROM stock_prices
            WHERE stock_id = $1
            ORDER BY date DESC
            "#
        );

        let rows = match sqlx::query(&query)
            .bind(stock_id)
            .fetch_all(&self.pool)
            .await
        {
            Ok(rows) => rows,
            Err(e) => return Err(e.into()),
        };

        let mut stock_prices = Vec::new();
        for r in rows {
            let id: Uuid = r.get("id");
            let stock_id: Uuid = r.get("stock_id");
            let date: Date = r.get("date");
            let open: String = r.get("open");
            let high: String = r.get("high");
            let low: String = r.get("low");
            let close: String = r.get("close");
            let volume: String = r.get("volume");
            let change: String = r.get("change");
            let change_percent: String = r.get("change_percent");
            let turnover: String = r.get("turnover");
            let transactions: String = r.get("transactions");
            let pe_ratio: Option<String> = r.get("pe_ratio");
            let pb_ratio: Option<String> = r.get("pb_ratio");
            let dividend_yield: Option<String> = r.get("dividend_yield");
            let market_cap: Option<String> = r.get("market_cap");
            let foreign_buy: Option<String> = r.get("foreign_buy");
            let trust_buy: Option<String> = r.get("trust_buy");
            let dealer_buy: Option<String> = r.get("dealer_buy");

            let stock_price = StockPrice {
                id,
                stock_id,
                date,
                open: BigDecimal::from_str(&open).unwrap_or_else(|_| BigDecimal::from(0)),
                high: BigDecimal::from_str(&high).unwrap_or_else(|_| BigDecimal::from(0)),
                low: BigDecimal::from_str(&low).unwrap_or_else(|_| BigDecimal::from(0)),
                close: BigDecimal::from_str(&close).unwrap_or_else(|_| BigDecimal::from(0)),
                volume: u64::from_str(&volume).unwrap_or(0),
                change: BigDecimal::from_str(&change).unwrap_or_else(|_| BigDecimal::from(0)),
                change_percent: BigDecimal::from_str(&change_percent).unwrap_or_else(|_| BigDecimal::from(0)),
                turnover: u64::from_str(&turnover).unwrap_or(0),
                transactions: u64::from_str(&transactions).unwrap_or(0),
                pe_ratio: pe_ratio.map(|v| BigDecimal::from_str(&v).unwrap_or_else(|_| BigDecimal::from(0))),
                pb_ratio: pb_ratio.map(|v| BigDecimal::from_str(&v).unwrap_or_else(|_| BigDecimal::from(0))),
                dividend_yield: dividend_yield.map(|v| BigDecimal::from_str(&v).unwrap_or_else(|_| BigDecimal::from(0))),
                market_cap: market_cap.map(|v| u64::from_str(&v).unwrap_or(0)),
                foreign_buy: foreign_buy.map(|v| i64::from_str(&v).unwrap_or(0)),
                trust_buy: trust_buy.map(|v| i64::from_str(&v).unwrap_or(0)),
                dealer_buy: dealer_buy.map(|v| i64::from_str(&v).unwrap_or(0)),
            };
            stock_prices.push(stock_price);
        }

        Ok(stock_prices)
    }

    async fn find_by_stock_id_and_date_range(
        &self,
        stock_id: &Uuid,
        start_date: Option<Date>,
        end_date: Option<Date>,
    ) -> Result<Vec<StockPrice>> {
        let query = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                format!(
                    r#"
                    SELECT 
                        id, stock_id, date, 
                        open::text as open, high::text as high, low::text as low, close::text as close,
                        volume::text as volume, 
                        change::text as change, change_percent::text as change_percent,
                        turnover::text as turnover, transactions::text as transactions, 
                        pe_ratio::text as pe_ratio, pb_ratio::text as pb_ratio, dividend_yield::text as dividend_yield,
                        market_cap::text as market_cap,
                        foreign_buy::text as foreign_buy, trust_buy::text as trust_buy, dealer_buy::text as dealer_buy
                    FROM stock_prices
                    WHERE stock_id = $1 AND date BETWEEN $2 AND $3
                    ORDER BY date DESC
                    "#
                )
            }
            (Some(start), None) => {
                format!(
                    r#"
                    SELECT 
                        id, stock_id, date, 
                        open::text as open, high::text as high, low::text as low, close::text as close,
                        volume::text as volume, 
                        change::text as change, change_percent::text as change_percent,
                        turnover::text as turnover, transactions::text as transactions, 
                        pe_ratio::text as pe_ratio, pb_ratio::text as pb_ratio, dividend_yield::text as dividend_yield,
                        market_cap::text as market_cap,
                        foreign_buy::text as foreign_buy, trust_buy::text as trust_buy, dealer_buy::text as dealer_buy
                    FROM stock_prices
                    WHERE stock_id = $1 AND date >= $2
                    ORDER BY date DESC
                    "#
                )
            }
            (None, Some(end)) => {
                format!(
                    r#"
                    SELECT 
                        id, stock_id, date, 
                        open::text as open, high::text as high, low::text as low, close::text as close,
                        volume::text as volume, 
                        change::text as change, change_percent::text as change_percent,
                        turnover::text as turnover, transactions::text as transactions, 
                        pe_ratio::text as pe_ratio, pb_ratio::text as pb_ratio, dividend_yield::text as dividend_yield,
                        market_cap::text as market_cap,
                        foreign_buy::text as foreign_buy, trust_buy::text as trust_buy, dealer_buy::text as dealer_buy
                    FROM stock_prices
                    WHERE stock_id = $1 AND date <= $2
                    ORDER BY date DESC
                    "#
                )
            }
            (None, None) => {
                format!(
                    r#"
                    SELECT 
                        id, stock_id, date, 
                        open::text as open, high::text as high, low::text as low, close::text as close,
                        volume::text as volume, 
                        change::text as change, change_percent::text as change_percent,
                        turnover::text as turnover, transactions::text as transactions, 
                        pe_ratio::text as pe_ratio, pb_ratio::text as pb_ratio, dividend_yield::text as dividend_yield,
                        market_cap::text as market_cap,
                        foreign_buy::text as foreign_buy, trust_buy::text as trust_buy, dealer_buy::text as dealer_buy
                    FROM stock_prices
                    WHERE stock_id = $1
                    ORDER BY date DESC
                    "#
                )
            }
        };

        let mut query_builder = sqlx::query(&query).bind(stock_id);

        if let Some(start) = start_date {
            query_builder = query_builder.bind(start);
        }

        if let Some(end) = end_date {
            query_builder = query_builder.bind(end);
        }

        let rows = match query_builder.fetch_all(&self.pool).await {
            Ok(rows) => rows,
            Err(e) => return Err(e.into()),
        };

        let mut stock_prices = Vec::new();
        for r in rows {
            let id: Uuid = r.get("id");
            let stock_id: Uuid = r.get("stock_id");
            let date: Date = r.get("date");
            let open: String = r.get("open");
            let high: String = r.get("high");
            let low: String = r.get("low");
            let close: String = r.get("close");
            let volume: String = r.get("volume");
            let change: String = r.get("change");
            let change_percent: String = r.get("change_percent");
            let turnover: String = r.get("turnover");
            let transactions: String = r.get("transactions");
            let pe_ratio: Option<String> = r.get("pe_ratio");
            let pb_ratio: Option<String> = r.get("pb_ratio");
            let dividend_yield: Option<String> = r.get("dividend_yield");
            let market_cap: Option<String> = r.get("market_cap");
            let foreign_buy: Option<String> = r.get("foreign_buy");
            let trust_buy: Option<String> = r.get("trust_buy");
            let dealer_buy: Option<String> = r.get("dealer_buy");

            let stock_price = StockPrice {
                id,
                stock_id,
                date,
                open: BigDecimal::from_str(&open).unwrap_or_else(|_| BigDecimal::from(0)),
                high: BigDecimal::from_str(&high).unwrap_or_else(|_| BigDecimal::from(0)),
                low: BigDecimal::from_str(&low).unwrap_or_else(|_| BigDecimal::from(0)),
                close: BigDecimal::from_str(&close).unwrap_or_else(|_| BigDecimal::from(0)),
                volume: u64::from_str(&volume).unwrap_or(0),
                change: BigDecimal::from_str(&change).unwrap_or_else(|_| BigDecimal::from(0)),
                change_percent: BigDecimal::from_str(&change_percent).unwrap_or_else(|_| BigDecimal::from(0)),
                turnover: u64::from_str(&turnover).unwrap_or(0),
                transactions: u64::from_str(&transactions).unwrap_or(0),
                pe_ratio: pe_ratio.map(|v| BigDecimal::from_str(&v).unwrap_or_else(|_| BigDecimal::from(0))),
                pb_ratio: pb_ratio.map(|v| BigDecimal::from_str(&v).unwrap_or_else(|_| BigDecimal::from(0))),
                dividend_yield: dividend_yield.map(|v| BigDecimal::from_str(&v).unwrap_or_else(|_| BigDecimal::from(0))),
                market_cap: market_cap.map(|v| u64::from_str(&v).unwrap_or(0)),
                foreign_buy: foreign_buy.map(|v| i64::from_str(&v).unwrap_or(0)),
                trust_buy: trust_buy.map(|v| i64::from_str(&v).unwrap_or(0)),
                dealer_buy: dealer_buy.map(|v| i64::from_str(&v).unwrap_or(0)),
            };
            stock_prices.push(stock_price);
        }

        Ok(stock_prices)
    }

    async fn find_latest_by_stock_id(&self, stock_id: &Uuid) -> Result<Option<StockPrice>> {
        let query = format!(
            r#"
            SELECT 
                id, stock_id, date, 
                open::text as open, high::text as high, low::text as low, close::text as close,
                volume::text as volume, 
                change::text as change, change_percent::text as change_percent,
                turnover::text as turnover, transactions::text as transactions, 
                pe_ratio::text as pe_ratio, pb_ratio::text as pb_ratio, dividend_yield::text as dividend_yield,
                market_cap::text as market_cap,
                foreign_buy::text as foreign_buy, trust_buy::text as trust_buy, dealer_buy::text as dealer_buy
            FROM stock_prices
            WHERE stock_id = $1
            ORDER BY date DESC
            LIMIT 1
            "#
        );

        let row_result = sqlx::query(&query)
            .bind(stock_id)
            .fetch_one(&self.pool)
            .await;

        match row_result {
            Ok(r) => {
                let id: Uuid = r.get("id");
                let stock_id: Uuid = r.get("stock_id");
                let date: Date = r.get("date");
                let open: String = r.get("open");
                let high: String = r.get("high");
                let low: String = r.get("low");
                let close: String = r.get("close");
                let volume: String = r.get("volume");
                let change: String = r.get("change");
                let change_percent: String = r.get("change_percent");
                let turnover: String = r.get("turnover");
                let transactions: String = r.get("transactions");
                let pe_ratio: Option<String> = r.get("pe_ratio");
                let pb_ratio: Option<String> = r.get("pb_ratio");
                let dividend_yield: Option<String> = r.get("dividend_yield");
                let market_cap: Option<String> = r.get("market_cap");
                let foreign_buy: Option<String> = r.get("foreign_buy");
                let trust_buy: Option<String> = r.get("trust_buy");
                let dealer_buy: Option<String> = r.get("dealer_buy");

                let stock_price = StockPrice {
                    id,
                    stock_id,
                    date,
                    open: BigDecimal::from_str(&open).unwrap_or_else(|_| BigDecimal::from(0)),
                    high: BigDecimal::from_str(&high).unwrap_or_else(|_| BigDecimal::from(0)),
                    low: BigDecimal::from_str(&low).unwrap_or_else(|_| BigDecimal::from(0)),
                    close: BigDecimal::from_str(&close).unwrap_or_else(|_| BigDecimal::from(0)),
                    volume: u64::from_str(&volume).unwrap_or(0),
                    change: BigDecimal::from_str(&change).unwrap_or_else(|_| BigDecimal::from(0)),
                    change_percent: BigDecimal::from_str(&change_percent).unwrap_or_else(|_| BigDecimal::from(0)),
                    turnover: u64::from_str(&turnover).unwrap_or(0),
                    transactions: u64::from_str(&transactions).unwrap_or(0),
                    pe_ratio: pe_ratio.map(|v| BigDecimal::from_str(&v).unwrap_or_else(|_| BigDecimal::from(0))),
                    pb_ratio: pb_ratio.map(|v| BigDecimal::from_str(&v).unwrap_or_else(|_| BigDecimal::from(0))),
                    dividend_yield: dividend_yield.map(|v| BigDecimal::from_str(&v).unwrap_or_else(|_| BigDecimal::from(0))),
                    market_cap: market_cap.map(|v| u64::from_str(&v).unwrap_or(0)),
                    foreign_buy: foreign_buy.map(|v| i64::from_str(&v).unwrap_or(0)),
                    trust_buy: trust_buy.map(|v| i64::from_str(&v).unwrap_or(0)),
                    dealer_buy: dealer_buy.map(|v| i64::from_str(&v).unwrap_or(0)),
                };
                Ok(Some(stock_price))
            }
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
