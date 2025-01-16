use std::fs;
use std::sync::Arc;

use wind::{Router, Request, Response, response::HTTPCodes::*, Param};

fn main() -> std::io::Result<()> {
    let _loopback = "127.0.0.1:8000";
    let _myip = "172.29.150.135:80";

    let router = &mut Router::new();

    // test middleware
    router.use_middleware(Arc::new(|_req: &mut Request, _res: &mut Response| {
        let param = Param::new("name=simon");
        if param.is_some() {
            _req.params.push(param.unwrap());
        }

        _res.headers.push("Access-Control-Allow-Origin: *".to_string());
        
        _res.next();
    }));

    router.get("/", Arc::new(|_req: &mut Request, _res: &mut Response| {
        // println!("Request: {:?}", _req);
        let contents = fs::read("../client/webpage.html");

        if contents.is_ok() {
            *_res = Response::ok(contents.unwrap());
        } else {
            let vec = Vec::from("Not Found".as_bytes());
            *_res = Response::new(NotFound, vec);
        }
    }));

    router.get("/images/default-dance-fortnite.gif", Arc::new(|_req: &mut Request, _res: &mut Response| {
        let contents = fs::read("../client/images/default-dance-fortnite.gif");

        if contents.is_ok() {
            _res.send(OK, contents.unwrap());
        } else {
            let vec = Vec::from("fortnite gif not found :(".as_bytes());
            _res.send(InternalServerError, vec);
        }
    }));

    router.get("/get_some_info", Arc::new(|_req: &mut Request, _res: &mut Response| {
        _res.send(OK, Vec::from("Hello world!"));
    }));

    router.post("/same_url", Arc::new(|_req: &mut Request, _res: &mut Response| {
        _res.send(OK, Vec::from("Hello from post!"));
    }));

    router.get("/same_url", Arc::new(|_req: &mut Request, _res: &mut Response| {
        _res.send(OK, Vec::from("Hello from get!"));
    }));

    router.post("/send_body", Arc::new(|_req: &mut Request, _res: &mut Response| {
        _res.send(OK, _req.body.clone());
    }));

    router.use_middleware(Arc::new(|_req: &mut Request, _res: &mut Response| {
        println!("{}: {}", _req.query, _res.code.as_str());         
        
        _res.next();
    }));

    router.listen( _myip);

    Ok(())
}
