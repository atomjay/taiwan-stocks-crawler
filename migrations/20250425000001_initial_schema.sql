-- 創建 stocks 表
CREATE TABLE IF NOT EXISTS stocks (
    id UUID PRIMARY KEY,
    code VARCHAR(10) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    last_updated TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 創建 stock_prices 表
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
);
