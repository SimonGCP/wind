use std::fs;
use std::sync::Arc;

use cli_chat::{Router, Request, Response, response::HTTPCodes::*};

fn main() -> std::io::Result<()> {
    let _loopback = "127.0.0.1:8000";
    let _myip = "172.16.14.193:80";

    let router = &mut Router::new();

    // test middleware
    router.use_middleware(Arc::new(|_request: &Request| -> Response {
        println!("Hello from middleware!");
        Response::next()
    }));

    router.route("/", Arc::new(|_request: &Request| -> Response {
        println!("Request: {:?}", _request);
        let contents = fs::read("../client/webpage.html");

        if contents.is_ok() {
            Response::ok(contents.unwrap())
        } else {
            let vec = Vec::from("Not Found".as_bytes());
            Response::new(NotFound, vec)
        }
    }));

    router.route("/get_some_info", Arc::new(|_request: &Request| -> Response {
        println!("Request: {:?}", _request);
        println!("Params: {:?}", _request.params);
        // let contents = fs::read("client/data/some_data.json");
        let param = _request.params.get(0);

        if param.is_some() {
            Response::ok(Vec::from(param.unwrap().value.as_str().as_bytes()))
        } else {
            Response::new(BadRequest, Vec::from("No parameters given"))
        }
    }));

    router.route("/write_some_info", Arc::new(|_request: &Request| -> Response {
        println!("Request: {:?}", _request);
        let params = &_request.params;
        println!("Params: {:?}", params);

        let vec = Vec::from(String::from("OK!").as_bytes());
        Response::new(OK, vec)
    }));

    router.route("/slow_request", Arc::new(|_request: &Request| -> Response {
        std::thread::sleep(std::time::Duration::from_secs(10));

        Response::ok(Vec::from("OK!".as_bytes()))
    }));

    router.listen( _loopback);

    Ok(())
}
