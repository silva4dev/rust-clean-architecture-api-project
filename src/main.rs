use rust_clean_api::api::server::run;

#[tokio::main]
async fn main() {
    run().await.unwrap();
}