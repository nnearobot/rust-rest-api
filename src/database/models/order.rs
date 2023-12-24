use std::time::SystemTime;
use crate::database::model::Model;

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
    pub fn new<A, B>(table_id: A, menu_id: B) -> Self
    where
        A: Into<i32>,
        B: Into<i32>,
    {
        Self {
          id: None,
          table_id: table_id.into(),
          menu_id: menu_id.into(),
          cooked_at: None,
          is_deleted: None,
          created_at: None,
          updated_at: None,
        }
    }


}

