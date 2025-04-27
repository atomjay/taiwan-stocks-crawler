# 這是我個人學習 Rust 的專案

## 台灣股票爬蟲系統 (Taiwan Stocks Crawler)

這是一個基於領域驅動設計 (DDD) 的台灣股票資料爬蟲系統，專注於從台灣證券交易所和 Yahoo Finance 爬取股票資料並存儲到 PostgreSQL 資料庫中。系統採用 Rust 語言開發，具有高效能和穩定性，並支援 LINE 通知功能，確保中文字符正確顯示。

## 功能特點

- **股票資料爬蟲**：爬取台灣股票市場上市公司列表和股價數據
- **股票價格歷史**：獲取股票的歷史價格數據，包括開盤價、最高價、最低價、收盤價、成交量等
- **財務指標**：獲取股票的本益比、股價淨值比、殖利率、市值等財務指標
- **三大法人買賣超**：獲取外資、投信、自營商的買賣超資訊
- **資料庫存儲**：將爬取的數據存儲到 PostgreSQL 資料庫，使用 ON CONFLICT 實現 upsert 操作
- **自動化數據更新**：定期自動爬取最新的股票數據
- **RESTful API**：提供 API 接口以訪問股票數據
- **LINE 通知**：支援透過 LINE Bot 發送股票價格和每日摘要通知，確保中文字符正確顯示
- **精確數值計算**：使用 BigDecimal 處理金融數據，確保計算精度

## 技術棧

- **語言**：Rust 2024 Edition
- **資料庫**：PostgreSQL 與 SQLx
- **HTTP 客戶端**：Reqwest
- **HTML 解析**：Scraper
- **序列化/反序列化**：Serde
- **Web 框架**：Axum
- **日誌**：Tracing
- **非同步運行時**：Tokio
- **日期時間處理**：Time
- **錯誤處理**：Anyhow
- **精確數值**：BigDecimal
- **字符編碼**：Encoding_rs (處理 BIG5 編碼)
- **通知服務**：LINE Messaging API

## 系統架構

本專案採用領域驅動設計 (DDD) 架構，分為四個主要層次，並經過重構以簡化結構：

```
src/
  ├── domain/                    # 領域層 - 核心業務邏輯和規則
  │   ├── models/                # 實體模型 - 具有唯一標識的對象
  │   │   ├── stock.rs           # 股票實體
  │   │   └── stock_price.rs     # 股價實體
  │   ├── repositories/          # 儲存庫接口 - 定義數據存取方法
  │   │   ├── stock_repository.rs
  │   │   └── stock_price_repository.rs
  │   └── value_objects/         # 值對象 - 無唯一標識的對象
  │       └── date_range.rs
  │
  ├── application/               # 應用層 - 協調領域對象完成用戶任務
  │   ├── dtos/                  # 數據傳輸對象 - 跨層數據傳輸
  │   │   ├── stock_dto.rs
  │   │   └── stock_price_dto.rs
  │   └── services/              # 應用服務 - 實現用例
  │       ├── stock_service.rs
  │       ├── stock_price_service.rs
  │       └── notification_service.rs  # 通知服務
  │
  ├── infra/                     # 基礎設施層 - 技術實現 (簡化名稱)
  │   ├── db/                    # 資料庫相關 - 數據庫操作 (簡化名稱)
  │   │   ├── database.rs
  │   │   ├── postgres_stock_repository.rs
  │   │   └── postgres_stock_price_repository.rs  # 實現 upsert 操作
  │   └── external_services/     # 外部服務 - 與外部系統交互
  │       ├── stock_crawler_service.rs  # 處理 BIG5 編碼
  │       └── line_notification_service.rs  # LINE 通知服務，處理 UTF-8 編碼
  │
  ├── api/                       # API 層 - 用戶界面 (簡化名稱)
  │   ├── controllers/           # 控制器 - 處理請求和響應
  │   │   ├── stock_controller.rs
  │   │   └── stock_price_controller.rs
  │   └── routes.rs              # API 路由 - 定義 API 端點 (簡化結構)
  │
  └── main.rs                    # 主程式入口點
```

