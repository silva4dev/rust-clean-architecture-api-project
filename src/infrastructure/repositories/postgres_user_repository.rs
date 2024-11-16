use std::env;
use std::sync::Arc;
use async_trait::async_trait;
use diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};
use diesel::result::Error;
use diesel::prelude::*;
use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::db::connection::{DBPool, establish_connection};
use crate::presentation::handlers::user_handler::NewUser;
use crate::schema::users::dsl::*;
use crate::schema::users;

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: DBPool,
}

impl PostgresUserRepository {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL")
            .expect("DB URL..");

        PostgresUserRepository {
            pool: establish_connection(&database_url),
        }
    }
}

#[async_trait]
impl UserRepository for Arc<PostgresUserRepository> {
    async fn find_by_email(&self, input_email: String) -> Result<User, Error> {
        let mut connection = self.pool.get().map_err(|_| Error::NotFound)?;
        let user = users
            .filter(email.eq(input_email))
            .first::<User>(& mut connection)
            .optional()
            .map_err(|err| {
                match err {
                    diesel::result::Error::NotFound => Error::NotFound,
                    _ => err,
                }
            })?;
        user.ok_or(Error::NotFound)
    }

    async fn save(&self, user: &NewUser) -> Result<(), Error> {
        let mut connection = self.pool.get().map_err(|_| Error::NotFound)?;

        diesel::insert_into(users::table)
            .values(user)
            .execute(& mut connection)
            .map_err(|err| { err })?;

        Ok(())
    }
}