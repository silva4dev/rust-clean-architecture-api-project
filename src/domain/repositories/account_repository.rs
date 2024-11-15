use async_trait::async_trait;
use crate::domain::entities::account_entity::AccountEntity;
use std::error::Error;

#[allow(dead_code)]
#[async_trait(?Send)]
pub trait AccountRepository {
    async fn create_account(&self, account: AccountEntity) -> Result<AccountEntity, Box<dyn Error>>;
    async fn find_account_by_email(&self, email: &str) -> Result<Option<AccountEntity>, Box<dyn Error>>;
}