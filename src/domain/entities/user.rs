use diesel::Queryable;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub phone: String,
    pub address: String,
}
