use std::error::Error;
use async_trait::async_trait;

#[allow(dead_code)]
#[async_trait(?Send)]
pub trait Usecase<T> {
    async fn execute(&mut self, input: Option<T>) -> Result<T, Box<dyn Error>>;
}