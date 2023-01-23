use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<ByteBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub status: usize,
    pub body: Option<ByteBuf>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoxedRequestError(Box<RequestError>);

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestError {
    pub kind: RequestErrorKind,
    pub url: Option<String>,
    pub message: String,
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[{:?}] {:?}: {}", self.kind, self.url, self.message)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestErrorKind {
    Serial,
    Request,
    Redirect,
    Status(u16),
    Body,
    Timeout,
    Unknown,
}

impl From<RequestError> for BoxedRequestError {
    fn from(inner: RequestError) -> Self {
        BoxedRequestError(Box::new(inner))
    }
}
