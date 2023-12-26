use crate::{
    http::{router::Router, *}, 
    database::models::order::{
        Order,
        OrderParams,
    },
};

/// Creates a router for `/orders` endpoint.
///
/// GET `/orders` returns a list of all items.
///
/// POST `/orders` creates an order for a specified table with a specified list of menu items.
pub fn create() -> Router<'static> {
    Router::new("/orders")
        .get("/", get_orders)

        // The client MUST be able to: add one or more items with a table number.
        // The application MUST, upon creation request, store the item, the table number, and how long the item will take to cook.
        .post("/", post_order)
}

fn get_orders(request: &str, params: &Vec<&str>) -> (String, String) {
    match Order::get_all_items() {
        Ok(items) => (OK_RESPONSE.to_string(), serde_json::to_string(&items).unwrap()),
        Err(error) => (INTERNAL_SERVER_ERROR.to_string(), error),
    }
}

fn post_order(request: &str, params: &Vec<&str>) -> (String, String) {
    let params_json = request
        .split("\r\n\r\n")
        .last()
        .unwrap_or_default();

    let order_params: Result<OrderParams, serde_json::Error> = serde_json::from_str(params_json);
    match order_params {
        Ok(order_params) => match Order::create(order_params) {
            Ok(_) => {
                (OK_RESPONSE.to_string(), "Order created".to_string())
            },
            Err(error) => (INTERNAL_SERVER_ERROR.to_string(), error),
        },
        _ => (BAD_REQUEST.to_string(), "Error parcing json data".to_string()),
    }
}

