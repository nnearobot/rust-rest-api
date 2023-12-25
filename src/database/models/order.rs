use std::time::SystemTime;
use crate::database::model::Model;

use super::menu::Menu;

/// Order model
/// 
/// DB table: `order`
/// 
/// - id - i32 - order id
/// - table_id - i32 - table id this order is for
/// - menu_id - i32 - menu item id
/// - cooked_at - SystemTime - time when this item will be prepared
/// - is_deleted - bool - is this order deleted or not deleted (default)
/// - created_at - SystemTime - order time
/// - updated_at - SystemTime - most recent update time

pub const TABLE_NAME: &str = "order";

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
    pub fn create_order(order: Order) -> Result<u64, String> {
        Self::execute(&format!("INSERT INTO \"{}\" (table_id, menu_id, cooked_at, is_deleted, created_at, updated_at)
            VALUES (
                $1,
                $2,
                NOW() + INTERVAL '1 minute' * (SELECT time_to_cook_in_minutes FROM \"{}\" WHERE id = $2),
                FALSE,
                NOW(),
                NOW()
        );", Self::TABLE_NAME, Menu::TABLE_NAME), &[&order.table_id, &order.menu_id])
    }
}

