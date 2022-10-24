use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait RequestHandler<Payload: Debug, SuccessResult: Debug> {
    fn handle(&self, req: Payload) -> std::result::Result<SuccessResult, String>;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Result<S: Debug> {
    Success(S),
    Err(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request<Payload: Debug> {
    pub id: u64,
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<SuccessResult: Debug> {
    pub id: u64,
    pub result: Result<SuccessResult>,
}
