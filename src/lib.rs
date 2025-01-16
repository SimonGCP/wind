use std::{
    net::{
        TcpStream, 
        TcpListener
    },
    io::Write,
    vec::Vec,
    sync::{
        Arc,
        Mutex,
        mpsc,
    },
    thread,
};

pub mod route;
pub mod response;
pub mod request;

pub use route::Route;
pub use response::Response;
pub use request::Request;

pub struct Router {
    routes: Vec<Route>,   
    pool: ThreadPool,
}

impl Router {
    pub fn new() -> Router {
        let pool= ThreadPool::new(20);

        Router {
            routes: Vec::new(),
            pool,
        }
    }

    pub fn route(&mut self, path: &str, result: Arc<dyn Fn(&mut Request, &mut Response) + Send + Sync + 'static>) {
        let new_route = Route::new(path, "", result); 
        self.routes.push(new_route); 
    }

    pub fn get(&mut self, path: &str, result: Arc<dyn Fn(&mut Request, &mut Response) + Send + Sync + 'static>) {
        let new_route = Route::new(path, "GET", result); 
        self.routes.push(new_route); 
    }
    
    pub fn post(&mut self, path: &str, result: Arc<dyn Fn(&mut Request, &mut Response) + Send + Sync + 'static>) {
        let new_route = Route::new(path, "POST", result); 
        self.routes.push(new_route); 
    }
    
    pub fn put(&mut self, path: &str, result: Arc<dyn Fn(&mut Request, &mut Response) + Send + Sync + 'static>) {
        let new_route = Route::new(path, "PUT", result); 
        self.routes.push(new_route); 
    }

    pub fn delete(&mut self, path: &str, result: Arc<dyn Fn(&mut Request, &mut Response) + Send + Sync + 'static>) {
        let new_route = Route::new(path, "DELETE", result); 
        self.routes.push(new_route); 
    }

    pub fn listen(&self, address: &str) {
        let listener: TcpListener = TcpListener::bind(address).unwrap();

        println!("Starting server at {}", address);
        for client in listener.incoming() {
            match client {
                Ok(stream) => {
                    let routes = self.routes.clone();

                    self.pool.execute(move || {
                        Router::handle_client(routes, stream);
                    });
                }
                Err(e) => panic!("Error connecting: {e}"),
            }
        }
    }

    pub fn use_middleware(&mut self, result: Arc<dyn Fn(&mut Request, &mut Response) + Send + Sync + 'static>) {
        let new_middleware = Route::middleware(result);    
        self.routes.push(new_middleware);
    }

    pub fn handle_client(routes: Vec<Route>, mut stream: TcpStream) {
        let req = Request::new(&stream);
        if req.is_err() {
            Self::send_response(
                "text/html".to_string(), 
                &Response::new(response::HTTPCodes::BadRequest, Vec::from("could not parse request")),
                &mut stream
            ).unwrap();

            return;
        } 


        let mut req = req.unwrap();
        let mut res =  Response::empty();
        let mut res_sent = false;

        println!("body: {:?}", String::from_utf8(req.body.clone()));

        for route in routes {
            if (route.path == req.query && route.method == req.method && !res_sent) || route.middleware {
                route.get_result(&mut req, &mut res);

                if !res.next {
                    res_sent = true;
                }
            } 
        }

        Self::send_response("text/html".to_string(), &res, &mut stream).unwrap();
    }


    fn send_response(content_type: String, response: &Response, stream: &mut TcpStream) -> Result<(), std::io::Error> {
        let mut write_string = format!("HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n",
            response.code.as_str(),
            content_type,
            response.contents.len()
        );

        for header in response.headers.clone() {
            println!("header: {}", header);
            write_string.push_str(format!("{}\r\n", header).as_str());
        }

        write_string.push_str("\r\n");

        let write_string = write_string.as_bytes();

        let res1 = stream.write_all(write_string);
        if res1.is_err() {
            res1
        } else {
            stream.write_all(response.contents.as_slice())
        }
    }
}

// helper functions


#[derive(Debug, PartialEq)]
pub struct Param {
    pub key: String,
    pub value: String,
}

impl Param {
    pub fn new(param_string: &str) -> Option<Param> {
        // split between parameters and fragments
        let param_string = param_string.split("#").next().unwrap();
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

// multithreading structs

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    // println!("Worker {} got a job: executing", id);
                    job();
                },
                Message::Terminate => {
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    fn new(thread_count: usize) -> ThreadPool {
        assert!(thread_count > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(thread_count);
        for id in 0..thread_count {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
    
    pub fn execute<F>(&self, job: F)
        where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod router_tests {
    use super::*;
    
    /* test param string splitting */
    #[test]
    fn should_split() {
        let param_str = "name=simon";
        let param = Param::new(param_str);

        assert_eq!(param.unwrap(), Param{ key: String::from("name"), value: String::from("simon") });
    }

    #[test]
    fn should_split_fragment() {
        let param_str = "name=simon#bottom";
        let key = String::from("name");
        let value = String::from("simon");

        let param = Param::new(param_str);
        assert_eq!(param.unwrap(), Param{ key, value });
    }

    #[test]
    fn extra_equals_param() {
        let param_str = "name=simonage=21";
        let param = Param::new(param_str);

        assert_eq!(param, None); 
    }

    #[test]
    fn no_equals_param() {
        let param_str = "name";
        let param = Param::new(param_str);

        assert_eq!(param, None);
    }
}
