use crate::http::{router::Router, *};
use crate::database::models::{
    table::Table,
    order::Order,
};

/// Creates a router for `/tables` endpoint
/// 
/// GET `/tables` returns a list of all tables.
/// 
/// GET `/tables/:table_id_list/orders/` returns all items for a specified table list.
/// 
/// GET `/tables/:table_id/orders/:order_id` returns a specified item for a specified table number.
/// 
/// DELETE `/tables/:table_id/orders/:order_id` removes a specified item for a specified table number
/// and returns all currently preparing items for this table.
pub fn create() -> Router<'static> {
    Router::new("/tables")
        .get("/", get_tables)
        
        // The application MUST, upon query request, show all items for a specified table number.
        // The client MAY limit the number of specific tables in its requests to a finite set (at least 100).
        .get("/:table_id_list/orders/", get_table_orders)

        // The application MUST, upon query request, show a specified item for a specified table number.
        .get("/:table_id/orders/:order_id", get_order_for_table)

        // The application MUST, upon deletion request, remove a specified item for a specified table number.
        .delete("/:table_id/orders/:order_id", delete_order_for_table)
}


fn get_tables(_: &str, _: &Vec<&str>) -> (String, String) {
    match Table::get_all_tables() {
        Ok(items) => (OK_RESPONSE.to_string(), serde_json::to_string(&items).unwrap()),
        Err(error) => (INTERNAL_SERVER_ERROR.to_string(), error),
    }
}

fn get_table_orders(_: &str, params: &Vec<&str>) -> (String, String) {
    // params[0] is a list of table IDs (or just one table ID)
    let table_list_str: Vec<&str> = params[0].split(",").collect();
    // Parse the string IDs to i32, filtering out any invalid IDs
    let table_list_i32: Vec<i32> = table_list_str.iter()
        .filter_map(|&s| s.parse::<i32>().ok())
        .collect();

    if table_list_i32.is_empty() {
        return (BAD_REQUEST.to_string(), "Invalid table identificator".to_string());
    }

    match Order::get_active_for_tables(table_list_i32) {
        Ok(orders) => (OK_RESPONSE.to_string(), serde_json::to_string(&orders).unwrap()),
        Err(error) => (INTERNAL_SERVER_ERROR.to_string(), error),
    }
}

fn get_order_for_table(_: &str, params: &Vec<&str>) -> (String, String) {
    // params[0] is a table ID
    let table_id = match params[0].parse::<i32>() {
        Ok(id) => id,
        _ => return (BAD_REQUEST.to_string(), "Invalid table identificator".to_string()),
    };
    
    // params[1] is an order ID
    let order_id = match params[1].parse::<i32>() {
        Ok(id) => id,
        _ => return (BAD_REQUEST.to_string(), "Invalid order identificator".to_string()),
    };

    match Order::get_one_for_table(table_id, order_id) {
        Ok(order) => (OK_RESPONSE.to_string(), serde_json::to_string(&order).unwrap()),
        _ => (BAD_REQUEST.to_string(), "Invalid order identificator for the requested table".to_string()),
    }
}

fn delete_order_for_table(_: &str, params: &Vec<&str>) -> (String, String) {
    // params[0] is a table ID
    let table_id = match params[0].parse::<i32>() {
        Ok(id) => id,
        _ => return (BAD_REQUEST.to_string(), "Invalid table identificator".to_string()),
    };
    
    // params[1] is an order ID
    let order_id = match params[1].parse::<i32>() {
        Ok(id) => id,
        _ => return (BAD_REQUEST.to_string(), "Invalid order identificator".to_string()),
    };

    match Order::delete_one_for_table(table_id, order_id) {
        Ok(rows_modified) => {
            if rows_modified > 0 {
                return match Order::get_active_for_tables(vec![table_id]) {
                    Ok(orders) => (OK_RESPONSE.to_string(), serde_json::to_string(&orders).unwrap()),
                    Err(error) => (INTERNAL_SERVER_ERROR.to_string(), error),
                }
            }

            (BAD_REQUEST.to_string(), "No order deleted".to_string())
        },
        Err(error) => (INTERNAL_SERVER_ERROR.to_string(), error),
    }
}

