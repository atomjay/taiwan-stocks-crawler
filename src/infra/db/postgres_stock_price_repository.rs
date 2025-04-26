use crate::domain::models::StockPrice;
use crate::domain::repositories::StockPriceRepository;
use crate::domain::value_objects::Result;
use async_trait::async_trait;
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use time::Date;

pub struct PostgresStockPriceRepository {
    pool: PgPool,
}

impl PostgresStockPriceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StockPriceRepository for PostgresStockPriceRepository {
    async fn create(&self, stock_price: &StockPrice) -> Result<()> {
        // 使用非宏版本的 sqlx::query 來避免編譯時驗證
        let query = "
            INSERT INTO stock_prices (
                id, stock_id, date, open, high, low, close, volume, 
                change, change_percent, turnover, transactions, 
                pe_ratio, pb_ratio, dividend_yield, market_cap, 
                foreign_buy, trust_buy, dealer_buy
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, 
                $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19
            )
            ON CONFLICT (stock_id, date) DO UPDATE
            SET 
                open = $4, high = $5, low = $6, close = $7, volume = $8,
                change = $9, change_percent = $10, turnover = $11, transactions = $12,
                pe_ratio = $13, pb_ratio = $14, dividend_yield = $15, market_cap = $16,
                foreign_buy = $17, trust_buy = $18, dealer_buy = $19
        ";
        
        sqlx::query(query)
            .bind(stock_price.id)
            .bind(stock_price.stock_id)
            .bind(stock_price.date)
            .bind(stock_price.open)
            .bind(stock_price.high)
            .bind(stock_price.low)
            .bind(stock_price.close)
            .bind(stock_price.volume as i64)
            .bind(stock_price.change)
            .bind(stock_price.change_percent)
            .bind(stock_price.turnover as i64)
            .bind(stock_price.transactions as i64)
            .bind(stock_price.pe_ratio)
            .bind(stock_price.pb_ratio)
            .bind(stock_price.dividend_yield)
            .bind(stock_price.market_cap.map(|v| v as i64))
            .bind(stock_price.foreign_buy)
            .bind(stock_price.trust_buy)
            .bind(stock_price.dealer_buy)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<StockPrice>> {
        // 使用非宏版本的 sqlx::query_as 來避免編譯時驗證
        let query = "
            SELECT 
                id, stock_id, date, open, high, low, close, volume,
                change, change_percent, turnover, transactions,
                pe_ratio, pb_ratio, dividend_yield, market_cap,
                foreign_buy, trust_buy, dealer_buy
            FROM stock_prices
            WHERE id = $1
        ";
        
        let record = sqlx::query_as::<_, StockPriceRecord>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(record.map(|r| StockPrice {
            id: r.id,
            stock_id: r.stock_id,
            date: r.date,
            open: r.open,
            high: r.high,
            low: r.low,
            close: r.close,
            volume: r.volume as u64,
            change: r.change,
            change_percent: r.change_percent,
            turnover: r.turnover.map(|v| v as u64).unwrap_or(0),
            transactions: r.transactions.map(|v| v as u64).unwrap_or(0),
            pe_ratio: r.pe_ratio,
            pb_ratio: r.pb_ratio,
            dividend_yield: r.dividend_yield,
            market_cap: r.market_cap.map(|v| v as u64),
            foreign_buy: r.foreign_buy,
            trust_buy: r.trust_buy,
            dealer_buy: r.dealer_buy,
        }))
    }

    async fn find_by_stock_id(&self, stock_id: &Uuid) -> Result<Vec<StockPrice>> {
        // 使用非宏版本的 sqlx::query_as 來避免編譯時驗證
        let query = "
            SELECT 
                id, stock_id, date, open, high, low, close, volume,
                change, change_percent, turnover, transactions,
                pe_ratio, pb_ratio, dividend_yield, market_cap,
                foreign_buy, trust_buy, dealer_buy
            FROM stock_prices
            WHERE stock_id = $1
            ORDER BY date DESC
        ";
        
        let records = sqlx::query_as::<_, StockPriceRecord>(query)
            .bind(stock_id)
            .fetch_all(&self.pool)
            .await?;

        let prices = records
            .into_iter()
            .map(|r| StockPrice {
                id: r.id,
                stock_id: r.stock_id,
                date: r.date,
                open: r.open,
                high: r.high,
                low: r.low,
                close: r.close,
                volume: r.volume as u64,
                change: r.change,
                change_percent: r.change_percent,
                turnover: r.turnover.map(|v| v as u64).unwrap_or(0),
                transactions: r.transactions.map(|v| v as u64).unwrap_or(0),
                pe_ratio: r.pe_ratio,
                pb_ratio: r.pb_ratio,
                dividend_yield: r.dividend_yield,
                market_cap: r.market_cap.map(|v| v as u64),
                foreign_buy: r.foreign_buy,
                trust_buy: r.trust_buy,
                dealer_buy: r.dealer_buy,
            })
            .collect();

        Ok(prices)
    }

    async fn find_by_stock_id_and_date_range(
        &self,
        stock_id: &Uuid,
        start_date: Option<Date>,
        end_date: Option<Date>,
    ) -> Result<Vec<StockPrice>> {
        let prices = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                // 使用非宏版本的 sqlx::query_as 來避免編譯時驗證
                let query = "
                    SELECT 
                        id, stock_id, date, open, high, low, close, volume,
                        change, change_percent, turnover, transactions,
                        pe_ratio, pb_ratio, dividend_yield, market_cap,
                        foreign_buy, trust_buy, dealer_buy
                    FROM stock_prices
                    WHERE stock_id = $1 AND date BETWEEN $2 AND $3
                    ORDER BY date DESC
                ";
                
                let records = sqlx::query_as::<_, StockPriceRecord>(query)
                    .bind(stock_id)
                    .bind(start)
                    .bind(end)
                    .fetch_all(&self.pool)
                    .await?;

                records
                    .into_iter()
                    .map(|r| StockPrice {
                        id: r.id,
                        stock_id: r.stock_id,
                        date: r.date,
                        open: r.open,
                        high: r.high,
                        low: r.low,
                        close: r.close,
                        volume: r.volume as u64,
                        change: r.change,
                        change_percent: r.change_percent,
                        turnover: r.turnover.map(|v| v as u64).unwrap_or(0),
                        transactions: r.transactions.map(|v| v as u64).unwrap_or(0),
                        pe_ratio: r.pe_ratio,
                        pb_ratio: r.pb_ratio,
                        dividend_yield: r.dividend_yield,
                        market_cap: r.market_cap.map(|v| v as u64),
                        foreign_buy: r.foreign_buy,
                        trust_buy: r.trust_buy,
                        dealer_buy: r.dealer_buy,
                    })
                    .collect()
            }
            (Some(start), None) => {
                // 使用非宏版本的 sqlx::query_as 來避免編譯時驗證
                let query = "
                    SELECT 
                        id, stock_id, date, open, high, low, close, volume,
                        change, change_percent, turnover, transactions,
                        pe_ratio, pb_ratio, dividend_yield, market_cap,
                        foreign_buy, trust_buy, dealer_buy
                    FROM stock_prices
                    WHERE stock_id = $1 AND date >= $2
                    ORDER BY date DESC
                ";
                
                let records = sqlx::query_as::<_, StockPriceRecord>(query)
                    .bind(stock_id)
                    .bind(start)
                    .fetch_all(&self.pool)
                    .await?;

                records
                    .into_iter()
                    .map(|r| StockPrice {
                        id: r.id,
                        stock_id: r.stock_id,
                        date: r.date,
                        open: r.open,
                        high: r.high,
                        low: r.low,
                        close: r.close,
                        volume: r.volume as u64,
                        change: r.change,
                        change_percent: r.change_percent,
                        turnover: r.turnover.map(|v| v as u64).unwrap_or(0),
                        transactions: r.transactions.map(|v| v as u64).unwrap_or(0),
                        pe_ratio: r.pe_ratio,
                        pb_ratio: r.pb_ratio,
                        dividend_yield: r.dividend_yield,
                        market_cap: r.market_cap.map(|v| v as u64),
                        foreign_buy: r.foreign_buy,
                        trust_buy: r.trust_buy,
                        dealer_buy: r.dealer_buy,
                    })
                    .collect()
            }
            (None, Some(end)) => {
                // 使用非宏版本的 sqlx::query_as 來避免編譯時驗證
                let query = "
                    SELECT 
                        id, stock_id, date, open, high, low, close, volume,
                        change, change_percent, turnover, transactions,
                        pe_ratio, pb_ratio, dividend_yield, market_cap,
                        foreign_buy, trust_buy, dealer_buy
                    FROM stock_prices
                    WHERE stock_id = $1 AND date <= $2
                    ORDER BY date DESC
                ";
                
                let records = sqlx::query_as::<_, StockPriceRecord>(query)
                    .bind(stock_id)
                    .bind(end)
                    .fetch_all(&self.pool)
                    .await?;

                records
                    .into_iter()
                    .map(|r| StockPrice {
                        id: r.id,
                        stock_id: r.stock_id,
                        date: r.date,
                        open: r.open,
                        high: r.high,
                        low: r.low,
                        close: r.close,
                        volume: r.volume as u64,
                        change: r.change,
                        change_percent: r.change_percent,
                        turnover: r.turnover.map(|v| v as u64).unwrap_or(0),
                        transactions: r.transactions.map(|v| v as u64).unwrap_or(0),
                        pe_ratio: r.pe_ratio,
                        pb_ratio: r.pb_ratio,
                        dividend_yield: r.dividend_yield,
                        market_cap: r.market_cap.map(|v| v as u64),
                        foreign_buy: r.foreign_buy,
                        trust_buy: r.trust_buy,
                        dealer_buy: r.dealer_buy,
                    })
                    .collect()
            }
            (None, None) => self.find_by_stock_id(stock_id).await?,
        };

