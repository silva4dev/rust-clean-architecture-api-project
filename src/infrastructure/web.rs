use crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository;
use crate::presentation::routes::user_routes::routes;
use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;

pub async fn run() -> std::io::Result<()> {
    let repo = PostgresUserRepository::new();
    let app_data = web::Data::new(repo);

    info!("Starting server...");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .configure(routes)
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
