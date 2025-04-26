// 引入必要的外部庫
use crate::domain::entities::{Stock, StockPrice};
use crate::domain::value_objects::date_range::DateRange;
use anyhow::{anyhow, Result};
use reqwest::Client;
use scraper::{Html, Selector, Element, ElementRef};
use time::{Date, Month, OffsetDateTime};
use tracing::{info, warn, error};
use uuid::Uuid;
use std::collections::HashMap;
use std::str::FromStr;

// 股票爬蟲服務結構體
/// 股票爬蟲服務，用於爬取股票列表和股票價格數據
pub struct StockCrawlerService {
    // HTTP 客戶端，用於發送網絡請求
    client: Client,
}

impl StockCrawlerService {
    // 創建新的股票爬蟲服務實例
    /// 創建新的股票爬蟲服務實例
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
    
    // 爬取台灣股票市場的股票列表
    /// 爬取台灣股票市場的股票列表
    pub async fn crawl_stocks(&self) -> Result<Vec<Stock>> {
        info!("開始爬取股票列表...");
        
        // 定義熱門股票列表（硬編碼）
        let popular_stocks = vec![
            ("2330", "台積電"),
            ("2317", "鴻海"),
            ("2454", "聯發科"),
            ("2412", "中華電"),
            ("2308", "台達電"),
        ];
        
        // 創建股票實體列表
        let mut stocks = Vec::new();
        
        // 為每個熱門股票創建股票實體
        for (code, name) in popular_stocks {
            let stock = Stock {
                id: Uuid::new_v4(),  // 生成唯一識別碼
                code: code.to_string(),  // 股票代碼
                name: name.to_string(),  // 股票名稱
                last_updated: OffsetDateTime::now_utc(),  // 最後更新時間
            };
            
            info!("找到股票: {} - {}", stock.code, stock.name);
            stocks.push(stock);
        }
        
        Ok(stocks)
    }
    
    // 爬取特定股票的歷史價格數據
    /// 爬取特定股票的歷史價格數據
    pub async fn crawl_stock_prices(&self, stock_code: &str) -> Result<Vec<StockPrice>> {
        info!("開始爬取股票 {} 的價格資料...", stock_code);
        
        // 檢查股票代碼是否在我們的硬編碼列表中
        let popular_stocks = vec![
            "2330", "2317", "2454", "2412", "2308"
        ];
        
        if !popular_stocks.contains(&stock_code) {
            info!("股票 {} 不在硬編碼列表中，跳過爬取", stock_code);
            return Ok(Vec::new());
        }
        
        // 取得當前年月
        let now = OffsetDateTime::now_utc();
        let year = now.year();
        let month = now.month() as u8;
        
        // 構建 Yahoo Finance 歷史數據 URL
        let url = format!(
            "https://tw.stock.yahoo.com/quote/{}.TW/history?period=1mo",
            stock_code
        );
        info!("爬取價格資料 URL: {}", url);
        
        // 發送 HTTP 請求並獲取響應
        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        
        // 解析 HTML 文檔
        let document = Html::parse_document(&html);
        
        // 選擇表格行
        let row_selector = Selector::parse("div[class='Pb(10px)'] table tbody tr").unwrap();
        
        // 爬取股票基本資訊（本益比、股價淨值比、殖利率、市值等）
        let stock_info = self.crawl_stock_info(stock_code).await?;
        
        // 爬取三大法人買賣超資訊
        let institutional_investors = self.crawl_institutional_investors(stock_code).await?;
        
        // 創建股票價格列表
        let mut prices = Vec::new();
        
        // 遍歷表格行
        for row in document.select(&row_selector) {
            let cells: Vec<_> = row.select(&Selector::parse("td").unwrap()).collect();
            
            if cells.len() >= 6 {
                // 解析日期
                let date_text = cells[0].text().collect::<Vec<_>>().join("");
                let date_parts: Vec<&str> = date_text.trim().split('/').collect();
                
                if date_parts.len() != 3 {
                    continue;
                }
                
                let year = date_parts[0].parse::<i32>().unwrap_or(year);
                let month = date_parts[1].parse::<u8>().unwrap_or(month);
                let day = date_parts[2].parse::<u8>().unwrap_or(1);
                
                // 創建日期對象
                let date = Date::from_calendar_date(year, Month::try_from(month).unwrap_or(Month::January), day).unwrap_or_else(|_| {
                    Date::from_calendar_date(year, Month::January, 1).unwrap()
                });
                
                // 解析開盤價、最高價、最低價、收盤價
                let open = self.parse_float_from_text(cells[1].text().collect::<Vec<_>>().join("").trim());
                let high = self.parse_float_from_text(cells[2].text().collect::<Vec<_>>().join("").trim());
                let low = self.parse_float_from_text(cells[3].text().collect::<Vec<_>>().join("").trim());
                let close = self.parse_float_from_text(cells[4].text().collect::<Vec<_>>().join("").trim());
                
                // 解析成交量
                let volume_text = cells[5].text().collect::<Vec<_>>().join("").trim().replace(",", "");
                let volume = volume_text.parse::<i64>().unwrap_or(0);
                
                // 計算漲跌幅
                let change = if prices.is_empty() { 0.0 } else { close - prices.last().unwrap().close };
                let change_percent = if prices.is_empty() || prices.last().unwrap().close == 0.0 {
                    0.0
                } else {
                    change / prices.last().unwrap().close * 100.0
                };
                
                // 從股票基本資訊中獲取其他數據
                let date_str = format!("{}-{:02}-{:02}", year, month, day);
                let turnover = 0; // 成交金額，此處簡化處理
                let transactions = 0; // 成交筆數，此處簡化處理
                
                // 從爬取的基本資訊中獲取本益比、股價淨值比、殖利率、市值
                let pe_ratio = stock_info.get("pe_ratio").and_then(|v| *v);
                let pb_ratio = stock_info.get("pb_ratio").and_then(|v| *v);
                let dividend_yield = stock_info.get("dividend_yield").and_then(|v| *v);
                let market_cap = stock_info.get("market_cap").and_then(|v| *v).map(|v| v as i64).unwrap_or(0);
                
                // 從三大法人買賣超資訊中獲取外資、投信、自營商買賣超
                let (foreign_buy, trust_buy, dealer_buy) = institutional_investors
                    .get(&date_str)
                    .cloned()
                    .unwrap_or((0, 0, 0));
                
                // 創建股票價格實體
                let stock_price = StockPrice::new(
                    Uuid::new_v4(),
                    Uuid::nil(), // 暫時使用空 UUID，後續會更新
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
                );
                
                prices.push(stock_price);
            }
        }
        
        info!("成功爬取股票 {} 的 {} 筆價格資料", stock_code, prices.len());
        Ok(prices)
    }
    
