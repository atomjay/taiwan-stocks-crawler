use crate::domain::models::{Stock, StockPrice};
use crate::infra::external_services::line_notification_service::LineNotificationService;
use crate::application::services::{StockService, StockPriceService};
use anyhow::{Result, Context};
use std::sync::Arc;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use tracing::info;
use uuid::Uuid;

/// 通知服務，負責處理股票相關通知
pub struct NotificationService {
    stock_service: Arc<StockService>,
    price_service: Arc<StockPriceService>,
    line_service: Arc<LineNotificationService>,
}

impl NotificationService {
    /// 創建新的通知服務實例
    pub fn new(
        stock_service: Arc<StockService>,
        price_service: Arc<StockPriceService>,
    ) -> Result<Self> {
        // 從環境變數獲取 LINE 配置
        let channel_access_token = std::env::var("LINE_CHANNEL_ACCESS_TOKEN")
            .context("LINE_CHANNEL_ACCESS_TOKEN 環境變數未設置")?;
        
        let user_id = std::env::var("LINE_USER_ID")
            .context("LINE_USER_ID 環境變數未設置")?;
        
        // 創建 LINE 通知服務
        let line_service = Arc::new(LineNotificationService::new(
            channel_access_token,
            user_id,
        ));
        
        Ok(Self {
            stock_service,
            price_service,
            line_service,
        })
    }
    
    /// 發送股票價格通知
    pub async fn send_stock_price_notification(&self, stock_code: &str) -> Result<()> {
        // 獲取股票資訊
        let stock = self.stock_service.get_stock_by_code(stock_code)
            .await?
            .context(format!("找不到股票代碼為 {} 的股票", stock_code))?;
        
        // 獲取最新價格
        let latest_price = self.price_service.get_latest_price_by_stock_id(&stock.id)
            .await?
            .context(format!("找不到股票 {} 的最新價格", stock_code))?;
        
        // 當前日期
        let date = time::OffsetDateTime::now_utc().date();
        
        // 將 DTO 轉換為模型
        let price_model = StockPrice {
            id: Uuid::new_v4(),
            stock_id: Uuid::parse_str(&stock.id)?,
            date,
            open: BigDecimal::from_str(&latest_price.open.to_string())?,
            high: BigDecimal::from_str(&latest_price.high.to_string())?,
            low: BigDecimal::from_str(&latest_price.low.to_string())?,
            close: BigDecimal::from_str(&latest_price.close.to_string())?,
            volume: latest_price.volume,
            change: BigDecimal::from_str(&latest_price.change.to_string())?,
            change_percent: BigDecimal::from_str(&latest_price.change_percent.to_string())?,
            turnover: latest_price.turnover,
            transactions: latest_price.transactions,
            pe_ratio: latest_price.pe_ratio.map(|v| BigDecimal::from_str(&v.to_string()).unwrap_or_else(|_| BigDecimal::from(0))),
            pb_ratio: latest_price.pb_ratio.map(|v| BigDecimal::from_str(&v.to_string()).unwrap_or_else(|_| BigDecimal::from(0))),
            dividend_yield: latest_price.dividend_yield.map(|v| BigDecimal::from_str(&v.to_string()).unwrap_or_else(|_| BigDecimal::from(0))),
            market_cap: latest_price.market_cap,
            foreign_buy: latest_price.foreign_buy,
            trust_buy: latest_price.trust_buy,
            dealer_buy: latest_price.dealer_buy,
        };
        
        // 發送通知
        let stock_model = Stock {
            id: Uuid::parse_str(&stock.id)?,
            code: stock.code.clone(),
            name: stock.name.clone(),
            last_updated: time::OffsetDateTime::now_utc(),
        };
        
        self.line_service.send_stock_price_notification(&stock_model, &price_model).await?;
        
        Ok(())
    }
    
    /// 發送每日股票摘要通知
    pub async fn send_daily_summary(&self) -> Result<()> {
        // 獲取當前日期
        let today = time::OffsetDateTime::now_utc().date();
        
        // 獲取所有股票
        let stocks = self.stock_service.get_all_stocks().await?;
        if stocks.is_empty() {
            info!("沒有股票數據可用於摘要");
            return Ok(());
        }
        
        // 收集所有股票的最新價格
        let mut stock_prices = Vec::new();
        for stock in &stocks {
            // 獲取最新價格
            let latest_price = self.price_service.get_latest_price_by_stock_id(&stock.id)
                .await?
                .context(format!("找不到股票 {} 的最新價格", stock.code))?;
            
            // 當前日期
            let date = time::OffsetDateTime::now_utc().date();
            
            // 將 DTO 轉換為模型
            let price_model = StockPrice {
                id: Uuid::new_v4(),
                stock_id: Uuid::parse_str(&stock.id)?,
                date,
                open: BigDecimal::from_str(&latest_price.open.to_string())?,
                high: BigDecimal::from_str(&latest_price.high.to_string())?,
                low: BigDecimal::from_str(&latest_price.low.to_string())?,
                close: BigDecimal::from_str(&latest_price.close.to_string())?,
                volume: latest_price.volume,
                change: BigDecimal::from_str(&latest_price.change.to_string())?,
                change_percent: BigDecimal::from_str(&latest_price.change_percent.to_string())?,
                turnover: latest_price.turnover,
                transactions: latest_price.transactions,
                pe_ratio: latest_price.pe_ratio.map(|v| BigDecimal::from_str(&v.to_string()).unwrap_or_else(|_| BigDecimal::from(0))),
                pb_ratio: latest_price.pb_ratio.map(|v| BigDecimal::from_str(&v.to_string()).unwrap_or_else(|_| BigDecimal::from(0))),
                dividend_yield: latest_price.dividend_yield.map(|v| BigDecimal::from_str(&v.to_string()).unwrap_or_else(|_| BigDecimal::from(0))),
                market_cap: latest_price.market_cap,
                foreign_buy: latest_price.foreign_buy,
                trust_buy: latest_price.trust_buy,
                dealer_buy: latest_price.dealer_buy,
            };
            
            // 發送通知
            let stock_model = Stock {
                id: Uuid::parse_str(&stock.id)?,
                code: stock.code.clone(),
                name: stock.name.clone(),
                last_updated: time::OffsetDateTime::now_utc(),
            };
            
            stock_prices.push((stock_model, price_model));
        }
        
        if stock_prices.is_empty() {
            info!("沒有股票價格數據可用於摘要");
            return Ok(());
        }
        
        // 發送摘要通知
        self.line_service.send_daily_summary(today, stock_prices).await?;
        
        Ok(())
    }
    
    /// 發送自訂訊息
    pub async fn send_custom_message(&self, message: &str) -> Result<()> {
        info!("發送自訂訊息: {}", message);
        self.line_service.send_custom_message(message).await?;
        
        Ok(())
    }
}
