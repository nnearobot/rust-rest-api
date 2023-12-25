use std::{
    str::FromStr,
    collections::HashMap,
};
use std::fmt::{self, Display, Formatter};

use super::method::Method;

const DYN_PATH_KEY: &str = ":dyn";

type Handler = fn(&str, &Vec<&str>) -> (String, String);
pub type Handlers<'a> = HashMap<Method, Handler>;
pub type Routes<'a> = HashMap<&'a str, Router<'a>>;

/// The struct that defines endpoints and their handlers.
pub struct Router<'a> {
    base: Option<&'a str>,
    pub handlers: Handlers<'a>,
    pub routes: Routes<'a>,
}


impl<'a> Router<'a> {
    /// Creates a new `Router`.
    ///
    /// # Arguments
    /// 
    /// * `base` - a path that will precede all paths in this `Router`.
    /// 
    /// # Panics
    /// 
    /// Panics if the base is not empty and doesn't start with `/` char.
    pub fn new(base: &'a str) -> Self {

        // The base may be empty, `/` or some other string started with `/`.
        // If it is not empty, we split it by `/` and get at least 2 elements: "" and "base" (or empty, too).
        // The second element is what we need: this is either base or empty string that means root, the same as None.
        let base: Option<&str> = match base {
            "" => None,
            _ => {
                if base.is_empty() {
                    panic!("Endpoint must not be empty. Use \"/\" for root.");
                }
                if !base.starts_with("/") {
                    panic!("Endpoint must start with a \"/\".");
                }
            
                match base.split("/").nth(1).unwrap_or_default() {
                    b if !b.is_empty() => Some(b),
                    _ => None,
                }
            }
        };

        Self {
            base,
            handlers: HashMap::new(),
            routes: HashMap::new(),
        }
    }

    /// Merges all routes from another router to current one under it's base.
    ///
    /// **Current router has an ownership of another router here.**
    /// 
    /// # Arguments
    /// 
    /// * `other_router` - a router which routes should be merged to current router.
    pub fn merge_from(mut self, mut other_router: Router<'a>) -> Self {
        match self.base {
            None => self.routes.extend(other_router.routes.drain()),
            Some(b) => self.routes.entry(b).or_insert(Router::new("")).routes.extend(other_router.routes.drain()),
        }

        self
    }

    /// Returns a handler for specified endpoint and method.
    /// 
    /// # Arguments
    /// 
    /// * `endpoint` - an endpoint path;
    /// * `method` - a string representation of the request method.
    pub fn get_handler(&self, endpoint: &'a str, method: &str) -> Result<(&Handler, Vec<&str>), String> {
        let method = Method::from_str(method)?;

        let handler = self.handler(Self::get_path(endpoint), 0, method, Vec::new());
        return match handler {
            Ok((h, p)) => Ok((h, p)),
            Err(error) => Err(error),
        }
    }

    /// A recursive function that iterates all the endpoint's paths of the router and returns handler for the specified method
    fn handler(&self, path: Vec<&'a str>, depth: usize, method: Method, mut params: Vec<&'a str>) -> Result<(&Handler, Vec<&str>), String> {

        // If we reached a path's end (there is no next element in path),
        // we should store a handler in current router's handlers
        if depth == path.len() {
            return match self.handlers.get(&method) {
                None => Err(format!("No handler for {} method found.", method)),
                Some(hh) => Ok((hh, params)),
            }
        }

        // We try to get a router by the key where the key is an endpoint's path.
        if self.routes.contains_key(path[depth]) {
            return self.routes.get(path[depth]).unwrap().handler(path, depth + 1, method, params);
        }

        // If there no such key in routes, we try to check if there is a DYN_PATH_KEY key.
        // If it is, we suppose that this is a variable parameter and store it in params.
        // And then we proceed with DYN_PATH_KEY's router.
        if self.routes.contains_key(DYN_PATH_KEY) {
            params.push(path[depth]);
            return self.routes.get(DYN_PATH_KEY).unwrap().handler(path, depth + 1, method, params);
        }

        // If no related key found this is a 404 error
        return Err("Undefined endpoint.".to_string());
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
    pub fn get(mut self, endpoint: &'a str, handler: Handler) -> Self {
        self.add_route(Self::get_path(endpoint), 0, Method::Get, handler);

        self
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
    pub fn post(mut self, endpoint: &'a str, handler: Handler) -> Self {
        self.add_route(Self::get_path(endpoint), 0, Method::Post, handler);

        self
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
    pub fn delete(mut self, endpoint: &'a str, handler: Handler) -> Self {
        self.add_route(Self::get_path(endpoint), 0, Method::Delete, handler);

        self
    }

    /// A recursive function that fills a router with all the endpoint's paths recursively
    fn add_route(&mut self, path: Vec<&'a str>, depth: usize, method: Method, handler: Handler) {
        // If the current router has a base option, we should store a newly added routes under it base's key router.
        let mut router = match self.base {
            None => self,
            Some(b) => self.routes.entry(b).or_insert(Router::new("")),
        };

        // If we reached a path's end (there is no next element in path),
        // we should store a handler in current router's handlers
        if depth == path.len() {
            router.handlers
                .entry(method)
                .or_insert(handler);
            return;
        }

        // The key is a path's part.
        // If the key starts with `:`this means that this part is dynamicly generated,
        // i.e. it represents some variable.
        // We store this route in a specific key `:dyn`.
        let key = if path[depth].starts_with(":") { DYN_PATH_KEY } else { path[depth] };
        router = router.routes
            .entry(key)
            .or_insert(Router::new(""));

        // Proceed to the next part of the path storing it under current router
        router.add_route(path, depth + 1, method, handler);
    }

    fn get_path(endpoint: &str) -> Vec<&str> {
        endpoint.split("/").filter(| x | !x.is_empty()).collect()
    }
}


/// Formats the Router instance for checking purpoises
impl<'a> Display for Router<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fn print_routes(routes: &Routes, f: &mut Formatter<'_>, depth: usize) -> fmt::Result {
            for (key, router) in routes {
                writeln!(f, "{:indent$}/{}", "", key, indent = depth * 2)?;
                for (method, handler) in &router.handlers {
                    writeln!(f, "{:indent$}-{}", "", method, indent = (depth + 1) * 2)?;
                }
                print_routes(&router.routes, f, depth + 1)?;
            }
            Ok(())
        }

        print_routes(&self.routes, f, 0)
    }
}