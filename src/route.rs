use std::io::Error;

use crate::response::Response;
use crate::request::Request;

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

#[cfg(test)]
mod route_tests {
    use super::*;

    #[test]
    fn route_function_test() {
        // router.route("/", Box::new(|_request: Request| -> Result<Response, Error> {
        let route = Route::new("/", Box::new(|_req: Request| -> Result<Response, Error> {
            Response::new("200 OK", Vec::new())
        }));

        let some_request = Request::new(String::from(""), String::from(""), Vec::new());
        assert_eq!(Response::new("200 OK", Vec::new()).unwrap(), route.get_result(some_request).unwrap());
    }
}