    // 爬取股票基本資訊（本益比、股價淨值比、殖利率、市值等）
    /// 爬取股票基本資訊（本益比、股價淨值比、殖利率、市值等）
    async fn crawl_stock_info(&self, stock_code: &str) -> Result<HashMap<String, Option<f64>>> {
        info!("開始爬取股票 {} 的基本資訊...", stock_code);
        
        // 檢查股票代碼是否在我們的硬編碼列表中
        let popular_stocks = vec![
            "2330", "2317", "2454", "2412", "2308"
        ];
        
        if !popular_stocks.contains(&stock_code) {
            info!("股票 {} 不在硬編碼列表中，跳過爬取基本資訊", stock_code);
            return Ok(HashMap::new());
        }
        
        let url = format!("https://tw.stock.yahoo.com/quote/{}.TW", stock_code);
        info!("爬取基本資訊 URL: {}", url);
        
        // 發送 HTTP 請求並獲取響應
        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        
        // 解析 HTML 文檔
        let document = Html::parse_document(&html);
        
        // 創建結果 HashMap
        let mut result = HashMap::new();
        
        // 解析本益比
        let pe_ratio = self.extract_value_from_document(&document, "本益比");
        result.insert("pe_ratio".to_string(), pe_ratio);
        
        // 解析股價淨值比
        let pb_ratio = self.extract_value_from_document(&document, "股價淨值比");
        result.insert("pb_ratio".to_string(), pb_ratio);
        
        // 解析殖利率
        let dividend_yield = self.extract_value_from_document(&document, "殖利率");
        result.insert("dividend_yield".to_string(), dividend_yield);
        
        // 解析市值
        let market_cap = self.extract_value_from_document(&document, "市值");
        result.insert("market_cap".to_string(), market_cap);
        
        info!("成功爬取股票 {} 的基本資訊", stock_code);
        Ok(result)
    }
    
