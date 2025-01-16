use std::fs;
use std::sync::Arc;

use wind::{Server, Router, Request, Response, response::HTTPCodes::*, Param};

fn main() -> std::io::Result<()> {
    let _loopback = "127.0.0.1:8000";
    let _myip = "172.29.150.135:80";

    let server = &mut Server::new();

    let mut router = Router::new("/route");

    router.get("/new_route", Arc::new(|_req: &mut Request, _res: &mut Response| {
       println!("Hello from router!"); 

       _res.send(OK, Vec::from("Hello from router!"));
    }));

    server.use_router(router);

    // test middleware
    server.use_middleware(Arc::new(|_req: &mut Request, _res: &mut Response| {
        let param = Param::new("name=simon");
        if param.is_some() {
            _req.params.push(param.unwrap());
        }

        _res.headers.push("Access-Control-Allow-Origin: *".to_string());
        
        _res.next();
    }));

    server.get("/new_route", Arc::new(|_req: &mut Request, _res: &mut Response| {
       println!("Hello from outside router!"); 

       _res.send(OK, Vec::from("Hello from router!"));
    }));

    server.get("/route/fake_route", Arc::new(|_req: &mut Request, _res: &mut Response| {
        _res.send(OK, Vec::from("Hello from outside router!"));
    }));

    server.get("/", Arc::new(|_req: &mut Request, _res: &mut Response| {
        // println!("Request: {:?}", _req);
        let contents = fs::read("../client/webpage.html");

        if contents.is_ok() {
            *_res = Response::ok(contents.unwrap());
        } else {
            let vec = Vec::from("Not Found".as_bytes());
            *_res = Response::new(NotFound, vec);
        }
    }));

    server.get("/images/default-dance-fortnite.gif", Arc::new(|_req: &mut Request, _res: &mut Response| {
        let contents = fs::read("../client/images/default-dance-fortnite.gif");

        if contents.is_ok() {
            _res.send(OK, contents.unwrap());
        } else {
            let vec = Vec::from("fortnite gif not found :(".as_bytes());
            _res.send(InternalServerError, vec);
        }
    }));

    server.get("/get_some_info", Arc::new(|_req: &mut Request, _res: &mut Response| {
        _res.send(OK, Vec::from("Hello world!"));
    }));

    server.post("/same_url", Arc::new(|_req: &mut Request, _res: &mut Response| {
        _res.send(OK, Vec::from("Hello from post!"));
    }));

    server.get("/same_url", Arc::new(|_req: &mut Request, _res: &mut Response| {
        _res.send(OK, Vec::from("Hello from get!"));
    }));

    server.post("/send_body", Arc::new(|_req: &mut Request, _res: &mut Response| {
        _res.send(OK, _req.body.clone());
    }));

    // logging
    server.use_middleware(Arc::new(|_req: &mut Request, _res: &mut Response| {
        println!("{}: {}", _req.query, _res.code.as_str());         
        
        _res.next();
    }));

    server.use_middleware(Arc::new(|_req: &mut Request, _res: &mut Response| {
        if _res.code == NotFound {
            _res.send(NotFound, Vec::from("Not found :("));
        }
    }));

    server.listen( _loopback);

    Ok(())
}
