use tokio_postgres::{NoTls, Client, Error};
use dotenv::dotenv;
use std::env;

pub struct PgHelper;

#[allow(dead_code)]
impl PgHelper {
    pub async fn connect_to_db() -> Result<Client, Error> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });
        Ok(client)
    }
}
