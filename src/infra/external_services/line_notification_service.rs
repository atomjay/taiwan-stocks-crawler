use crate::domain::models::{Stock, StockPrice};
use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use tracing::{info, error};
use time::Date;
use bigdecimal::BigDecimal;

/// LINE 通知服務，用於發送股票相關通知到 LINE Bot
pub struct LineNotificationService {
    client: Client,
    channel_access_token: String,
    pub user_id: String,
}

impl LineNotificationService {
    /// 創建新的 LINE 通知服務實例
    pub fn new(channel_access_token: String, user_id: String) -> Self {
        Self {
            client: Client::new(),
            channel_access_token,
            user_id,
        }
    }

    /// 發送股票價格通知到 LINE
    pub async fn send_stock_price_notification(
        &self, 
        stock: &Stock, 
        price: &StockPrice
    ) -> Result<()> {
        info!("發送股票價格通知到 LINE: {} {}", stock.code, stock.name);
        
        let message = self.build_stock_price_message(stock, price);
        self.send_push_message(&self.user_id, &message).await
    }

    /// 發送每日股票摘要通知到 LINE
    pub async fn send_daily_summary(
        &self, 
        date: Date, 
        stocks: Vec<(Stock, StockPrice)>
    ) -> Result<()> {
        info!("發送每日股票摘要通知到 LINE");
        
        let message = self.build_daily_summary_message(date, &stocks);
        self.send_push_message(&self.user_id, &message).await
    }

    /// 發送自訂訊息到 LINE
    pub async fn send_custom_message(&self, text: &str) -> Result<()> {
        info!("發送自訂訊息到 LINE: {}", text);
        
        let message = json!({
            "type": "text",
            "text": text
        });
        
        self.send_push_message(&self.user_id, &message).await
    }

