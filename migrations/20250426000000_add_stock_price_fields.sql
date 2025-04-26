-- Add new columns to stock_prices table
ALTER TABLE stock_prices ADD COLUMN change FLOAT NOT NULL DEFAULT 0.0;
ALTER TABLE stock_prices ADD COLUMN change_percent FLOAT NOT NULL DEFAULT 0.0;
ALTER TABLE stock_prices ADD COLUMN turnover BIGINT NOT NULL DEFAULT 0;
ALTER TABLE stock_prices ADD COLUMN transactions BIGINT NOT NULL DEFAULT 0;
ALTER TABLE stock_prices ADD COLUMN pe_ratio FLOAT;
ALTER TABLE stock_prices ADD COLUMN pb_ratio FLOAT;
ALTER TABLE stock_prices ADD COLUMN dividend_yield FLOAT;
ALTER TABLE stock_prices ADD COLUMN market_cap BIGINT;
ALTER TABLE stock_prices ADD COLUMN foreign_buy BIGINT;
ALTER TABLE stock_prices ADD COLUMN trust_buy BIGINT;
ALTER TABLE stock_prices ADD COLUMN dealer_buy BIGINT;
