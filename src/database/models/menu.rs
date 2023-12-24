/// Menu model
/// 
/// DB table: `menu`
/// 
/// - id - i32 - menu item id
/// - name - String - name of the menu item
/// - description - String - description of the menu item
/// - time_to_cook_in_minutes - i32 - time to prepare this item in minutes

pub const TABLE_NAME: &str = "menu";

#[derive(Serialize, Deserialize)]
pub struct Menu {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub time_to_cook_in_minutes: i32,
}

