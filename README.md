# 這是我個人學習 Rust 的專案

## 台灣股票爬蟲系統 (Taiwan Stocks Crawler)

這是一個基於領域驅動設計 (DDD) 的台灣股票資料爬蟲系統，專注於從台灣證券交易所和 Yahoo Finance 爬取股票資料並存儲到 PostgreSQL 資料庫中。系統採用 Rust 語言開發，具有高效能和穩定性。

## 功能特點

- **股票資料爬蟲**：爬取台灣股票市場上市公司列表和股價數據
- **股票價格歷史**：獲取股票的歷史價格數據，包括開盤價、最高價、最低價、收盤價、成交量等
- **財務指標**：獲取股票的本益比、股價淨值比、殖利率、市值等財務指標
- **三大法人買賣超**：獲取外資、投信、自營商的買賣超資訊
- **資料庫存儲**：將爬取的數據存儲到 PostgreSQL 資料庫
- **自動化數據更新**：定期自動爬取最新的股票數據
- **RESTful API**：提供 API 接口以訪問股票數據

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
  │       └── stock_price_service.rs
  │
  ├── infra/                     # 基礎設施層 - 技術實現 (簡化名稱)
  │   ├── db/                    # 資料庫相關 - 數據庫操作 (簡化名稱)
  │   │   ├── database.rs
  │   │   ├── postgres_stock_repository.rs
  │   │   └── postgres_stock_price_repository.rs
  │   └── external_services/     # 外部服務 - 與外部系統交互
  │       └── stock_crawler_service.rs
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
    
    B7 --> C[創建 API 路由]
    B7 --> D[執行爬蟲任務]
    
    D --> D1[爬取股票列表]
    D1 --> D2[保存股票到資料庫]
    D2 --> D3[爬取股票價格]
    D3 --> D4[保存價格到資料庫]
    
    C --> E[啟動 Web 服務器]
    
    subgraph 爬蟲流程
    D1
    D3 --> D3_1[爬取股票基本資訊]
    D3 --> D3_2[爬取三大法人買賣超]
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
    end
    
    A1 --> B1
    A2 --> B2
    A3 --> A1
    A3 --> A2
    
    B1 --> C1
    B1 --> C3
    B2 --> C2
    B2 --> C3
    
    C3 --> D2
    C3 --> D3
    
    D1 --> C1
    D1 --> C2
    D2 --> D4
    D3 --> D4
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
        +f64 open
        +f64 high
        +f64 low
        +f64 close
        +u64 volume
        +f64 change
        +f64 change_percent
        +u64 turnover
        +u64 transactions
        +Option~f64~ pe_ratio
        +Option~f64~ pb_ratio
        +Option~f64~ dividend_yield
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
        -extract_value_from_document(document, label): Option~f64~
    }
    
    class StockService {
        -stock_repository: Arc~dyn StockRepository~
        +new(stock_repository): StockService
        +create_stock(dto): Result~Stock~
        +get_stock_by_code(code): Result~Option~Stock~~
        +get_all_stocks(): Result~Vec~Stock~~
    }
    
    class StockPriceService {
        -price_repository: Arc~dyn StockPriceRepository~
        +new(price_repository): StockPriceService
        +create_stock_price(dto): Result~StockPrice~
        +get_stock_prices_by_stock_id(stock_id): Result~Vec~StockPrice~~
    }
    
    class StockRepository {
        <<interface>>
        +create(stock): Result~Stock~
        +find_by_code(code): Result~Option~Stock~~
        +find_all(): Result~Vec~Stock~~
    }
    
    class StockPriceRepository {
        <<interface>>
        +create(price): Result~StockPrice~
        +find_by_stock_id(stock_id): Result~Vec~StockPrice~~
    }
    
    class PostgresStockRepository {
        -pool: PgPool
        +new(pool): PostgresStockRepository
        +create(stock): Result~Stock~
        +find_by_code(code): Result~Option~Stock~~
        +find_all(): Result~Vec~Stock~~
    }
    
    class PostgresStockPriceRepository {
        -pool: PgPool
        +new(pool): PostgresStockPriceRepository
        +create(price): Result~StockPrice~
        +find_by_stock_id(stock_id): Result~Vec~StockPrice~~
    }
    
    Stock "1" -- "n" StockPrice : has
    StockRepository <|.. PostgresStockRepository : implements
    StockPriceRepository <|.. PostgresStockPriceRepository : implements
    StockService --> StockRepository : uses
    StockPriceService --> StockPriceRepository : uses
    StockCrawlerService ..> Stock : creates
    StockCrawlerService ..> StockPrice : creates
```

### 系統執行流程 (Sequence Diagram)

```mermaid
sequenceDiagram
    participant Main as 主程式
    participant System as 系統初始化
    participant Crawler as 股票爬蟲服務
    participant StockSvc as 股票服務
    participant PriceSvc as 股票價格服務
    participant DB as 資料庫
    participant Web as Web服務器
    
    Main->>System: 初始化系統
    System->>DB: 建立資料庫連接
    System->>DB: 執行資料庫遷移
    System-->>Main: 返回初始化完成
    
    Main->>Crawler: 開始爬蟲任務
    Crawler->>Crawler: 爬取股票列表
    
    loop 對每支股票
        Crawler->>StockSvc: 保存股票
        StockSvc->>DB: 儲存股票資料
        DB-->>StockSvc: 儲存成功
        
        Crawler->>Crawler: 爬取股票價格
        Crawler->>Crawler: 爬取股票基本資訊
        Crawler->>Crawler: 爬取三大法人買賣超
        
        Crawler->>StockSvc: 查詢股票ID
        StockSvc->>DB: 查詢股票
        DB-->>StockSvc: 返回股票資料
        StockSvc-->>Crawler: 返回股票ID
        
        loop 對每個價格資料
            Crawler->>PriceSvc: 保存價格
            PriceSvc->>DB: 儲存價格資料
            DB-->>PriceSvc: 儲存成功
        end
    end
    
    Main->>Web: 啟動Web服務器
    Web->>Web: 設置API路由
    Web-->>Main: 服務器啟動成功
