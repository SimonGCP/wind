use std::sync::Arc;

use crate::response::Response;
use crate::request::Request;

pub enum Next {
    Next,
    Error(std::io::Error),
}

#[derive(Clone)]
pub struct Route {
    pub path: String,
    pub result: Arc<dyn Fn(&Request) -> Response + Send + Sync + 'static>,
    pub middleware: bool
}

impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

impl Route {
    pub fn new (path: &str, result: Arc<dyn Fn(&Request) -> Response + Send + Sync + 'static>) -> Route {
        Route { path: path.to_string(), result, middleware: false }
    }

    pub fn middleware(result: Arc<dyn Fn(&Request) -> Response + Send + Sync + 'static>) -> Route {
        Route { path: "".to_string(), result, middleware: true }
    }

    pub fn get_result(&self, req: &Request) -> Response {
        (self.result)(req)
    }
}

#[cfg(test)]
mod route_tests {
    use super::*;

    // #[test]
    // fn route_function_test() {
    //     // router.route("/", Box::new(|_request: Request| -> Result<Response, Error> {
    //     let route = Route::new("/", Box::new(|_req: Request| -> Result<Response, Error> {
    //         Response::new("200 OK", Vec::new())
    //     }));

    //     let some_request = Request::new(String::from(""), String::from(""), Vec::new());
    //     assert_eq!(Response::new("200 OK", Vec::new()).unwrap(), route.get_result(some_request).unwrap());
    // }
}
