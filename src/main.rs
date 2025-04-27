// 引入應用層服務
use crate::application::services::{StockPriceService, StockService, NotificationService};
// 引入應用層 DTO
use crate::application::dtos::{CreateStockDto, CreateStockPriceDto};
// 引入基礎設施層的爬蟲服務
use crate::infra::external_services::stock_crawler_service::StockCrawlerService;
// 引入基礎設施層的資料庫儲存庫實現
use crate::infra::db::postgres_stock_repository::PostgresStockRepository;
use crate::infra::db::postgres_stock_price_repository::PostgresStockPriceRepository;
// 引入資料庫連接池創建函數
use crate::infra::db::database::create_pool;
// 引入表現層控制器和處理函數
use crate::api::controllers::{
    stock_controller::StockController, 
    stock_price_controller::StockPriceController,
};
use crate::api::routes::{get_all_stocks, get_stock_by_code, get_stock_prices_by_stock_id};
// 引入 Axum Web 框架相關組件
use axum::{
    Router,
    routing::get,
};
// 引入環境變數處理庫
use dotenv::dotenv;
// 引入標準庫的錯誤處理模組
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
// 引入 Tokio 非同步運行時
use tokio::net::TcpListener;
use tokio::time as tokio_time;
// 引入 time 庫
use time::{OffsetDateTime, UtcOffset};
// 引入日誌記錄相關組件
use tracing::{info, error};
// 引入日誌訂閱器
use tracing_subscriber;

// 引入領域實體
use crate::domain::models::StockPrice;

// 引入 Uuid 類別
use uuid::Uuid;

// 定義 AppState 結構體
#[derive(Clone)]
pub struct AppState {
    stock_controller: Arc<StockController>,
    price_controller: Arc<StockPriceController>,
}

// 程式入口點
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化系統
    let (_pool, stock_service, price_service, stock_controller, price_controller) = match initialize_system().await {
        Ok(result) => result,
        Err(e) => {
            error!("系統初始化失敗: {}", e);
            return Err(e.into());
        }
    };
    
    // 創建 API 路由
    let app = create_api_router(stock_controller.clone(), price_controller.clone());
    
    // 初始化爬蟲服務
    let crawler_service = Arc::new(StockCrawlerService::new());
    
    // 執行爬蟲任務
    if let Err(e) = crawl_and_save_data(crawler_service.clone(), stock_service.clone(), price_service.clone()).await {
        error!("爬蟲任務執行失敗: {}", e);
    } else {
        info!("爬蟲任務執行成功");
    }
    
    // 初始化通知服務
    let notification_service = match NotificationService::new(stock_service.clone(), price_service.clone()) {
        Ok(service) => {
            info!("通知服務初始化成功");
            Arc::new(service)
        },
        Err(e) => {
            error!("通知服務初始化失敗: {}", e);
            return Err(e.into());
        }
    };
    
    // 發送初始通知
    if let Err(e) = notification_service.send_custom_message("台灣股票爬蟲系統已啟動，開始監控股票數據").await {
        error!("初始通知發送失敗: {}", e);
    } else {
        info!("初始通知發送成功");
    }
    
    // 啟動定時通知任務 - 由於 Send 問題，暫時不使用 tokio::spawn
    // 直接在主線程中執行一次通知
    info!("執行一次股票通知測試");
    if let Err(e) = notification_service.send_daily_summary().await {
        error!("每日摘要通知測試發送失敗: {}", e);
    } else {
        info!("每日摘要通知測試發送成功");
    }
    
    // 啟動 Web 服務器
    if let Err(e) = start_web_server(app).await {
        error!("Web 服務器啟動失敗: {}", e);
        return Err(e.into());
    } else {
        info!("Web 服務器啟動成功");
    }
    
    Ok(())
}

/// 排程每日通知任務
async fn schedule_daily_notification(
    notification_service: Arc<NotificationService>,
    stock_service: Arc<StockService>,
    price_service: Arc<StockPriceService>,
    crawler_service: Arc<StockCrawlerService>
) {
    info!("啟動每日通知排程任務");
    
    // 設定每日固定時間發送通知 (例如：每天下午 3:30，台灣股市收盤後)
    let mut interval = tokio_time::interval(Duration::from_secs(60 * 60 * 24)); // 每24小時執行一次
    
    loop {
        interval.tick().await;
        
        // 檢查當前時間是否為工作日且在下午 3:30 左右
        let now = OffsetDateTime::now_utc().to_offset(UtcOffset::from_hms(8, 0, 0).unwrap());
        let hour = now.hour();
        let minute = now.minute();
        let weekday = now.weekday();
        
        // 如果是週一到週五，且時間在下午 3:30 左右
        if (weekday.number_from_monday() <= 5) && (hour == 15 && minute >= 30 && minute <= 40) {
            info!("開始執行每日股票數據爬取和通知任務");
            
            // 重新爬取最新數據
            if let Err(e) = crawl_and_save_data(crawler_service.clone(), stock_service.clone(), price_service.clone()).await {
                error!("每日股票數據爬取失敗: {}", e);
            } else {
                info!("每日股票數據爬取成功");
            }
            
            // 發送每日摘要通知
            if let Err(e) = notification_service.send_daily_summary().await {
                error!("每日摘要通知發送失敗: {}", e);
            } else {
                info!("每日摘要通知發送成功");
            }
        }
    }
}

