use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::migrate::MigrateDatabase;
use std::time::Duration;
use tracing::info;

/// 創建資料庫連接池
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}

/// 運行資料庫遷移
/// 
/// 這個函數會檢查資料庫是否存在，如果不存在則創建它，
/// 然後運行 migrations 目錄中的所有遷移文件。
/// 這是管理資料庫結構的推薦方式，比直接在代碼中執行 SQL 更加可維護。
pub async fn run_migrations(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 從資料庫 URL 中提取資料庫名稱
    let db_name = database_url.split('/').last().unwrap_or("taiwan_stocks");
    
    info!("開始檢查資料庫是否存在: {}", db_name);
    
    // 檢查資料庫是否存在，如果不存在則創建
    if !sqlx::Postgres::database_exists(database_url).await? {
        info!("資料庫 {} 不存在，正在創建...", db_name);
        sqlx::Postgres::create_database(database_url).await?;
        info!("資料庫 {} 創建成功", db_name);
    } else {
        info!("資料庫 {} 已存在", db_name);
    }
    
    // 創建連接池
    info!("正在創建資料庫連接池...");
    let pool = create_pool(database_url).await?;
    info!("資料庫連接池創建成功");
    
    // 運行遷移文件
    info!("開始運行資料庫遷移...");
    
    // 使用 SQLx 的遷移系統運行所有遷移文件
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => {
            info!("資料庫遷移成功完成");
            Ok(())
        },
        Err(e) => {
            info!("資料庫遷移失敗: {:?}", e);
            Err(Box::new(e))
        }
    }
}
