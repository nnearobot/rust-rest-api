use crate::database::model::Model;

/// A model for the `menu` DB table.
/// 
/// DB table: `menu`
/// 
/// ## Fields:
/// 
/// - `id` - i32 - menu item id
/// - `name` - String - name of the dish
/// - `description` - String - description
/// - `time_to_cook_in_minutes` - i32 - time to prepare this item in minutes.
#[derive(Serialize, Deserialize)]
pub struct Menu {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub time_to_cook_in_minutes: i32,
}

impl Model for Menu {
    const TABLE_NAME: &'static str = "menu";
}

impl Menu {
    /// Returns the entire menu.
    pub fn get_all_items() -> Result<Vec<Menu>, String> {
        let rows = Self::query_all_rows()?;
        let mut menu_items = Vec::new();
        for row in rows {
            menu_items.push(Menu {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                time_to_cook_in_minutes: row.get("time_to_cook_in_minutes"),
            });
        }
        Ok(menu_items)
    }
}