    // 從 HTML 文檔中提取特定標籤的值
    /// 從 HTML 文檔中提取特定標籤的值
    fn extract_value_from_document(&self, document: &Html, label: &str) -> Option<f64> {
        // 選擇包含標籤的元素
        let label_selector = Selector::parse(&format!("div:contains(\"{}\")", label)).unwrap();
        
        // 遍歷所有匹配的元素
        for element in document.select(&label_selector) {
            let text = element.text().collect::<Vec<_>>().join("");
            if text.contains(label) {
                // 獲取父元素
                if let Some(parent) = element.parent() {
                    if let Some(parent_element) = parent.value().as_element() {
                        // 選擇值元素
                        let value_selector = Selector::parse("span").unwrap();
                        for value_element in element.select(&value_selector) {
                            let value_text = value_element.text().collect::<Vec<_>>().join("");
                            // 解析數值
                            return self.parse_float_from_text(value_text.trim());
                        }
                    }
                }
            }
        }
        
        None
    }
    
    // 爬取三大法人買賣超資訊
    /// 爬取三大法人買賣超資訊
    async fn crawl_institutional_investors(&self, stock_code: &str) -> Result<HashMap<String, (i64, i64, i64)>> {
        info!("開始爬取股票 {} 的三大法人買賣超資訊...", stock_code);
        
        // 檢查股票代碼是否在我們的硬編碼列表中
        let popular_stocks = vec![
            "2330", "2317", "2454", "2412", "2308"
        ];
        
        if !popular_stocks.contains(&stock_code) {
            info!("股票 {} 不在硬編碼列表中，跳過爬取三大法人資訊", stock_code);
            return Ok(HashMap::new());
        }
        
        // 取得當前年月
        let now = OffsetDateTime::now_utc();
        let year = now.year();
        let month = now.month() as u8;
        
        // 構建 Yahoo Finance 三大法人買賣超 URL
        let url = format!(
            "https://tw.stock.yahoo.com/quote/{}.TW/institutional-trading",
            stock_code
        );
        info!("爬取三大法人買賣超資訊 URL: {}", url);
        
        // 發送 HTTP 請求並獲取響應
        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        
        // 解析 HTML 文檔
        let document = Html::parse_document(&html);
        
        // 選擇表格行
        let row_selector = Selector::parse("div[class='Pb(10px)'] table tbody tr").unwrap();
        
        // 創建結果 HashMap
        let mut result = HashMap::new();
        
        // 遍歷表格行
        for row in document.select(&row_selector) {
            let cells: Vec<_> = row.select(&Selector::parse("td").unwrap()).collect();
            
            if cells.len() >= 4 {
                // 解析日期
                let date_text = cells[0].text().collect::<Vec<_>>().join("");
                let date_parts: Vec<&str> = date_text.trim().split('/').collect();
                
                if date_parts.len() != 3 {
                    continue;
                }
                
                let year = date_parts[0].parse::<i32>().unwrap_or(year);
                let month = date_parts[1].parse::<u8>().unwrap_or(month);
                let day = date_parts[2].parse::<u8>().unwrap_or(1);
                
                // 解析外資買賣超
                let foreign_text = cells[1].text().collect::<Vec<_>>().join("").trim().replace(",", "");
                let foreign_buy = foreign_text.parse::<i64>().unwrap_or(0);
                
                // 解析投信買賣超
                let trust_text = cells[2].text().collect::<Vec<_>>().join("").trim().replace(",", "");
                let trust_buy = trust_text.parse::<i64>().unwrap_or(0);
                
                // 解析自營商買賣超
                let dealer_text = cells[3].text().collect::<Vec<_>>().join("").trim().replace(",", "");
                let dealer_buy = dealer_text.parse::<i64>().unwrap_or(0);
                
                // 構建日期字符串作為 key
                let date_str = format!("{}-{:02}-{:02}", year, month, day);
                
                // 只處理目標股票
                if code != stock_code {
                    continue;
                }
                
                // 存儲結果
                result.insert(date_str, (foreign_buy, trust_buy, dealer_buy));
            }
        }
        
        info!("成功爬取股票 {} 的三大法人買賣超資訊", stock_code);
        Ok(result)
    }
    
    // 從文本中解析浮點數
    /// 從文本中解析浮點數
    fn parse_float_from_text(&self, text: &str) -> Option<f64> {
        // 移除千分位逗號和其他非數字字符
        let cleaned_text = text.replace(",", "").replace("%", "");
        
        // 嘗試解析為浮點數
        cleaned_text.parse::<f64>().ok()
    }
}
