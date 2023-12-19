use std::{
    str::FromStr,
    collections::HashMap,
};
use super::method::Method;

type Handler = fn(&str) -> (String, String);
pub type Handlers = HashMap<Method, Handler>;
pub type Routes = HashMap<String, Handlers>;

/// The struct that defines endpoints and its' handlers.
pub struct Router {
    base: String,
    pub routes: Routes,
}

impl Router {
    /// Creates a new `Router`.
    ///
    /// # Arguments
    /// 
    /// * `base` - a path that will precede all paths in this `Router`.
    pub fn new(base: &str) -> Self {
        Self {
            base: base.to_string(),
            routes: HashMap::new(),
        }
    }

    /// Returns a hashMap with all routes of this `Router`.
    pub fn get_routes(self) -> Routes {
        self.routes
    }

    /// Merges all routes from another router to current one under it's base.
    ///
    /// **Current router has an ownership of another router here.**
    /// 
    /// # Arguments
    /// 
    /// * `other_router` - a router which routes should be merged to current router.
    pub fn merge_from(mut self, other_router: Router) -> Self {
        for (endpoint, handlers) in other_router.get_routes() {
            for (method, handler) in handlers {
                self = match method {
                    Method::Get => self.get(&endpoint[..], handler),
                    Method::Post => self.post(&endpoint[..], handler),
                    Method::Delete => self.delete(&endpoint[..], handler),
                };
            }
        }
        self
    }

    /// Returns a handler for specified endpoint and method.
    /// 
    /// # Arguments
    /// 
    /// * `endpoint` - an endpoint path;
    /// * `method` - a string representation of the request method.
    pub fn get_handler<'a>(&'a self, endpoint: &'a str, method: &'a str) -> Result<&'a Handler, String> {
        let method = Method::from_str(method)?;
        
        self.routes
            .get(endpoint)
            .ok_or(format!("No handler for {} {} found.", method, endpoint))?
            .get(&method)
            .ok_or(format!("Undefined {} endpoint for {} method.", endpoint, &method))
        }

    /// Add GET method handler for a specified endpoint.
    ///
    /// # Arguments
    /// 
    /// * `endpoint` - a path relative to base;
    /// * `handler` - a pointer to function that handles.
    ///
    /// # Panics
    ///
    /// Panics if the endpoint is incorrect.
    pub fn get(self, endpoint: &str, handler: Handler) -> Self {
        self.route(Method::Get, endpoint, handler)
    }

    /// Add POST method handler for a specified endpoint.
    ///
    /// # Arguments
    /// 
    /// * `endpoint` - a path relative to base;
    /// * `handler` - a pointer to function that handles.
    ///
    /// # Panics
    ///
    /// Panics if the endpoint is incorrect.
    pub fn post(self, endpoint: &str, handler: Handler) -> Self {
        self.route(Method::Post, endpoint, handler)
    }

    /// Add DELETE method handler for a specified endpoint.
    ///
    /// # Arguments
    /// 
    /// * `endpoint` - a path relative to base;
    /// * `handler` - a pointer to function that handles.
    ///
    /// # Panics
    ///
    /// Panics if the endpoint is incorrect.
    pub fn delete(self, endpoint: &str, handler: Handler) -> Self {
        self.route(Method::Delete, endpoint, handler)
    }

    fn route(mut self, method: Method, endpoint: &str, handler: Handler) -> Self {
        let endpoint = format!("{}{}", &self.base, endpoint);
        if endpoint.is_empty() {
            panic!("Endpoint must not be empty. Use \"/\" for root.");
        }
        if !endpoint.starts_with('/') {
            panic!("Endpoint must start with a \"/\".");
        }

        self.routes
            .entry(endpoint)
            .or_insert(HashMap::new())
            .insert(method, handler);

        self
    }
}