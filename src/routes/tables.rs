use crate::http::{router::Router, *};
use crate::database::models::table::Table;

pub fn create() -> Router<'static> {
    Router::new("/tables")
        .get("/", get_tables)
        
        // The application MUST, upon query request, show all items for a specified table number.
        // The client MAY limit the number of specific tables in its requests to a finite set (at least 100).
        .get("/:table_id_list/orders/", get_table_orders)

        // The application MUST, upon query request, show a specified item for a specified table number.
        .get("/:table_id/orders/:order_id", get_order_for_table)
}

fn get_tables(request: &str, params: &Vec<&str>) -> (String, String) {
    match Table::get_all_tables() {
        Ok(items) => (OK_RESPONSE.to_string(), serde_json::to_string(&items).unwrap()),
        Err(error) => (INTERNAL_SERVER_ERROR.to_string(), error.to_string()),
    }
}

fn get_table_orders(request: &str, params: &Vec<&str>) -> (String, String) {
    (OK_RESPONSE.to_string(), format!("Called: get_table_orders with table_id_list = {}", params[0]))
}

fn get_order_for_table(request: &str, params: &Vec<&str>) -> (String, String) {
    (OK_RESPONSE.to_string(), format!("Called: get_order_for_table with table = {} and order = {}", params[0], params[1]))
}

