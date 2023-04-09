use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, Pool};
use anyhow::Result;

use crate::models::subscription::Subscription;
use crate::models::episode::Episode;

const DB_URL: &str = "sqlite://sqlite.db";

// TODO: Close with trait
#[derive(Debug, Clone)]
pub struct Store {
    db: Pool<Sqlite>
}

// TODO: Update to use logging
impl Store {
    pub async fn init() -> Result<Self> {
        // Check if database exist
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            println!("Creating database {}", DB_URL);
            Sqlite::create_database(DB_URL).await?;
        } else {
            println!("Database already exists");
        }

        // Create Store with connection
        let db = SqlitePool::connect(DB_URL).await?;

        // Migrate
        println!("Starting migrations");

        let crate_dir = std::env::var("CARGO_MANIFEST_DIR")?;
        let migrations = std::path::Path::new(&crate_dir).join("./migrations");

        sqlx::migrate::Migrator::new(migrations)
            .await?
            .run(&db)
            .await?;

        println!("Migrations success");

        Ok(Self { db })
    }

    pub async fn create_subscription(&self, sub: Subscription) -> Result<()> {
        sqlx::query("INSERT INTO subscriptions (id, url) VALUES (?,?)")
            .bind(sub.id)
            .bind(sub.url)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub async fn delete_subscription(&self, sub: Subscription) -> Result<()> {
        sqlx::query("DELETE FROM subscriptions WHERE id = ?")
            .bind(sub.id)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub async fn create_episode(&self, ep: Episode) -> Result<()> {
        sqlx::query("INSERT INTO episodes (uuid, url, length, type, link, image, title, description, author, duration, pub_date) VALUES (?,?,?,?,?,?,?,?,?,?,?)")
            .bind(ep.uuid)
            .bind(ep.enclosure.url)
            .bind(ep.enclosure.length)
            .bind(ep.enclosure.enclosure_type)
            .bind(ep.link)
            .bind(ep.image)
            .bind(ep.title)
            .bind(ep.description)
            .bind(ep.author)
            .bind(ep.duration)
            .bind(ep.pub_date)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
