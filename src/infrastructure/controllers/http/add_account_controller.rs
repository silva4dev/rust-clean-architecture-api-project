use serde_json::json;
use warp::{Rejection, Reply};

pub async fn add_account_controller(body: serde_json::Value) -> Result<impl Reply, Rejection> {
  println!("Received body: {:?}", body);
  Ok(warp::reply::json(&json!({ "message": "Hello World" })))
}