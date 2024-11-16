use crate::{domain::{repositories::user_repository::UserRepository, services::user_service::UserService}, presentation::handlers::user_handler::NewUser};

pub struct RegisterUseCase<T: UserRepository> {
    user_service: UserService<T>,
}

impl<T: UserRepository> RegisterUseCase<T> {
    pub fn new(user_repo: T) -> Self {
        let user_service = UserService::new(user_repo);
        RegisterUseCase { user_service }
    }

    pub async fn execute(&self, new_user: NewUser) -> Result<(), diesel::result::Error> {
       self.user_service.register_user(new_user).await
    }
}
