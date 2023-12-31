use crate::database::model::Model;
use std::fmt;

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
    pub fn get_all_items() -> Result<Vec<MenuOutput>, String> {
        let rows = Self::query_all_rows()?;
        let mut menu_items = Vec::new();
        for row in rows {
            menu_items.push(MenuOutput {
                id: row.get("menu_id"),
                name: row.get("menu_name"),
                description: row.get("menu_description"),
                time_to_cook_in_minutes: row.get("time_to_cook_in_minutes"),
            });
        }
        Ok(menu_items)
    }
}


#[derive(Serialize, Deserialize)]
pub struct MenuOutput {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub time_to_cook_in_minutes: i32,
}

impl fmt::Debug for MenuOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}. {} ({}), time: {} minutes",
            self.id,
            self.name,
            self.description,
            self.time_to_cook_in_minutes
        )
    }
}