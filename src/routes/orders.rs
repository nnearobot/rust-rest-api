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
/// GET `/orders/:order_id` returns a specified order data.
///
/// POST `/orders` creates an order for a specified table with a specified list of menu items
/// and returns all currently preparing items for this table.
pub fn create() -> Router<'static> {
    Router::new("/orders")
        // The application must be able to give a quick snapshot of any or all items on its list at any time.
        .get("/", get_orders)
        .get("/:order_id", get_one_order)

        // The client MUST be able to: add one or more items with a table number.
        // The application MUST, upon creation request, store the item, the table number, and how long the item will take to cook.
        .post("/", post_order)
}

fn get_orders(request: &str, params: &Vec<&str>) -> (String, String) {
    match Order::get_all_cooking() {
        Ok(items) => (OK_RESPONSE.to_string(), serde_json::to_string(&items).unwrap()),
        Err(error) => (INTERNAL_SERVER_ERROR.to_string(), error),
    }
}

fn get_one_order(request: &str, params: &Vec<&str>) -> (String, String) {
    // params[0] is an order ID
    let order_id = match params[0].parse::<i32>() {
        Ok(id) => id,
        _ => return (BAD_REQUEST.to_string(), "Invalid order identificator".to_string()),
    };

    match Order::get_one(order_id) {
        Ok(order) => (OK_RESPONSE.to_string(), serde_json::to_string(&order).unwrap()),
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
        Ok(order_params) => {
            let table_id = order_params.table_id;
            match Order::create(order_params) {
                Ok(_) => {
                    return match Order::get_active_for_tables(vec![table_id]) {
                        Ok(orders) => (OK_RESPONSE.to_string(), serde_json::to_string(&orders).unwrap()),
                        Err(error) => (INTERNAL_SERVER_ERROR.to_string(), error),
                    }
                },
                Err(error) => (INTERNAL_SERVER_ERROR.to_string(), error),
            }
        },
        _ => (BAD_REQUEST.to_string(), "Error parcing json data".to_string()),
    }
}

