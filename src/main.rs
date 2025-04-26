// 引入應用層服務
use crate::application::services::{StockPriceService, StockService};
// 引入領域層儲存庫介面
use crate::domain::repositories::{StockPriceRepository, StockRepository};
// 引入基礎設施層的爬蟲服務
use crate::infrastructure::external_services::StockCrawlerService;
// 引入基礎設施層的資料庫儲存庫實現
use crate::infrastructure::persistence::{PostgresStockPriceRepository, PostgresStockRepository};
// 引入資料庫連接池創建函數
use crate::infrastructure::persistence::database::create_pool;
// 引入表現層控制器
use crate::presentation::controllers::{StockController, StockPriceController};
// 引入表現層 API 路由
use crate::presentation::api::create_api_router;
// 引入 Axum Web 框架相關組件
use axum::Router;
// 引入環境變數處理庫
use dotenv::dotenv;
// 引入標準庫的錯誤處理模組
use std::error::Error;
// 引入 Tokio 非同步運行時
use tokio::net::TcpListener;
// 引入 PostgreSQL 連接池類型
use sqlx::postgres::PgPool;
// 引入日誌記錄相關組件
use tracing::{info, error};
// 引入日誌訂閱器
use tracing_subscriber;
// 引入 UUID 生成庫
use uuid::Uuid;

