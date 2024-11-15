use crate::{application::contracts::jwt::Jwt, domain::repositories::account_repository::AccountRepository};

#[allow(dead_code)]
pub struct AddAccountUsecase<'a> {
    account_repository: &'a dyn AccountRepository,
    jwt_service: &'a dyn Jwt,
}

#[allow(dead_code)]
impl<'a> AddAccountUsecase<'a> {
    pub fn new(account_repository: &'a dyn AccountRepository, jwt_service: &'a dyn Jwt) -> Self {
        AddAccountUsecase { account_repository, jwt_service }
    }
}