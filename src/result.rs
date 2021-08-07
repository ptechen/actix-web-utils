use serde::{Serialize};
use std::fmt::Debug;
use std::future::Future;
// use std::error::Error;
use actix_web::{HttpResponse, error::Error};

#[derive(Serialize, Debug, Default)]
pub struct Empty{}

#[derive(Serialize, Debug)]
pub struct ResponseResult<T>
    where T: Serialize + Debug
{
    pub code: usize,
    pub msg: String,
    pub result: T,
}

impl Default for ResponseResult<Empty>
{
    fn default() -> ResponseResult<Empty> {
        ResponseResult {
            code: 0,
            msg: String::from("success"),
            result: Empty::default(),
        }
    }
}

impl <T> ResponseResult<T>
    where T: Serialize + Debug
{
    pub async fn ok(data: T) -> HttpResponse {
        HttpResponse::Ok().json(ResponseResult {
            code: 0,
            msg: String::from("success"),
            result: data,
        })
    }

    pub async fn err(code: usize, msg: String, data: T) -> HttpResponse {
        HttpResponse::Ok().json(ResponseResult {
            code,
            msg,
            result: data,
        })
    }

    pub async fn res(code: usize, f: impl Future<Output=Result<T, Error>>) -> HttpResponse {
        match f.await {
            Ok(d) => {
                ResponseResult::ok(d).await
            }
            Err(e) => {
                ResponseResult::err(code, e.to_string(), Empty::default()).await
            }
        }
    }
}