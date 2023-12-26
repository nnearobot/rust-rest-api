use std::time::SystemTime;
use crate::database::model::Model;

use super::menu::Menu;

/// A model for the `order` DB table.
/// 
/// DB table: `order`
/// 
/// ## Fields:
/// 
/// - `id` - i32 - order id
/// - `table_id` - i32 - table id this order is for
/// - `menu_id` - i32 - menu item id
/// - `cooked_at` - SystemTime - time when this item will be prepared
/// - `is_deleted` - bool - is this order deleted or not deleted (default)
/// - `created_at` - SystemTime - order time
/// - `updated_at` - SystemTime - most recent update time.
#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: Option<i32>,
    pub table_id: i32,
    pub menu_id: i32,
    pub cooked_at: Option<SystemTime>,
    pub is_deleted: Option<bool>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}

impl Model for Order {
    const TABLE_NAME: &'static str = "order";
}

impl Order {
    /// Returns all order items from database.
    pub fn get_all_items() -> Result<Vec<Order>, String> {
        let rows = Self::query_all_rows()?;
        let mut orders = Vec::new();
        for row in rows {
            orders.push(Order {
                id: row.get(0),
                table_id: row.get(1),
                menu_id: row.get(2),
                cooked_at: row.get(3),
                is_deleted: row.get(4),
                created_at: row.get(5),
                updated_at: row.get(6),
            });
        }
        Ok(orders)
    }

    /// Add an order with specified menu items to a database for specified table.
    /// 
    /// The time when the order item will be ready is calculated by adding the time it takes to prepare the specified menu item to the time the order is created.
    /// 
    /// Several menu items may be passed at once.
    pub fn create(order_params: OrderParams) -> Result<u64, String> {
        let mut query_string = format!("INSERT INTO \"{}\" (table_id, menu_id, cooked_at, is_deleted, created_at, updated_at) VALUES ", Self::TABLE_NAME);
        let mut values = Vec::new();
        for menu_id in order_params.menu_id {
            values.push(format!("(
                {},
                {},
                NOW() + INTERVAL '1 minute' * (SELECT time_to_cook_in_minutes FROM \"{}\" WHERE id = {}),
                FALSE,
                NOW(),
                NOW())", order_params.table_id, menu_id, Menu::TABLE_NAME, menu_id));
        }
        query_string += &values.join(", ");
        Self::execute(&query_string, &[])
    }

    /// Delete the order by its ID.
    /// 
    /// A record in a table will not be deleted but marked as deleted.
    /// 
    /// ##The order can not be deleted in the cases:
    /// 
    /// - if there is no order found with the specified ID
    /// - if the specified order does not belong to the specified table
    /// - if the specified order is already deleted
    /// - if the order is already prepared.
    pub fn delete_one_for_table(table_id: i32, order_id: i32) -> Result<u64, String> {
        Self::execute(&format!("
            UPDATE\"{}\"
            SET is_deleted = true
            WHERE id = $1
            AND table_id = $2
            AND is_deleted = false
            AND cooked_at > NOW()
        ", Self::TABLE_NAME), &[&order_id, &table_id])
    }

    /// Returns all orders for specified tables.
    pub fn get_all_for_tables(table_list: Vec<i32>) -> Result<Vec<Order>, String> {
        match Self::query(&format!("SELECT * FROM \"{}\" WHERE table_id = ANY($1)", Order::TABLE_NAME), &[&table_list]) {
            Ok(rows) => {
                let mut orders = Vec::new();
                for row in rows {
                    orders.push(Order {
                        id: row.get(0),
                        table_id: row.get(1),
                        menu_id: row.get(2),
                        cooked_at: row.get(3),
                        is_deleted: row.get(4),
                        created_at: row.get(5),
                        updated_at: row.get(6),
                    });
                }
                Ok(orders)
            },
            Err(error) => Err(error),
        }
    }


    /// Returns an order with specified ID for specified table.
    /// 
    /// If the specified order does not belong to a specified table nothing will be returned.
    pub fn get_one_for_table(table_id: i32, order_id: i32) -> Result<Order, String> {
        Self::query_one(&format!("SELECT * FROM \"{}\" WHERE table_id = $1 AND id = $2", Order::TABLE_NAME), &[&table_id, &order_id])
        .map(|row| Order {
            id: row.get(0),
            table_id: row.get(1),
            menu_id: row.get(2),
            cooked_at: row.get(3),
            is_deleted: row.get(4),
            created_at: row.get(5),
            updated_at: row.get(6),
        })
    }
}


#[derive(Serialize, Deserialize)]
pub struct OrderParams {
    pub table_id: i32,
    pub menu_id: Vec<i32>,
}