## 系統流程圖與 UML 圖表

### 系統流程圖 (Mermaid Flowchart)

```mermaid
flowchart TD
    A[程式啟動] --> B[初始化系統]
    B --> B1[設置日誌系統]
    B --> B2[載入環境變數]
    B --> B3[建立資料庫連接池]
    B --> B4[執行資料庫遷移]
    B --> B5[初始化儲存庫]
    B --> B6[初始化應用服務]
    B --> B7[初始化控制器]
    B --> B8[初始化通知服務]
    
    B7 --> C[創建 API 路由]
    B7 --> D[執行爬蟲任務]
    
    D --> D1[爬取股票列表]
    D1 --> D2[保存股票到資料庫]
    D2 --> D3[爬取股票價格]
    D3 --> D4[保存價格到資料庫]
    D4 --> D5[發送通知]
    
    C --> E[啟動 Web 服務器]
    
    subgraph 爬蟲流程
    D1
    D3 --> D3_1[爬取股票基本資訊]
    D3 --> D3_2[爬取三大法人買賣超]
    end
    
    subgraph 通知流程
    D5 --> D5_1[發送初始通知]
    D5 --> D5_2[發送每日摘要]
    D5 --> D5_3[發送股價變動通知]
    end
```

### 系統架構圖 (Mermaid Flowchart)

```mermaid
flowchart TB
    subgraph 表現層 - API
        A1[Stock Controller]
        A2[Stock Price Controller]
        A3[API 路由]
    end
    
    subgraph 應用層 - Application
        B1[Stock Service]
        B2[Stock Price Service]
        B3[DTOs]
        B4[Notification Service]
    end
    
    subgraph 領域層 - Domain
        C1[Stock 實體]
        C2[Stock Price 實體]
        C3[儲存庫介面]
    end
    
    subgraph 基礎設施層 - Infrastructure
        D1[Stock Crawler Service]
        D2[Postgres Stock Repository]
        D3[Postgres Stock Price Repository]
        D4[資料庫連接]
        D5[LINE Notification Service]
    end
    
    A1 --> B1
    A2 --> B2
    A3 --> A1
    A3 --> A2
    
    B1 --> C1
    B1 --> C3
    B2 --> C2
    B2 --> C3
    B4 --> D5
    
    C3 --> D2
    C3 --> D3
    
    D1 --> C1
    D1 --> C2
    D2 --> D4
    D3 --> D4
    D5 --> B4
```

### 類別圖 (UML Class Diagram)

