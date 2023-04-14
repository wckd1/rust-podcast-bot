use chrono::{DateTime, Utc};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, Pool, Row};
use anyhow::Result;

use crate::models::subscription::Subscription;
use crate::models::episode::{Episode, Enclosure};

const DB_URL: &str = "sqlite://sqlite.db";

#[derive(Debug, Clone)]
pub struct Store {
    db: Pool<Sqlite>
}

impl Store {
    pub async fn new() -> Result<Self> {
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

    // Subscriptions
    pub async fn create_subscription(&self, sub: Subscription) -> Result<()> {
        sqlx::query("INSERT INTO subscriptions (id, url, filter, update_interval, last_updated) VALUES (?,?,?,?,?)")
            .bind(sub.id)
            .bind(sub.url)
            .bind(sub.filter)
            .bind(sub.update_interval)
            .bind(sub.last_updated.to_string())
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub async fn update_subscription(&self, sub: Subscription) -> Result<()> {
        sqlx::query("UPDATE subscriptions SET last_updated = ? WHERE id = ?")
            .bind(sub.last_updated.to_string())
            .bind(sub.id)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub async fn delete_subscription(&self, id: String) -> Result<()> {
        sqlx::query("DELETE FROM subscriptions WHERE id = ?")
            .bind(id)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub async fn get_subscriptions(&self) -> Result<Vec<Subscription>> {
        let result = sqlx::query("SELECT * FROM subscriptions")
            .fetch_all(&self.db)
            .await?
            .iter()
            .map(|row| {
                let upd_string: String = row.get("last_updated");
                
                return Subscription {
                    id: row.get("id"),
                    url: row.get("url"),
                    filter: row.get("filter"),
                    update_interval: row.get("update_interval"),
                    last_updated: upd_string.parse::<DateTime<Utc>>().unwrap(),
                }
            })
            .collect();

        Ok(result)
    }

    // Episodes
    pub async fn create_episode(&self, ep: Episode) -> Result<()> {
        sqlx::query("INSERT INTO episodes (uuid, url, length, type, link, image, title, description, author, duration, pub_date) VALUES (?,?,?,?,?,?,?,?,?,?,?)")
            .bind(ep.uuid)
            .bind(ep.enclosure.url)
            .bind(ep.enclosure.length.to_string())
            .bind(ep.enclosure.enclosure_type)
            .bind(ep.link)
            .bind(ep.image)
            .bind(ep.title)
            .bind(ep.description)
            .bind(ep.author)
            .bind(ep.duration.to_string())
            .bind(ep.pub_date)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub async fn get_episodes(&self, limit: u64) -> Result<Vec<Episode>> {
        let result = sqlx::query("SELECT * FROM episodes LIMIT ?")
            .bind(limit.to_string())
            .fetch_all(&self.db)
            .await?
            .iter()
            .map(|row| {
                let length: String = row.get("length");
                let duration: String = row.get("duration");

                return Episode { 
                    uuid: row.get("uuid"), 
                    enclosure: Enclosure { 
                        url: row.get("url"), 
                        length: length.parse::<u64>().unwrap(), 
                        enclosure_type: row.get("type") 
                    }, 
                    link: row.get("link"), 
                    image: row.get("image"), 
                    title: row.get("title"), 
                    description: row.get("description"), 
                    author: row.get("author"), 
                    duration: duration.parse::<u64>().unwrap(), 
                    pub_date: row.get("pub_date") 
                }
            })
            .collect();

        Ok(result)
    }
}
