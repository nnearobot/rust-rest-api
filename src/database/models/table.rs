use crate::database::model::Model;

/// Table model
/// 
/// DB table: `table`
/// 
/// - id - i32 - client id
/// - description - String - description of the client: which table it belongs to

#[derive(Serialize, Deserialize)]
pub struct Table {
    pub id: Option<i32>,
    pub description: String,
}

impl Model for Table {
    const TABLE_NAME: &'static str = "table";
}

impl Table {
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
