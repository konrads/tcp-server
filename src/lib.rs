use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub trait RequestHandler<Payload: Debug, SuccessResult: Debug> {
    fn handle(req: Payload) -> Result<SuccessResult, String>;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestResult<S: Debug> {
    Success(S),
    Err(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request<Payload: Debug> {
    pub id: u64,
    //    #[serde(borrow)]
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<SuccessResult: Debug> {
    pub id: u64,
    pub result: RequestResult<SuccessResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sum {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SumResult {
    pub res: u8,
}

pub struct SumRequestHandler {}

// https://stackoverflow.com/questions/71838366/rust-struct-field-that-implements-multiple-traits
impl RequestHandler<Sum, SumResult> for SumRequestHandler {
    fn handle(req: Sum) -> Result<SumResult, String> {
        req.x
            .checked_add(req.y)
            .map(|x| SumResult { res: x })
            .ok_or_else(|| format!("Failed to sum {} and {}", req.x, req.y))
    }
}

// pub struct TCPRequestHandler<P: Debug, SR: Debug, RH: RequestHandler<P, SR> + Sized> {
//     req_handler: RH,
// }