```mermaid
classDiagram
    class Stock {
        +Uuid id
        +String code
        +String name
        +OffsetDateTime last_updated
        +new(code, name): Stock
    }
    
    class StockPrice {
        +Uuid id
        +Uuid stock_id
        +Date date
        +BigDecimal open
        +BigDecimal high
        +BigDecimal low
        +BigDecimal close
        +u64 volume
        +BigDecimal change
        +BigDecimal change_percent
        +u64 turnover
        +u64 transactions
        +Option~BigDecimal~ pe_ratio
        +Option~BigDecimal~ pb_ratio
        +Option~BigDecimal~ dividend_yield
        +Option~u64~ market_cap
        +Option~i64~ foreign_buy
        +Option~i64~ trust_buy
        +Option~i64~ dealer_buy
        +new(...): StockPrice
        +with_details(...): StockPrice
        +calculate_change(prev_close): void
    }
    
    class StockCrawlerService {
        +new(): StockCrawlerService
        +crawl_stocks(): Result~Vec~Stock~~
        +crawl_stock_prices(stock_code): Result~Vec~StockPrice~~
        +crawl_stock_info(stock_code): Result~HashMap~String, f64~~
        +crawl_institutional_investors(stock_code): Result~HashMap~String, (i64, i64, i64)~~
        -parse_float_from_text(text): Option~f64~
        -parse_bigdecimal_from_text(text): Option~BigDecimal~
        -extract_value_from_document(document, label): Option~f64~
    }
    
    class LineNotificationService {
        -client: Client
        -channel_access_token: String
        -user_id: String
        +new(channel_access_token, user_id): LineNotificationService
        +send_stock_price_notification(stock, price): Result~()~
        +send_daily_summary(date, stocks): Result~()~
        +send_custom_message(text): Result~()~
        -send_push_message(user_id, message): Result~()~
        -build_stock_price_message(stock, price): Value
        -build_daily_summary_message(date, stocks): Value
        -utf8_encode(text): String  # 確保中文字符正確編碼
        -format_number(number): String
    }
    
    class StockService {
        -stock_repository: Arc~dyn StockRepository~
        +new(stock_repository): StockService
        +create_stock(dto): Result~Stock~
        +get_all_stocks(): Result~Vec~StockDto~~
        +get_stock_by_id(id): Result~Option~StockDto~~
        +get_stock_by_code(code): Result~Option~StockDto~~
        +update_stock(id, dto): Result~Stock~
        +delete_stock(id): Result~()~
    }
    
    class StockPriceService {
        -stock_price_repository: Arc~dyn StockPriceRepository~
        -stock_repository: Arc~dyn StockRepository~
        +new(stock_price_repository, stock_repository): StockPriceService
        +create_stock_price(dto): Result~StockPrice~
        +get_stock_price_by_id(id): Result~Option~StockPriceDto~~
        +get_stock_prices_by_stock_id(stock_id): Result~Vec~StockPriceDto~~
        +get_stock_prices_by_stock_code(code): Result~Vec~StockPriceDto~~
        +get_stock_prices_by_date_range(stock_id, start_date, end_date): Result~Vec~StockPriceDto~~
        +get_latest_stock_price(stock_id): Result~Option~StockPriceDto~~
    }
    
    class NotificationService {
        -line_service: Arc~LineNotificationService~
        -stock_price_service: Arc~StockPriceService~
        -stock_service: Arc~StockService~
        +new(line_service, stock_price_service, stock_service): NotificationService
        +send_stock_price_notification(stock_id): Result~()~
        +send_daily_summary(): Result~()~
        +send_custom_message(text): Result~()~
    }
    
    StockPrice "many" --> "1" Stock : belongs to
    StockService --> "uses" StockRepository
    StockPriceService --> "uses" StockPriceRepository
    StockPriceService --> "uses" StockRepository
    NotificationService --> "uses" LineNotificationService
    NotificationService --> "uses" StockPriceService
    NotificationService --> "uses" StockService
```

### 序列圖 (Sequence Diagram)

```mermaid
sequenceDiagram
    participant Main as 主程式
    participant System as 系統初始化
    participant Crawler as 股票爬蟲服務
    participant StockSvc as 股票服務
    participant PriceSvc as 股價服務
    participant NotifSvc as 通知服務
    participant LineNotif as LINE通知服務
    participant DB as 資料庫
    
    Main->>System: 啟動系統
    System->>DB: 初始化資料庫連接
    System->>Crawler: 初始化爬蟲服務
    System->>StockSvc: 初始化股票服務
    System->>PriceSvc: 初始化股價服務
    System->>NotifSvc: 初始化通知服務
    System->>LineNotif: 初始化LINE通知服務
    
    Main->>Crawler: 執行爬蟲任務
    Crawler->>Crawler: 爬取股票列表
    Crawler->>StockSvc: 保存股票數據
    StockSvc->>DB: 儲存股票
    
    loop 對每支股票
        Crawler->>Crawler: 爬取股票價格
        Crawler->>Crawler: 爬取基本資訊
        Crawler->>Crawler: 爬取三大法人買賣超
        Crawler->>PriceSvc: 保存股價數據
        PriceSvc->>DB: 儲存股價
    end
    
    Main->>NotifSvc: 發送初始通知
    NotifSvc->>LineNotif: 發送自訂訊息
    LineNotif-->>NotifSvc: 通知結果
    
    Main->>NotifSvc: 發送每日摘要
    NotifSvc->>StockSvc: 獲取所有股票
    StockSvc->>DB: 查詢股票
    DB-->>StockSvc: 股票列表
    StockSvc-->>NotifSvc: 股票列表
    NotifSvc->>PriceSvc: 獲取最新價格
    PriceSvc->>DB: 查詢價格
    DB-->>PriceSvc: 價格數據
    PriceSvc-->>NotifSvc: 價格數據
    NotifSvc->>LineNotif: 發送每日摘要
    LineNotif-->>NotifSvc: 通知結果
    
    Main->>System: 啟動API服務器
```

