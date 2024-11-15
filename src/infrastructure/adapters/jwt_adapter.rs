use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use chrono::{Utc, Duration};

use crate::application::contracts::jwt::{Jwt, Claims};

const SECRET: &[u8] = b"secret";

pub struct JwtAdapter;

impl Jwt for JwtAdapter {
    fn generate_jwt(&self, user_id: &str) -> String {
        let expiration = (Utc::now() + Duration::hours(1)).timestamp() as usize;
        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
        };
        let encoding_key = EncodingKey::from_secret(SECRET);
        encode(&Header::new(Algorithm::HS256), &claims, &encoding_key)
            .expect("Error creating JWT token")
    }

    fn validate_jwt(&self, token: &str) -> Result<Claims, String> {
        let decoding_key = DecodingKey::from_secret(SECRET);
        let validation = Validation::new(Algorithm::HS256);
        decode::<Claims>(token, &decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| format!("Error validating token: {}", e))
    }
}
