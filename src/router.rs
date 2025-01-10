use std::{
    net::{
        TcpStream, 
        TcpListener
    },
    fs,
    io::{
        Write,
        BufReader,
        prelude::*,
        Error,
        ErrorKind,
    },
    vec::Vec,
};

pub mod route;
pub mod response;
pub mod request;

pub use route::Route;
pub use response::Response;
pub use request::Request;

pub struct Router {
    root: String,
    routes: Vec<Route>,   
}

impl Router {
    pub fn new(root: &str) -> Router {
        let root = root.to_string();

        Router {
            root: root.to_string(),
            routes: Vec::new(),
        }
    }

    pub fn listen(&self, address: &str) {
        let listener: TcpListener = TcpListener::bind(address).unwrap();

        println!("Starting server at {}", address);
        for client in listener.incoming() {
            match client {
                Ok(stream) => {
                    self.handle_client(stream); 
                }
                Err(e) => panic!("Error connecting: {e}"),
            }
        }
    }

    pub fn route(&mut self, path: &str, result: Box<dyn Fn(Request) -> Result<Response, Error>>) -> Result<(), String> {
        let new_route = Route::new(path, Box::new(result)); 
        self.routes.push(new_route); 

        Ok(())
    }

    pub fn handle_client(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        if http_request.len() < 1 {
            println!("{:?}", http_request);
            return
        }

        println!("client request: {}", http_request[0]);

        let content_type = String::from("text/html");
        let request: Vec<&str> = http_request[0]
            .as_str()
            .split("?")
            .collect();

        for strings in request {
            println!("{}", strings);
        }

        let request = http_request.get(0).unwrap();

        let mut response: Result<Response, std::io::Error> = Err(Error::new(ErrorKind::Other, "Not initialized"));

        if request.starts_with("GET") {
            let request = &request[4..]; // remove GET part of request

            // checks if the HTTP request has extra parameters
            let contains_params = request.contains("?"); 
            let request = request.replace("?", " "); 

            let words: Vec<_> = request.split(" ").collect();
            let request = String::from(*words.get(0).unwrap());

            let params = match contains_params {
                true => {
                    let param_string = String::from(*words.get(1).unwrap());
                    let param_split= param_string.split("&"); 
                    let mut param_vec: Vec<Param> = vec![];

                    for param in param_split {
                        let param = Param::new(param); 
                        if param.is_some() {
                            param_vec.push(param.unwrap());
                        }
                    }

                    Some(param_vec)
                }
                false => None, 
            };

            let req_clone = request.clone();

            let cur_request = Request::new(
                String::from("GET"),
                request,
                params.unwrap_or(vec![])
            );

            for route in &self.routes {
                if route.path == req_clone {
                    println!("Route path: \"{}\"", route.path);
                    response = route.get_result(cur_request);

                    break;
                } 
            }

            // if no route, check for matching files
            if response.is_err() {
                let e = response.unwrap_err();
                match e.kind() {
                    ErrorKind::Other => { 
                        println!("sending: {}{}", self.root, req_clone);
                        let path = format!("{}{}", self.root, req_clone);
                        response = Response::ok(fs::read(path)
                            .unwrap_or(Vec::from(String::from("")))
                        )
                    },
                    _ => panic!("{:?}", e),
                }
            }
        }
        
        let response = response.unwrap_or(Response {
            code: "400 Bad Request",
            contents: Vec::from(String::from("Bad Request")),
        });

        send_response(content_type, &response, &mut stream).unwrap();
    }
}

// helper functions

fn send_response(content_type: String, response: &Response, stream: &mut TcpStream) -> Result<(), std::io::Error> {
    let write_string = format!("HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n",
        response.code,
        content_type,
        response.contents.len()
    );
    let write_string = write_string.as_bytes();

    let res1 = stream.write_all(write_string);
    if res1.is_err() {
        res1
    } else {
        stream.write_all(response.contents.as_slice())
    }
}

#[derive(Debug)]
pub struct Param {
    pub key: String,
    pub value: String,
}

impl Param {
    fn new(param_string: &str) -> Option<Param> {
        let mut split_param = param_string.split("=");
        
        let key = split_param.next();
        if key.is_none() {
            return None; 
        } 
        let key = key.unwrap();

        let value = split_param.next();
        if value.is_none() {
            return None;
        }
        let value = value.unwrap();

        // there should be only 2 values in the split
        if split_param.next().is_some() {
            return None;
        }

        Some(Param{ key: String::from(key), value: String::from(value) })
    }
}