## 環境設置

### 前置需求

- Rust 2024 Edition
- PostgreSQL 15+
- LINE Messaging API 帳號 (用於通知功能)

### 環境變數

創建 `.env` 文件並設置以下環境變數：

```
DATABASE_URL=postgres://username:password@localhost:5432/taiwan_stocks
LINE_CHANNEL_ACCESS_TOKEN=your_line_channel_access_token
LINE_USER_ID=your_line_user_id
```

### 資料庫設置

1. 創建 PostgreSQL 資料庫：

```sql
CREATE DATABASE taiwan_stocks;
```

2. 運行遷移腳本：

```bash
cargo run --bin migrate
```

## 運行

```bash
cargo run
```

## 主要功能

### 股票爬蟲

系統會自動爬取台灣證券交易所的股票列表和價格數據，並存儲到資料庫中。爬蟲服務會處理中文編碼問題，使用 BIG5 編碼解析網頁內容，確保正確顯示股票名稱。

### LINE 通知

系統支援透過 LINE Bot 發送以下類型的通知：

1. **初始通知**：系統啟動時發送
2. **每日摘要**：包含漲幅前5名和跌幅前5名的股票
3. **股價變動通知**：當股票價格發生顯著變化時發送

所有通知都經過 UTF-8 編碼處理，確保中文字符（如股票名稱）能夠正確顯示，避免出現亂碼（如 `�x�d`）。

### 數據處理

系統使用 BigDecimal 處理金融數據，確保計算精度，避免浮點數計算誤差。在與 PostgreSQL 數據庫交互時，系統會正確處理數值類型的轉換。

### 資料庫操作

系統使用 ON CONFLICT 子句實現 upsert 操作，允許在保存股票價格時更新現有記錄，而不是因為唯一約束違反而導致錯誤。這確保了數據的一致性和完整性。

## 常見問題

1. **資料庫連接失敗**
   - 確認 PostgreSQL 服務已啟動
   - 檢查 `.env` 文件中的連接字串是否正確
   - 確認資料庫用戶有適當的權限

2. **爬蟲失敗**
   - 檢查網絡連接
   - 確認目標網站是否更改了 HTML 結構
   - 調整爬蟲服務中的選擇器

3. **LINE 通知失敗**
   - 確認 LINE Channel Access Token 是否有效
   - 確認 LINE User ID 是否正確
   - 檢查 LINE Messaging API 的配額限制

4. **中文亂碼問題**
   - 系統使用 BIG5 編碼處理台灣網站的中文字符
   - 確保數據庫使用 UTF-8 編碼
   - 通知服務中使用 utf8_encode 方法確保中文正確顯示在 LINE 通知中
   - 如果仍有亂碼問題，檢查爬蟲服務中的字符編碼處理邏輯

## 未來計劃

- [ ] 增加更多股票資訊來源
- [ ] 實現股票價格預測功能
- [ ] 增加更多通知渠道 (Email, Telegram 等)
- [ ] 優化爬蟲效率和穩定性
- [ ] 增加用戶界面 (Web UI)
- [ ] 改進字符編碼處理，支持更多語言和編碼格式

## 貢獻

歡迎提交 Issue 和 Pull Request。

## 許可證

MIT
