use crate::{domain::{entities::account_entity::AccountEntity, repositories::account_repository::AccountRepository}, infrastructure::helpers::pg_helper::PgHelper};
use tokio_postgres::Client;
use async_trait::async_trait;
use std::error::Error;

pub struct PgAccountRepository;

#[allow(dead_code)]
impl PgAccountRepository {
    pub fn new() -> Self {
        PgAccountRepository {}
    }

    async fn get_db_client() -> Result<Client, Box<dyn Error>> {
        let client = PgHelper::connect_to_db().await?;
        Ok(client)
    }
}

#[async_trait(?Send)]
impl AccountRepository for PgAccountRepository {
    async fn create_account(&self, account: AccountEntity) -> Result<AccountEntity, Box<dyn Error>> {
        let client = PgAccountRepository::get_db_client().await?;
        
        let stmt = client.prepare("INSERT INTO accounts (id, email, password) VALUES ($1, $2, $3) RETURNING id, email, password").await?;
        let row = client.query_one(&stmt, &[&account.id, &account.email, &account.password]).await?;
        
        Ok(AccountEntity {
            id: row.get(0),
            email: row.get(1),
            password: row.get(2),
        })
    }

    async fn find_account_by_email(&self, email: &str) -> Result<Option<AccountEntity>, Box<dyn Error>> {
        let client = PgAccountRepository::get_db_client().await?;

        let stmt = client.prepare("SELECT id, email, password FROM accounts WHERE email = $1").await?;
        let rows = client.query(&stmt, &[&email]).await?;

        if let Some(row) = rows.get(0) {
            Ok(Some(AccountEntity {
                id: row.get(0),
                email: row.get(1),
                password: row.get(2),
            }))
        } else {
            Ok(None)
        }
    }
}
