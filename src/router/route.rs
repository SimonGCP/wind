use std::io::Error;

use super::{
    response::Response,
    request::Request,
};

pub struct Route {
    pub path: String,
    pub result: Box<dyn Fn(Request) -> Result<Response, Error>>,
}

impl Route {
    pub fn new (path: &str, result: Box<dyn Fn(Request) -> Result<Response, Error>>) -> Route {
        Route { path: path.to_string(), result }
    }

    pub fn get_result(&self, req: Request) -> Result<Response, Error> {
        (self.result)(req)
    }
}
