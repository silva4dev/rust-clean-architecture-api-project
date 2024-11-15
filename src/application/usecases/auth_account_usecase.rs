use crate::{application::contracts::jwt::Jwt, domain::repositories::account_repository::AccountRepository};

#[allow(dead_code)]
pub struct AuthAccountUsecase<'a> {
    account_repository: &'a dyn AccountRepository,
    jwt_service: &'a dyn Jwt,
}

#[allow(dead_code)]
impl<'a> AuthAccountUsecase<'a> {
    pub fn new(account_repository: &'a dyn AccountRepository, jwt_service: &'a dyn Jwt) -> Self {
        AuthAccountUsecase { account_repository, jwt_service }
    }
}