use std::fs;
use std::io::Error;

mod router;

use router::{Router, Request, Response, response::HTTPCodes::*};

fn main() -> std::io::Result<()> {
    let _loopback = "127.0.0.1:80";
    let _myip = "172.16.14.193:80";

    let mut router = Router::new("client");
    router.route("/", Box::new(|_request: Request| -> Result<Response, Error> {
        println!("Request: {:?}", _request);
        let contents = fs::read("../client/webpage.html");

        if contents.is_ok() {
            Response::ok(contents.unwrap())
        } else {
            let vec = Vec::from(String::from("Not Found").as_bytes());
            Response::new(NotFound.as_str(), vec)
        }
    })).unwrap();

    router.route("/get_some_info", Box::new(|_request: Request| -> Result<Response, Error> {
        println!("Request: {:?}", _request);
        println!("Params: {:?}", _request.params);
        // let contents = fs::read("client/data/some_data.json");
        let param = _request.params.get(0);

        if param.is_some() {
            Response::ok(Vec::from(param.unwrap().value.as_str().as_bytes()))
        } else {
            Response::new(BadRequest.as_str(), Vec::from("No parameters given"))
        }
    })).unwrap();

    router.route("/write_some_info", Box::new(|_request: Request| -> Result<Response, Error> {
        println!("Request: {:?}", _request);
        let params = _request.params;
        println!("Params: {:?}", params);

        let vec = Vec::from(String::from("OK!").as_bytes());
        Response::new(OK.as_str(), vec)
    })).unwrap();

    router.listen(_loopback);

    Ok(())
}
