use serde_json::{json, Value};
use warp::{Rejection, Reply};

pub async fn auth_account_controller(body: Value) -> Result<impl Reply, Rejection> {
  println!("Received body: {:?}", body);
  Ok(warp::reply::json(&json!({ "message": "Hello World" })))
}