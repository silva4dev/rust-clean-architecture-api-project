use uuid::Uuid;
use rust_clean_api::domain::entities::account_entity::AccountEntity;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn creates_account_with_provided_id() {
        let sut = AccountEntity::new(AccountEntity {
            id: Some("1234".to_string()),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        });

        assert_eq!(sut.id, Some("1234".to_string()));
        assert_eq!(sut.email, "test@example.com");
        assert_eq!(sut.password, "password123");
    }

    #[test]
    fn creates_account_with_generated_id() {
        let sut = AccountEntity::new(AccountEntity {
            id: None,
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        });

        assert!(sut.id.is_some());
        assert!(Uuid::parse_str(sut.id.as_ref().unwrap()).is_ok());
        assert_eq!(sut.email, "test@example.com");
        assert_eq!(sut.password, "password123");
    }
}
