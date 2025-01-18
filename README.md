# wind
A lightweight, multithreaded rust web framework built with no external dependencies. It is inspired by express.js and uses similar syntax when creating endpoints for a REST API.
This project is a work in progress and still needs more documentation and testing, use caution if trying it in your own project

## Motivation
Creating this project was out of personal interest to delve deeper into how HTTP servers parse requests and send responses. It also provided an opportunity to do some work with Rust and low level socket programming.

## Features:
### Server:
To create a server, use:
```rust
fn main() {
    let server = &mut Server::new();

    /* your routes... */

    let address = "127.0.0.1:80";
    server.listen(address);
}
```

### Routing:
Creating routes for your server is done like so:
```rust
fn main() {
    /* ... */

    server.get("/hello", Arc::new(|_req: &mut Request, _res: &mut Response| {
        _res.send(OK, Vec::from("Hello from your webserver!");
    }));

    /* ... */
}
```
Routes support the default HTTP request methods: `GET, POST, PUT` and `DELETE`.
Use `sudo cargo run` to start the server: then use `curl http://localhost:80/hello` or open the URL in a web browser to see the message

You can also use the `Router` struct to organize routing

### Middleware
The server supports middleware function. To create a middleware function, use `server.use_middleware`
```rust
fn main() {
    /* ... */
    
    server.use_middleware(Arc::new(|_req: &mut Request, _res: &mut Response| {
        // log the HTTP code of the Response
        println!("{}: {}", _req.query, _res.code.as_str());

        _res.next();
    });

    /* ... */
}
```

See the docs for more info (work in progress)

