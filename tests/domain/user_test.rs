use rust_clean_architecture_api_project::domain::entities::user::User;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_user() {
        let sut = User {
            id: 1,
            name: "John Doe".to_string(),
            email: Some("johndoe@example.com".to_string()),
            phone: "(19)99281-9212".to_string(),
            address: "XYZ ABC Tower".to_string(),
        };

        assert_eq!(sut.id, 1);
        assert_eq!(sut.name, "John Doe");
        assert_eq!(sut.email.unwrap(), "johndoe@example.com");
        assert_eq!(sut.phone, "(19)99281-9212");
        assert_eq!(sut.address, "XYZ ABC Tower")
    }
}