// 程式入口點
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化日誌系統
    tracing_subscriber::fmt::init();
    info!("Starting Taiwan Stocks Crawler");
    
    // 載入環境變數
    dotenv().ok();
    
    // 從環境變數獲取資料庫連接字串
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    info!("連接到資料庫: {}", database_url);
    
    // 創建資料庫連接池
    let pool = create_pool(&database_url).await?;
    info!("資料庫連接建立成功");
    
    // 確保資料庫結構是最新的
    ensure_database_schema(&pool).await?;
    info!("資料庫結構檢查完成");
    
    // 初始化儲存庫
    let stock_repo = Arc::new(PostgresStockRepository::new(pool.clone()));
    let price_repo = Arc::new(PostgresStockPriceRepository::new(pool.clone()));
    info!("儲存庫初始化完成");
    
    // 初始化爬蟲服務
    let crawler_service = Arc::new(StockCrawlerService::new());
    info!("股票爬蟲服務初始化完成");
    
    // 初始化應用服務
    let stock_service = Arc::new(StockService::new(stock_repo.clone()));
    let price_service = Arc::new(StockPriceService::new(price_repo.clone()));
    info!("應用服務初始化完成");
    
    // 初始化控制器
    let stock_controller = Arc::new(StockController::new(stock_service.clone()));
    let price_controller = Arc::new(StockPriceController::new(price_service.clone()));
    info!("控制器初始化完成");
    
    // 創建 API 路由
    let app = create_api_router(stock_controller, price_controller);
    
    // 啟動爬蟲程序
    let crawler_service = crawler_service.clone();
    let stock_repo = stock_repo.clone();
    let price_repo = price_repo.clone();
    
    tokio::spawn(async move {
        info!("開始股票爬蟲程序...");
        
        // 爬取股票列表
        match crawler_service.crawl_stock_list().await {
            Ok(stocks) => {
                info!("成功爬取 {} 支股票", stocks.len());
                
                // Crawl prices for each stock
                for stock in &stocks { // 爬取所有股票的價格數據
                    info!("爬取股票價格: {} - {}", stock.code, stock.name);
                    
                    // 先檢查股票是否存在於資料庫中
                    match stock_repo.find_by_code(&stock.code).await {
                        Ok(Some(db_stock)) => {
                            info!("股票 {} 已存在於資料庫中，使用資料庫中的股票 ID", stock.code);
                            
                            match crawler_service.crawl_stock_prices(&stock.code).await {
                                Ok(mut prices) => {
                                    info!("成功爬取股票 {} 的 {} 筆價格資料", stock.code, prices.len());
                                    
                                    // 使用資料庫中的 stock_id
                                    for price in &mut prices {
                                        price.stock_id = db_stock.id;
                                    }
                                    
                                    // Save prices to database
                                    for price in &prices {
                                        if let Err(e) = price_repo.create(price).await {
                                            info!("儲存股票 {} 在 {} 的價格資料失敗: {}", 
                                                  stock.code, price.date, e);
                                        } else {
                                            info!("儲存股票 {} 在 {} 的價格資料", stock.code, price.date);
                                        }
                                    }
                                },
                                Err(e) => {
                                    info!("爬取股票 {} 的價格資料失敗: {}", stock.code, e);
                                }
                            }
                        },
                        Ok(None) => {
                            info!("股票 {} 不存在於資料庫中，先儲存股票資訊", stock.code);
                            if let Err(e) = stock_repo.save(stock).await {
                                info!("儲存股票 {} 失敗: {}", stock.code, e);
                            } else {
                                info!("儲存股票: {} - {}", stock.code, stock.name);
                                
                                // 現在股票已經儲存，可以爬取價格資料了
                                match crawler_service.crawl_stock_prices(&stock.code).await {
                                    Ok(mut prices) => {
                                        info!("成功爬取股票 {} 的 {} 筆價格資料", stock.code, prices.len());
                                        
                                        // 使用正確的 stock_id
                                        for price in &mut prices {
                                            price.stock_id = stock.id;
                                        }
                                        
                                        // Save prices to database
                                        for price in &prices {
                                            if let Err(e) = price_repo.create(price).await {
                                                info!("儲存股票 {} 在 {} 的價格資料失敗: {}", 
                                                      stock.code, price.date, e);
                                            } else {
                                                info!("儲存股票 {} 在 {} 的價格資料", stock.code, price.date);
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        info!("爬取股票 {} 的價格資料失敗: {}", stock.code, e);
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            info!("查詢股票 {} 失敗: {}", stock.code, e);
                        }
                    }
                }
                
                info!("股票爬蟲程序完成");
            },
            Err(e) => {
                error!("爬取股票列表失敗: {}", e);
            }
        }
    });
    
    // 啟動 Web 服務器
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    info!("API 服務器啟動在 http://0.0.0.0:3000");
    axum::serve(listener, app).await?;
    
    Ok(())
}

// 確保資料庫結構是最新的，如果需要則添加缺少的欄位
async fn ensure_database_schema(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    info!("檢查資料庫結構...");
    
    // 檢查 stocks 表是否存在，如果不存在則創建
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS stocks (
            id UUID PRIMARY KEY,
            code VARCHAR(10) UNIQUE NOT NULL,
            name VARCHAR(100) NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
    "#)
    .execute(pool)
    .await?;
    
    // 檢查 stock_prices 表是否存在，如果不存在則創建
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS stock_prices (
            id UUID PRIMARY KEY,
            stock_id UUID NOT NULL REFERENCES stocks(id),
            date DATE NOT NULL,
            open NUMERIC(10, 2) NOT NULL,
            high NUMERIC(10, 2) NOT NULL,
            low NUMERIC(10, 2) NOT NULL,
            close NUMERIC(10, 2) NOT NULL,
            volume NUMERIC(20, 0) NOT NULL,
            change NUMERIC(10, 2) NOT NULL DEFAULT 0.0,
            change_percent NUMERIC(10, 2) NOT NULL DEFAULT 0.0,
            turnover NUMERIC(20, 0) NOT NULL DEFAULT 0,
            transactions INTEGER NOT NULL DEFAULT 0,
            pe_ratio NUMERIC(10, 2),
            pb_ratio NUMERIC(10, 2),
            dividend_yield NUMERIC(10, 2),
            market_cap NUMERIC(20, 0),
            foreign_buy NUMERIC(20, 0),
            trust_buy NUMERIC(20, 0),
            dealer_buy NUMERIC(20, 0),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            UNIQUE(stock_id, date)
        )
    "#)
    .execute(pool)
    .await?;
    
    Ok(())
}

// 模組聲明
mod domain;
mod application;
mod infrastructure;
mod presentation;
