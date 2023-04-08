use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, Pool};

use crate::models::subscription::Subscription;

const DB_URL: &str = "sqlite://sqlite.db";

#[derive(Debug, Clone)]
pub enum StoreError {
    ConnectionError(String),
    MigrationError(String),
    QueryError(String),
}

#[derive(Debug, Clone)]
pub struct Store {
    db: Pool<Sqlite>
}

// TODO: Update to use logging
impl Store {
    pub async fn init() -> Result<Self, StoreError> {
        // Check if database exist
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            println!("Creating database {}", DB_URL);
            Sqlite::create_database(DB_URL)
                .await
                .or_else(|err| Err(StoreError::ConnectionError(err.to_string())))?;
        } else {
            println!("Database already exists");
        }

        // Create Store with connection
        let db = SqlitePool::connect(DB_URL)
            .await
            .or_else(|err| Err(StoreError::ConnectionError(err.to_string())))?;

        // Migrate
        println!("Starting migrations");

        let crate_dir = std::env::var("CARGO_MANIFEST_DIR")
            .or_else(|err| Err(StoreError::MigrationError(err.to_string())))?;
        let migrations = std::path::Path::new(&crate_dir).join("./migrations");

        sqlx::migrate::Migrator::new(migrations)
            .await
            .or_else(|err| Err(StoreError::MigrationError(err.to_string())))?
            .run(&db)
            .await
            .or_else(|err| Err(StoreError::MigrationError(err.to_string())))?;

        println!("Migrations success");

        Ok(Self { db })
    }

    pub async fn create_subscription(&self, sub: Subscription) -> Result<(), StoreError> {
        sqlx::query("INSERT INTO subscriptions (id, url) VALUES (?,?)")
            .bind(sub.id)
            .bind(sub.url)
            .execute(&self.db)
            .await
            .or_else(|err| Err(StoreError::QueryError(err.to_string())))?;

        Ok(())
    }

    pub async fn delete_subscription(&self, sub: Subscription) -> Result<(), StoreError> {
        sqlx::query("DELETE FROM subscriptions WHERE id = ?")
            .bind(sub.id)
            .execute(&self.db)
            .await
            .or_else(|err| Err(StoreError::QueryError(err.to_string())))?;

        Ok(())
    }
}
