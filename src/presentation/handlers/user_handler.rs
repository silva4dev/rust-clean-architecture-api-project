use crate::{
    application::use_cases::{get_user::GetUserUseCase, register_user::RegisterUseCase},
    infrastructure::repositories::postgres_user_repository::PostgresUserRepository,
    schema::users,
};
use actix_web::{
    get, post,
    web::{self, Path},
    HttpResponse,
};
use diesel::prelude::Insertable;
use log::error;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

#[post("")]
pub async fn register_user_handler(
    repo: web::Data<PostgresUserRepository>,
    input: web::Json<NewUser>,
) -> HttpResponse {
    if RegisterUseCase::new(repo.into_inner())
        .execute(input.into_inner())
        .await
        .is_ok()
    {
        HttpResponse::Ok().finish()
    } else {
        error!("Error registering user");
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/{email}")]
pub async fn get_by_email(
    repo: web::Data<PostgresUserRepository>,
    path: Path<(String,)>,
) -> HttpResponse {
    if let Some(user) = GetUserUseCase::new(repo.into_inner())
        .get(path.into_inner().0)
        .await
    {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().finish()
    }
}