```

### 系統組件圖 (Component Diagram)

```mermaid
flowchart LR
    subgraph 外部系統
        E1[台灣證券交易所]
        E2[公開資訊觀測站]
    end
    
    subgraph 爬蟲系統
        C1[股票爬蟲服務]
        C2[資料處理服務]
        C3[資料儲存服務]
        C4[Web API 服務]
        
        C1 --> C2
        C2 --> C3
        C3 --> C4
    end
    
    subgraph 資料庫
        D1[(PostgreSQL)]
    end
    
    E1 --> C1
    E2 --> C1
    C3 --> D1
    C4 --> D1
```

## 使用方法

### 環境設置

1. 確保已安裝 Rust 和 Cargo (建議使用 rustup)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. 安裝 PostgreSQL 資料庫
   ```bash
   # macOS
   brew install postgresql
   
   # Ubuntu
   sudo apt install postgresql
   ```

3. 創建資料庫
   ```bash
   createdb taiwan_stocks
   ```

4. 創建 `.env` 文件並設置資料庫連接字串：
   ```
   DATABASE_URL=postgres://username:password@localhost:5432/taiwan_stocks
   ```
   請將 `username` 和 `password` 替換為您的 PostgreSQL 用戶名和密碼。

### 編譯和運行

1. 克隆代碼庫
   ```bash
   git clone https://github.com/yourusername/taiwan-stocks-crawler.git
   cd taiwan-stocks-crawler
   ```

2. 編譯程式
   ```bash
   cargo build --release
   ```

3. 運行程式
   ```bash
   cargo run --release
   ```

程式會自動：
1. 檢查並創建必要的資料庫表結構
2. 爬取台灣熱門股票列表 (台積電、鴻海、聯發科、中華電、台達電)
3. 爬取每支股票的歷史價格數據、財務指標和三大法人買賣超資訊
4. 將數據存儲到資料庫中
5. 啟動 API 服務器，提供數據訪問接口

### API 端點

- `GET /api/stocks` - 獲取所有股票列表
- `GET /api/stocks/{code}` - 獲取特定股票詳情
- `GET /api/stocks/{code}/prices` - 獲取特定股票的價格歷史
- `GET /api/stocks/{code}/prices/latest` - 獲取特定股票的最新價格

## 數據模型

### 股票 (Stock)

| 欄位 | 類型 | 描述 |
|------|------|------|
| id | UUID | 唯一識別碼 |
| code | VARCHAR | 股票代碼 (如 2330) |
| name | VARCHAR | 股票名稱 (如 台積電) |
| last_updated | TIMESTAMP | 最後更新時間 |

### 股票價格 (StockPrice)

| 欄位 | 類型 | 描述 |
|------|------|------|
| id | UUID | 唯一識別碼 |
| stock_id | UUID | 股票 ID (外鍵) |
| date | DATE | 日期 |
| open | NUMERIC | 開盤價 |
| high | NUMERIC | 最高價 |
| low | NUMERIC | 最低價 |
| close | NUMERIC | 收盤價 |
| volume | NUMERIC | 成交量 |
| change | NUMERIC | 漲跌 |
| change_percent | NUMERIC | 漲跌幅 (%) |
| turnover | NUMERIC | 成交金額 |
| transactions | INTEGER | 成交筆數 |
| pe_ratio | NUMERIC | 本益比 |
| pb_ratio | NUMERIC | 股價淨值比 |
| dividend_yield | NUMERIC | 殖利率 (%) |
| market_cap | NUMERIC | 市值 |
| foreign_buy | NUMERIC | 外資買賣超 |
| trust_buy | NUMERIC | 投信買賣超 |
| dealer_buy | NUMERIC | 自營商買賣超 |

## 擴展功能

如需擴展功能，可以考慮：

1. **定時任務**：使用 cron 或 systemd timer 定期爬取數據
2. **數據分析**：添加股票技術指標計算 (如 MA、RSI、MACD 等)
3. **通知機制**：實現價格變動提醒 (如透過 Telegram、Email)
4. **用戶認證**：添加 JWT 認證以保護 API
5. **前端界面**：開發 Web 前端或移動應用以可視化數據
6. **回測系統**：實現交易策略回測功能

## 故障排除

### 常見問題

1. **資料庫連接失敗**
   - 確認 PostgreSQL 服務已啟動
   - 檢查 `.env` 文件中的連接字串是否正確
   - 確認資料庫用戶有適當的權限

2. **爬蟲失敗**
   - 檢查網絡連接
   - 確認目標網站結構未變更
   - 查看日誌以獲取詳細錯誤信息

## 貢獻指南

歡迎貢獻代碼、報告問題或提出功能建議。請遵循以下步驟：

1. Fork 代碼庫
2. 創建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 創建 Pull Request

## 授權

MIT

## 更新日誌

### v0.1.0 
- 初始版本發布
- 實現基本的股票爬蟲功能
- 添加股票價格、財務指標和三大法人買賣超數據爬取
- 實現 PostgreSQL 數據存儲
- 添加 RESTful API 接口
