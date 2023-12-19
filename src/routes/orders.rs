use crate::http::router::Router;

pub fn create() -> Router {
    Router::new("/orders")
        .get("/", get_orders)
        .post("/", post_order)
        .delete("/", delete_order)
}

fn get_orders(request: &str) -> (String, String) {
    println!("Called: get_orders");
    ("".to_string(), "".to_string())
}

fn post_order(request: &str) -> (String, String) {
    println!("Called: post_order");
    ("".to_string(), "".to_string())
}

fn delete_order(request: &str) -> (String, String) {
    println!("Called: delete_order");
    ("".to_string(), "".to_string())
}
