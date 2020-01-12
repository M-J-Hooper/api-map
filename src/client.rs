use crate::ApiMapResult;
use std::collections::{HashMap};
use hyper::{self, Request, Response, client::HttpConnector, Body};
use serde_json::{Value};

pub struct Client {
    inner: hyper::Client<HttpConnector, Body>,
    cache: HashMap<String, Response<Value>>,
}

impl Client {
    pub fn new() -> Client {
        Client { inner: hyper::Client::new(), cache: HashMap::new() }
    }

    pub fn request(&self, request: Request<&Value>) -> ApiMapResult<&Value> {
        match self.cache.get(&request.uri().to_string()) {
            Some(r) => Ok(r.body()), // Convert r to value
            None => Ok(&Value::Null), // self.inner.request(request)
        }
    }
}