use std::vec::Vec;
use std::sync::Arc;

use crate::{
    route::Route,
    request::Request,
    response::Response,
};

#[derive(Clone)]
pub struct Router {
    pub routes: Vec<Route>,   
    pub routers: Vec<Router>,
    pub root: String,
}

impl Router {
    pub fn new(root: &str) -> Router {
        assert!(root.starts_with("/"));

        Router {
            root: root.to_string(), routes: Vec::new(), routers: Vec::new(),
        }
    }

    pub fn get(&mut self, path: &str, result: Arc<dyn Fn(&mut Request, &mut Response) + Send + Sync + 'static>) {
        assert!(path.starts_with("/"));

        let new_route = Route::new(path, "GET", result); 
        self.routes.push(new_route); 
    }
    
    pub fn post(&mut self, path: &str, result: Arc<dyn Fn(&mut Request, &mut Response) + Send + Sync + 'static>) {
        assert!(path.starts_with("/"));
        
        let new_route = Route::new(path, "POST", result); 
        self.routes.push(new_route); 
    }
    
    pub fn put(&mut self, path: &str, result: Arc<dyn Fn(&mut Request, &mut Response) + Send + Sync + 'static>) {
        assert!(path.starts_with("/"));

        let new_route = Route::new(path, "PUT", result); 
        self.routes.push(new_route); 
    }

    pub fn delete(&mut self, path: &str, result: Arc<dyn Fn(&mut Request, &mut Response) + Send + Sync + 'static>) {
        assert!(path.starts_with("/"));

        let new_route = Route::new(path, "DELETE", result); 
        self.routes.push(new_route); 
    }

    pub fn use_middleware(&mut self, result: Arc<dyn Fn(&mut Request, &mut Response) + Send + Sync + 'static>) {
        let new_middleware = Route::middleware(result);    
        self.routes.push(new_middleware);
    }

    pub fn use_router(&mut self, router: Router) {
        self.routers.push(router);
    }

    pub fn handle_client(&self, query: String, req: &mut Request, res: &mut Response) -> bool {
        let mut res_sent = false;

        for route in self.routes.clone() {
            for router in &self.routers {
                if query.starts_with(router.root.as_str()) {
                    let sheared_query = query[router.root.len()..].to_string();

                    res_sent = router.handle_client(sheared_query, req, res);
                }
            }

            if (route.path == query && route.method == req.method && !res_sent) || route.middleware {
                route.get_result(req, res);

                if !res.next {
                    res_sent = true;
                }
            }
        } 

        res_sent
    }
}

