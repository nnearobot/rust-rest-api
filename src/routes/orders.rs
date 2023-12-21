use crate::http::{router::Router, *};

pub fn create() -> Router<'static> {
    Router::new("/orders")
        // The application MUST, upon creation request, store the item, the table number, and how long the item will take to cook.
        .post("/", post_order)

        // The application MUST, upon deletion request, remove a specified item for a specified table number.
        .delete("/:order_id", delete_order)
}

fn post_order(request: &str, params: &Vec<&str>) -> (String, String) {
    (OK_RESPONSE.to_string(), format!("Called: post_order no params"))
}

fn delete_order(request: &str, params: &Vec<&str>) -> (String, String) {
    (OK_RESPONSE.to_string(), format!("Called: delete_order with order_id = {}", params[0]))
}
