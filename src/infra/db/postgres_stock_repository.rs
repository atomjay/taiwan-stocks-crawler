use crate::domain::models::Stock;
use crate::domain::repositories::StockRepository;
use crate::domain::value_objects::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct PostgresStockRepository {
    pool: PgPool,
}

impl PostgresStockRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StockRepository for PostgresStockRepository {
    async fn save(&self, stock: &Stock) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO stocks (id, code, name, last_updated)
            VALUES ($1, $2, $3, to_timestamp($4))
            ON CONFLICT (code) DO UPDATE
            SET name = $3, last_updated = to_timestamp($4)
            "#,
            stock.id,
            stock.code,
            stock.name,
            stock.last_updated.unix_timestamp() as f64,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Stock>> {
        let record = sqlx::query!(
            r#"
            SELECT id, code, name, extract(epoch from last_updated) as "last_updated!: f64"
            FROM stocks
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(|r| {
            Stock {
                id: r.id,
                code: r.code,
                name: r.name,
                last_updated: OffsetDateTime::from_unix_timestamp(r.last_updated as i64)
                    .unwrap_or_else(|_| OffsetDateTime::now_utc()),
            }
        }))
    }

    async fn find_by_code(&self, code: &str) -> Result<Option<Stock>> {
        let record = sqlx::query!(
            r#"
            SELECT id, code, name, extract(epoch from last_updated) as "last_updated!: f64"
            FROM stocks
            WHERE code = $1
            "#,
            code,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(|r| {
            Stock {
                id: r.id,
                code: r.code,
                name: r.name,
                last_updated: OffsetDateTime::from_unix_timestamp(r.last_updated as i64)
                    .unwrap_or_else(|_| OffsetDateTime::now_utc()),
            }
        }))
    }

    async fn find_all(&self) -> Result<Vec<Stock>> {
        let records = sqlx::query!(
            r#"
            SELECT id, code, name, extract(epoch from last_updated) as "last_updated!: f64"
            FROM stocks
            ORDER BY code
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records
            .into_iter()
            .map(|r| {
                Stock {
                    id: r.id,
                    code: r.code,
                    name: r.name,
                    last_updated: OffsetDateTime::from_unix_timestamp(r.last_updated as i64)
                        .unwrap_or_else(|_| OffsetDateTime::now_utc()),
                }
            })
            .collect())
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM stocks
            WHERE id = $1
            "#,
            id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
