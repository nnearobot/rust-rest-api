use crate::database::model::Model;

/// A model for the `table` DB table.
/// 
/// DB table: `table`
/// 
/// ## Fields:
/// 
/// - `id` - i32 - table identificator
/// - `description` - String - description of the table, e.g. a place in the restaurant.

#[derive(Serialize, Deserialize)]
pub struct Table {
    pub id: Option<i32>,
    pub description: String,
}

impl Model for Table {
    const TABLE_NAME: &'static str = "table";
}

impl Table {
    /// Returns all table list.
    pub fn get_all_tables() -> Result<Vec<Table>, String> {
        let rows = Self::query_all_rows()?;
        let mut tables = Vec::new();
        for row in rows {
            tables.push(Table {
                id: row.get(0),
                description: row.get(1),
            });
        }
        Ok(tables)
    }
}
