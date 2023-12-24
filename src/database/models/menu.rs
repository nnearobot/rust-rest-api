use crate::database::model::Model;

/// Menu model
/// 
/// DB table: `menu`
/// 
/// - id - i32 - menu item id
/// - name - String - name of the menu item
/// - description - String - description of the menu item
/// - time_to_cook_in_minutes - i32 - time to prepare this item in minutes

#[derive(Serialize, Deserialize)]
pub struct Menu {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub time_to_cook_in_minutes: i32,
}

impl Model for Menu {
    const TABLE_NAME: &'static str = "menu";
}

impl Menu {
    pub fn get_all_items() -> Result<Vec<Menu>, String> {
        let rows = Self::query_all_rows()?;
        let mut menu_items = Vec::new();
        for row in rows {
            menu_items.push(Menu {
                id: row.get(0),
                name: row.get(1),
                description: row.get(2),
                time_to_cook_in_minutes: row.get(3),
            });
        }

        Ok(menu_items)
    }
}
