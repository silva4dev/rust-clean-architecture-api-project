use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountEntity {
    pub id: Option<String>,
    pub email: String,
    pub password: String
}

#[allow(dead_code)]
impl AccountEntity {
    pub fn new(props: AccountEntity) -> Self {
        AccountEntity {
            id: props.id.or_else(|| Some(Uuid::new_v4().to_string())),
            email: props.email,
            password: props.password
        }
    }
}
