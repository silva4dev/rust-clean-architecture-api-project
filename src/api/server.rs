use std::env;
use crate::api::routes::routes;

pub async fn run() -> Result<(), std::io::Error> {
    let port = 3333;

    env_logger::try_init().unwrap_or(());
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "warp=debug");

    println!("Server running on port {}", port);

    warp::serve(routes())
        .run(([127, 0, 0, 1], port)).await;

    Ok(())
}
