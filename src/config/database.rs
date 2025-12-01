use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::OnceLock;
use tokio::sync::OnceCell;

static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn initialize_database() -> Result<(), Box<dyn std::error::Error>> {
    let db_host = std::env::var("DB_HOST")?;
    let db_port = std::env::var("DB_PORT")?;
    let db_user = std::env::var("DB_USER")?;
    let db_password = std::env::var("DB_PASSWORD")?;
    let db_name = std::env::var("DB_NAME")?;
    
    let database_url = format!("postgresql://{}:{}@{}:{}/{}", db_user, db_password, db_host, db_port, db_name);
    
    let mut attempts = 0;
    let max_attempts = 5;
    
    loop {
        match PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await {
            Ok(pool) => {
                DB_POOL.set(pool).map_err(|_| "Failed to set database pool")?;
                return Ok(());
            }
            Err(e) => {
                attempts += 1;
                if attempts >= max_attempts {
                    return Err(e.into());
                }
                println!("Database connection attempt {} failed, retrying in 2 seconds...", attempts);
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        }
    }
}

pub async fn get_db_pool() -> &'static PgPool {
    DB_POOL.get_or_init(|| async {
        let db_host = std::env::var("DB_HOST").expect("DB_HOST not set");
        let db_port = std::env::var("DB_PORT").expect("DB_PORT not set");
        let db_user = std::env::var("DB_USER").expect("DB_USER not set");
        let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD not set");
        let db_name = std::env::var("DB_NAME").expect("DB_NAME not set");
        
        let database_url = format!("postgresql://{}:{}@{}:{}/{}", db_user, db_password, db_host, db_port, db_name);
        
        PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .expect("Failed to connect to database")
    }).await
}