/// 初始化系統：設置日誌、環境變數和資料庫連接
async fn initialize_system() -> Result<(
    Arc<sqlx::PgPool>, 
    Arc<StockService>, 
    Arc<StockPriceService>, 
    Arc<StockController>, 
    Arc<StockPriceController>
), Box<dyn Error>> {
    // 初始化日誌系統，使用更明確的配置
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();
    
    println!("程式開始執行");
    info!("Starting Taiwan Stocks Crawler");
    
    // 載入環境變數
    dotenv().ok();
    println!("環境變數載入完成");
    
    // 從環境變量中獲取資料庫 URL
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL 環境變量未設置");
    println!("資料庫 URL: {}", database_url);
    info!("資料庫 URL: {}", database_url);
    
    // 建立資料庫連接池
    println!("正在建立資料庫連接池...");
    let pool = create_pool(&database_url).await?;
    println!("資料庫連接建立成功");
    info!("資料庫連接建立成功");
    
    // 使用遷移系統確保資料庫結構存在
    run_migrations(&database_url).await?;
    info!("資料庫遷移完成");
    
    // 初始化儲存庫
    let stock_repo = Arc::new(PostgresStockRepository::new(pool.clone()));
    let price_repo = Arc::new(PostgresStockPriceRepository::new(pool.clone()));
    info!("儲存庫初始化完成");
    
    // 將連接池包裝在 Arc 中
    let pool_arc = Arc::new(pool);
    
    // 初始化應用服務
    let stock_service = Arc::new(StockService::new(stock_repo.clone()));
    let price_service = Arc::new(StockPriceService::new(price_repo.clone()));
    info!("應用服務初始化完成");
    
    // 初始化控制器
    let stock_controller = Arc::new(StockController::new(stock_service.clone()));
    let price_controller = Arc::new(StockPriceController::new(price_service.clone()));
    info!("控制器初始化完成");
    
    Ok((pool_arc, stock_service, price_service, stock_controller, price_controller))
}

/// 創建 API 路由
fn create_api_router(
    stock_controller: Arc<StockController>,
    price_controller: Arc<StockPriceController>
) -> Router {
    let app = Router::new()
        .route("/api/stocks", get(get_all_stocks))
        .route("/api/stocks/:code", get(get_stock_by_code))
        .route("/api/stocks/:code/prices", get(get_stock_prices_by_stock_id))
        .with_state(AppState {
            stock_controller,
            price_controller,
        });
    
    info!("API 路由初始化完成");
    app
}

/// 爬取股票和價格數據並保存到資料庫
async fn crawl_and_save_data(
    crawler_service: Arc<StockCrawlerService>,
    stock_service: Arc<StockService>,
    price_service: Arc<StockPriceService>
) -> Result<(), Box<dyn Error>> {
    info!("開始股票爬蟲程序...");
    
    // 爬取股票列表
    match crawler_service.crawl_stocks().await {
        Ok(stocks) => {
            info!("成功爬取 {} 支股票", stocks.len());
            
            // 保存股票到資料庫
            for stock in stocks {
                let stock_code = stock.code.clone();
                let stock_name = stock.name.clone();
                
                // 先保存股票
                match stock_service.create_stock(CreateStockDto {
                    code: stock_code.clone(),
                    name: stock_name.clone(),
                }).await {
                    Ok(_saved_stock) => {
                        info!("保存股票成功: {} - {}", stock_code, stock_name);
                        
                        // 爬取股票價格
                        match crawler_service.crawl_stock_prices(&stock_code).await {
                            Ok(prices) => {
                                info!("成功爬取 {} 的價格數據, 共 {} 筆", stock_code, prices.len());
                                
                                // 從資料庫中查詢股票的 ID
                                match stock_service.get_stock_by_code(&stock_code).await {
                                    Ok(Some(db_stock)) => {
                                        // 保存價格到資料庫，使用從資料庫中查詢到的股票 ID
                                        for price in prices {
                                            let stock_id = Uuid::parse_str(&db_stock.id).unwrap_or_default();
                                            let price_with_stock_id = StockPrice {
                                                stock_id,
                                                ..price
                                            };
                                            
                                            match price_service.create_stock_price(CreateStockPriceDto::from(price_with_stock_id)).await {
                                                Ok(_) => info!("保存價格成功: {} - {}", stock_code, price.date),
                                                Err(e) => error!("保存價格失敗: {} - {}, 錯誤: {}", stock_code, price.date, e),
                                            }
                                        }
                                    },
                                    Ok(None) => error!("無法從資料庫中查詢到股票: {}", stock_code),
                                    Err(e) => error!("查詢股票失敗: {}, 錯誤: {}", stock_code, e),
                                }
                            },
                            Err(e) => error!("爬取價格失敗: {}, 錯誤: {}", stock_code, e),
                        }
                        
                        // 避免請求過於頻繁
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    },
                    Err(e) => error!("保存股票失敗: {} - {}, 錯誤: {}", stock_code, stock_name, e),
                }
            }
        },
        Err(e) => error!("爬取股票列表失敗: {}", e),
    }
    
    Ok(())
}

/// 啟動 Web 服務器
async fn start_web_server(app: Router) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    info!("API 服務器啟動在 http://0.0.0.0:3000");
    axum::serve(listener, app).await?;
    
    Ok(())
}

// 使用遷移系統確保資料庫結構存在
async fn run_migrations(database_url: &str) -> Result<(), Box<dyn Error>> {
    // 調用 database.rs 中的 run_migrations 函數
    infra::db::database::run_migrations(database_url).await
}

// 模組聲明
mod domain;
mod application;
mod infra;
mod api;
