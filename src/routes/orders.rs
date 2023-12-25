use crate::{
    http::{router::Router, *}, 
    database::models::order::Order
};

pub fn create() -> Router<'static> {
    Router::new("/orders")
        // The application MUST, upon creation request, store the item, the table number, and how long the item will take to cook.
        .post("/", post_order)

        // The application MUST, upon deletion request, remove a specified item for a specified table number.
        .delete("/:order_id", delete_order)
}

fn post_order(request: &str, params: &Vec<&str>) -> (String, String) {
    let params_json = request
        .split("\r\n\r\n")
        .last()
        .unwrap_or_default();

    let order: Result<Order, serde_json::Error> = serde_json::from_str(params_json);
    match order {
        Ok(order) => 
            match Order::create_order(order) {
                Ok(_) => {
                    (OK_RESPONSE.to_string(), "Order created".to_string())
                },
                Err(error) => (INTERNAL_SERVER_ERROR.to_string(), error.to_string()),
            },
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error parcing json data".to_string()),
    }
}

fn delete_order(request: &str, params: &Vec<&str>) -> (String, String) {
    (OK_RESPONSE.to_string(), format!("Called: delete_order with order_id = {}", params[0]))
}
