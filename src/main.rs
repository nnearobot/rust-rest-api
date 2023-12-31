use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    sync::Arc,
    env, usize,
    time::Duration,
    thread,
    str
};
use rand::Rng;

#[macro_use]
extern crate serde_derive;

const SERVER_URI: &str = env!("SERVER_URI");
const SERVER_PORT: &str = env!("SERVER_PORT");
const THREADS_NUMBER: &str = env!("THREADS_NUMBER");

mod thread_pool;
mod http;
mod routes;
mod database;

use http::router::Router;
use serde_json::json;
use thread_pool::*;

use crate::database::models::{menu::MenuOutput, order::OrderOutput};


fn main() {
    // Warm up database
    if let Err(e) = database::set_database() {
        println!("{}", e);
        return;
    }

    let listener = TcpListener::bind(get_server_address()).unwrap();
    println!("Server has started on port {}", SERVER_PORT);

    let router = Arc::new(routes::create("/v1"));

    println!("{}", router); // temporarily for testing

    let pool = ThreadPool::new(THREADS_NUMBER.parse::<usize>().unwrap_or(1) + 10);

    // “Clients” can be simulated as simple threads calling the main server application with a variety of requests.
    // There should be more than one, preferably around 5-10 running at any one time.
    for table_num in 1..=10 {
        pool.execute(move || {
            client(table_num);
        });
    }

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

fn get_server_address() -> String {
    format!("{}:{}", SERVER_URI, SERVER_PORT)
}



/// Simulates client requests
fn client(table_id: usize) {
    println!("Table {} client started", table_id);
    thread::sleep(Duration::from_secs(5));


    // List a menu
    let menu_list = reqwest::blocking::get(get_client_address("/v1/menu/"))
        .unwrap()
        .json::<Vec<MenuOutput>>()
        .unwrap();
    println!("Table {} menu: {:#?} items", table_id, menu_list.len());
    let menu_count = menu_list.len();


    thread::sleep(Duration::from_secs(5));


    let mut rng = rand::thread_rng();

    // Make an order with 3 random menu items
    let order = json!({
        "table_id": table_id,
        "menu_id": [
            menu_list.get(rng.gen_range(0..menu_count)).unwrap().id,
            menu_list.get(rng.gen_range(0..menu_count)).unwrap().id,
            menu_list.get(rng.gen_range(0..menu_count)).unwrap().id
        ]
    });
    let client = reqwest::blocking::Client::new();
    let order_list = client.post(get_client_address("/v1/orders/"))
        .body(serde_json::to_string(&order).unwrap())
        .send()
        .unwrap()
        .json::<Vec<OrderOutput>>()
        .unwrap();
    println!("Table {} orders: {:#?}", table_id, order_list);


    thread::sleep(Duration::from_secs(5));


    // delete one random order for current table
    let client = reqwest::blocking::Client::new();
    let order_list = client.delete(get_client_address(&format!("/v1/tables/{}/orders/{}", table_id, order_list.get(rng.gen_range(0..order_list.len())).unwrap().id)))
        .send()
        .unwrap()
        .json::<Vec<OrderOutput>>()
        .unwrap();
    println!("Table {} orders after deletion: {:#?}", table_id, order_list);


    thread::sleep(Duration::from_secs(5));


    // Show a specified item for current table
    let order_id = order_list.get(rng.gen_range(0..order_list.len())).unwrap().id;
    let order = reqwest::blocking::get(get_client_address(&format!("/v1/tables/{}/orders/{}", table_id, order_id)))
        .unwrap()
        .json::<OrderOutput>()
        .unwrap();
    println!("{:#?}", order);


    thread::sleep(Duration::from_secs(200));

    // Show all items for current table
    let orders = reqwest::blocking::get(get_client_address(&format!("/v1/tables/{}/orders", table_id)))
        .unwrap()
        .json::<Vec<OrderOutput>>()
        .unwrap();
    println!("Table {} orders after 200 sec: {:#?}", table_id, orders);
}


fn get_client_address(url: &str) -> String {
    format!("http://{}{}", &get_server_address(), url)
}