        Ok(prices)
    }

    async fn find_latest_by_stock_id(&self, stock_id: &Uuid) -> Result<Option<StockPrice>> {
        // 使用非宏版本的 sqlx::query_as 來避免編譯時驗證
        let query = "
            SELECT 
                id, stock_id, date, open, high, low, close, volume,
                change, change_percent, turnover, transactions,
                pe_ratio, pb_ratio, dividend_yield, market_cap,
                foreign_buy, trust_buy, dealer_buy
            FROM stock_prices
            WHERE stock_id = $1
            ORDER BY date DESC
            LIMIT 1
        ";
        
        let record = sqlx::query_as::<_, StockPriceRecord>(query)
            .bind(stock_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(record.map(|r| StockPrice {
            id: r.id,
            stock_id: r.stock_id,
            date: r.date,
            open: r.open,
            high: r.high,
            low: r.low,
            close: r.close,
            volume: r.volume as u64,
            change: r.change,
            change_percent: r.change_percent,
            turnover: r.turnover.map(|v| v as u64).unwrap_or(0),
            transactions: r.transactions.map(|v| v as u64).unwrap_or(0),
            pe_ratio: r.pe_ratio,
            pb_ratio: r.pb_ratio,
            dividend_yield: r.dividend_yield,
            market_cap: r.market_cap.map(|v| v as u64),
            foreign_buy: r.foreign_buy,
            trust_buy: r.trust_buy,
            dealer_buy: r.dealer_buy,
        }))
    }
}

// 定義一個輔助結構體來處理查詢結果
#[derive(FromRow)]
struct StockPriceRecord {
    id: Uuid,
    stock_id: Uuid,
    date: Date,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i64,
    change: f64,
    change_percent: f64,
    turnover: Option<i64>,
    transactions: Option<i64>,
    pe_ratio: Option<f64>,
    pb_ratio: Option<f64>,
    dividend_yield: Option<f64>,
    market_cap: Option<i64>,
    foreign_buy: Option<i64>,
    trust_buy: Option<i64>,
    dealer_buy: Option<i64>,
}
