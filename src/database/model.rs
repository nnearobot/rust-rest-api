use postgres::{Row, types::ToSql};

use crate::database;

/// Model trait. All models should implement it.
pub trait Model {
    const TABLE_NAME: &'static str;

    fn query_all_rows() -> Result<Vec<Row>, String> {
        match database::client() {
            Ok(mut client) => 
                match client.query(&format!("SELECT * FROM \"{}\"", Self::TABLE_NAME), &[]) {
                    Ok(items) => Ok(items),
                    Err(error) => Err(error.to_string()),
                },
            _ => Err(database::CONECTION_ERROR.to_string()),
        }
    }

    fn execute(query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, String> {
        match database::client() {
            Ok(mut client) => 
                match client.execute(query, params) {
                    Ok(rows) => Ok(rows),
                    Err(error) => Err(error.to_string()),
                },
            _ => Err(database::CONECTION_ERROR.to_string()),
        }
    }

}
