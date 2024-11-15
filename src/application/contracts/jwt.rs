use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[allow(dead_code)]
pub trait Jwt {
    fn generate_jwt(&self, user_id: &str) -> String;
    fn validate_jwt(&self, token: &str) -> Result<Claims, String>;
}
