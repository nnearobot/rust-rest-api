use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::Arc,
    //time::Duration,
    //thread,
};

const SERVER_URI: &str = "127.0.0.1";
const SERVER_PORT: &str = "7878";
const THREADS_NUMBER: usize = 20;

mod thread_pool;
mod http;
mod routes;

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
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let request_arr: Vec<&str> = request_line.split("?").nth(0).unwrap().split_whitespace().collect();

    println!("{}",request_line); // temporarily for testing
    //thread::sleep(Duration::from_secs(5)); // temporarily for multithreading testing

    let (status_line, content) = match router.handler(request_arr[1], request_arr[0]) {
        Ok((handler, params)) => handler(&request_line, &params),
        Err(error) => (http::NOT_FOUND.to_string(), error),
    };

    let response = format!("{status_line}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}