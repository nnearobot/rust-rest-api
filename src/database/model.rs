use postgres::Row;

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

}
