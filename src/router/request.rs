use std::vec::Vec;

use crate::router::Param;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub query: String,
    pub params: Vec<Param>,
}

impl Request {
    pub fn new(method: String, query: String, params: Vec<Param>) -> Request {
        Request {method, query, params}
    }
}
