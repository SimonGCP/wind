use std::vec::Vec;

use crate::Param;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod request_tests {
    use super::*;

    #[test]
    fn test_new_request() {
        assert_eq!(
            Request::new(String::from("method"), String::from("query"), Vec::new()),
            Request {method: String::from("method"), query: String::from("query"), params: Vec::new()}
        );
    }
}
