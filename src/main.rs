use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    sync::Arc,
    //time::Duration,
    //thread,
};


#[macro_use]
extern crate serde_derive;

const SERVER_URI: &str = "127.0.0.1";
const SERVER_PORT: &str = "7878";
const THREADS_NUMBER: usize = 20;

mod thread_pool;
mod http;
mod routes;
mod database;

use http::router::Router;
use thread_pool::*;


fn main() {
    let listener = TcpListener::bind(format!("{}:{}", SERVER_URI, SERVER_PORT)).unwrap();
    println!("Server has started on port {}", SERVER_PORT);

    let router = Arc::new(routes::create("/v1"));

    println!("{}", router); // temporarily for testing

    let pool = ThreadPool::new(THREADS_NUMBER);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let router = Arc::clone(&router);

        pool.execute(|| {
            handle_connection(stream, router);
        });
    }

    println!("Shutting down.");
}


fn handle_connection(mut stream: TcpStream, router: Arc<Router>) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let request_arr: Vec<&str> = request
                .lines()
                .next()
                .unwrap()
                .split_whitespace()
                .collect();

            //thread::sleep(Duration::from_secs(5)); // temporarily for multithreading testing

            let (status_line, content) = match router.get_handler(request_arr[1].split("?").nth(0).unwrap(), request_arr[0]) {
                Ok((handler, params)) => handler(&request, &params),
                Err(error) => (http::NOT_FOUND.to_string(), error),
            };

            let response = format!("HTTP/1.1 {status_line}\r\n\r\n{content}");
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