    /// 發送 Push 訊息到 LINE
    async fn send_push_message(&self, user_id: &str, message: &serde_json::Value) -> Result<()> {
        let url = "https://api.line.me/v2/bot/message/push";
        
        let payload = json!({
            "to": user_id,
            "messages": [message]
        });

        info!("發送 LINE 訊息");        
        let response = self.client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.channel_access_token))
            .json(&payload)
            .send()
            .await?;
        
        if response.status().is_success() {
            info!("LINE 訊息發送成功");
            Ok(())
        } else {
            let status = response.status();
            let error_body = response.text().await?;
            error!("LINE 訊息發送失敗: {} - {}", status, error_body);
            Err(anyhow::anyhow!("LINE 訊息發送失敗: {}", error_body))
        }
    }

    /// 構建股票價格訊息
    fn build_stock_price_message(&self, stock: &Stock, price: &StockPrice) -> serde_json::Value {
        // 計算漲跌顏色
        let zero = BigDecimal::from(0);
        let color = if price.change >= zero { "#FF0000" } else { "#00FF00" };
        
        // 構建 Flex Message
        json!({
            "type": "flex",
            "altText": format!("股票價格通知: {} {}", stock.code, stock.name),
            "contents": {
                "type": "bubble",
                "header": {
                    "type": "box",
                    "layout": "vertical",
                    "contents": [
                        {
                            "type": "text",
                            "text": format!("{} {}", stock.code, self.utf8_encode(&stock.name)),
                            "weight": "bold",
                            "size": "xl",
                            "color": "#000000"
                        }
                    ],
                    "backgroundColor": "#FFFFFF"
                },
                "body": {
                    "type": "box",
                    "layout": "vertical",
                    "contents": [
                        {
                            "type": "box",
                            "layout": "horizontal",
                            "contents": [
                                {
                                    "type": "text",
                                    "text": "收盤價",
                                    "weight": "bold",
                                    "size": "md",
                                    "flex": 1
                                },
                                {
                                    "type": "text",
                                    "text": format!("{}", price.close),
                                    "weight": "bold",
                                    "size": "md",
                                    "color": color,
                                    "align": "end",
                                    "flex": 1
                                }
                            ]
                        },
                        {
                            "type": "box",
                            "layout": "horizontal",
                            "contents": [
                                {
                                    "type": "text",
                                    "text": "漲跌",
                                    "weight": "bold",
                                    "size": "md",
                                    "flex": 1
                                },
                                {
                                    "type": "text",
                                    "text": format!("{} ({}%)", price.change, price.change_percent),
                                    "weight": "bold",
                                    "size": "md",
                                    "color": color,
                                    "align": "end",
                                    "flex": 1
                                }
                            ],
                            "margin": "md"
                        },
                        {
                            "type": "separator",
                            "margin": "md"
                        },
                        {
                            "type": "box",
                            "layout": "horizontal",
                            "contents": [
                                {
                                    "type": "text",
                                    "text": "開盤價",
                                    "size": "sm",
                                    "flex": 1
                                },
                                {
                                    "type": "text",
                                    "text": format!("{}", price.open),
                                    "size": "sm",
                                    "align": "end",
                                    "flex": 1
                                }
                            ],
                            "margin": "md"
                        },
                        {
                            "type": "box",
                            "layout": "horizontal",
                            "contents": [
                                {
                                    "type": "text",
                                    "text": "最高價",
                                    "size": "sm",
                                    "flex": 1
                                },
                                {
                                    "type": "text",
                                    "text": format!("{}", price.high),
                                    "size": "sm",
                                    "align": "end",
                                    "flex": 1
                                }
                            ],
                            "margin": "md"
                        },
                        {
                            "type": "box",
                            "layout": "horizontal",
                            "contents": [
                                {
                                    "type": "text",
                                    "text": "最低價",
                                    "size": "sm",
                                    "flex": 1
                                },
                                {
                                    "type": "text",
                                    "text": format!("{}", price.low),
                                    "size": "sm",
                                    "align": "end",
                                    "flex": 1
                                }
                            ],
                            "margin": "md"
                        },
                        {
                            "type": "box",
                            "layout": "horizontal",
                            "contents": [
                                {
                                    "type": "text",
                                    "text": "成交量",
                                    "size": "sm",
                                    "flex": 1
                                },
                                {
                                    "type": "text",
                                    "text": self.format_number(price.volume as i64),
                                    "size": "sm",
                                    "align": "end",
                                    "flex": 1
                                }
                            ],
                            "margin": "md"
                        },
                        {
                            "type": "box",
                            "layout": "horizontal",
                            "contents": [
                                {
                                    "type": "text",
                                    "text": "成交金額",
                                    "size": "sm",
                                    "flex": 1
                                },
                                {
                                    "type": "text",
                                    "text": self.format_number(price.turnover as i64),
                                    "size": "sm",
                                    "align": "end",
                                    "flex": 1
                                }
                            ],
                            "margin": "md"
                        },
                        {
                            "type": "box",
                            "layout": "horizontal",
                            "contents": [
                                {
                                    "type": "text",
                                    "text": "成交筆數",
                                    "size": "sm",
                                    "flex": 1
                                },
                                {
                                    "type": "text",
                                    "text": self.format_number(price.transactions as i64),
                                    "size": "sm",
                                    "align": "end",
                                    "flex": 1
                                }
                            ],
                            "margin": "md"
                        },
                        {
                            "type": "separator",
                            "margin": "md"
                        },
                        {
                            "type": "box",
                            "layout": "horizontal",
                            "contents": [
                                {
                                    "type": "text",
                                    "text": "日期",
                                    "size": "sm",
                                    "flex": 1
                                },
                                {
                                    "type": "text",
                                    "text": format!("{}", price.date),
                                    "size": "sm",
                                    "align": "end",
                                    "flex": 1
                                }
                            ],
                            "margin": "md"
                        }
                    ]
                }
            }
        })
    }

    /// 構建每日摘要訊息
    fn build_daily_summary_message(&self, date: Date, stocks: &[(Stock, StockPrice)]) -> serde_json::Value {
        // 按漲跌幅排序
        let mut sorted_stocks = stocks.to_vec();
        sorted_stocks.sort_by(|a, b| b.1.change_percent.partial_cmp(&a.1.change_percent).unwrap());
        
        // 取前5名和後5名
        let top_stocks: Vec<_> = sorted_stocks.iter().take(5).collect();
        let bottom_stocks: Vec<_> = sorted_stocks.iter().rev().take(5).collect();
        
        // 構建內容
        let mut contents = vec![
            json!({
                "type": "text",
                "text": format!("每日股票摘要 - {}", date),
                "weight": "bold",
                "size": "xl",
                "color": "#ffffff"
            })
        ];
        
        // 添加漲幅前5名
        contents.push(json!({
            "type": "text",
            "text": "漲幅前5名",
            "weight": "bold",
            "size": "md",
            "color": "#ffffff",
            "margin": "md"
        }));
        
        for (stock, price) in top_stocks {
            contents.push(json!({
                "type": "text",
                "text": format!("{} {} {:.2} {:+.2}%", 
                    stock.code, 
                    self.utf8_encode(&stock.name), 
                    price.close, 
                    price.change_percent
                ),
                "size": "sm",
                "color": "#ffffff"
            }));
        }
        
        // 添加跌幅前5名
        contents.push(json!({
            "type": "text",
            "text": "跌幅前5名",
            "weight": "bold",
            "size": "md",
            "color": "#ffffff",
            "margin": "md"
        }));
        
        for (stock, price) in bottom_stocks {
            contents.push(json!({
                "type": "text",
                "text": format!("{} {} {:.2} {:+.2}%", 
                    stock.code, 
                    self.utf8_encode(&stock.name), 
                    price.close, 
                    price.change_percent
                ),
                "size": "sm",
                "color": "#ffffff"
            }));
        }
        
        // 構建 Flex Message
        json!({
            "type": "flex",
            "altText": format!("每日股票摘要 - {}", date),
            "contents": {
                "type": "bubble",
                "body": {
                    "type": "box",
                    "layout": "vertical",
                    "contents": contents,
                    "backgroundColor": "#0066CC",
                    "paddingAll": "20px"
                },
                "footer": {
                    "type": "box",
                    "layout": "vertical",
                    "contents": [
                        {
                            "type": "button",
                            "action": {
                                "type": "uri",
                                "label": "查看更多",
                                "uri": "https://tw.stock.yahoo.com/"
                            },
                            "style": "primary"
                        }
                    ]
                }
            }
        })
    }

    /// 確保字符串是有效的 UTF-8 編碼
    fn utf8_encode(&self, text: &str) -> String {
        // 如果字符串已經是有效的 UTF-8，則直接返回
        // 否則嘗試將其轉換為有效的 UTF-8
        String::from_utf8_lossy(text.as_bytes()).into_owned()
    }

    /// 格式化數字（加上千分位）
    fn format_number(&self, number: i64) -> String {
        let mut result = String::new();
        let number_str = number.to_string();
        let len = number_str.len();
        
        for (i, c) in number_str.chars().enumerate() {
            result.push(c);
            if (len - i - 1) % 3 == 0 && i < len - 1 {
                result.push(',');
            }
        }
        
        result
    }
}
