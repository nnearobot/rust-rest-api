use postgres::{Row, types::ToSql};

use crate::database;

/// Model trait. All models should implement it.
/// Share a default implementation for a database queries.
pub trait Model {
    const TABLE_NAME: &'static str;

    fn query_all_rows() -> Result<Vec<Row>, String> {
        database::client()?
            .query(&format!("SELECT * FROM \"{}\"", Self::TABLE_NAME), &[])
            .map_err(|error| error.to_string())
    }

    /// Executes a statement, returning the resulting rows.
    /// 
    /// A statement may contain parameters, specified by $n, where n is the index of the parameter of the list provided, 1-indexed.
    fn query(query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, String> {
        database::client()?
            .query(query, params)
            .map_err(|error| error.to_string())
    }
    
    /// Executes a statement which returns a single row, returning it.
    ///
    /// Returns an error if the query does not return exactly one row.
    /// 
    /// A statement may contain parameters, specified by $n, where n is the index of the parameter of the list provided, 1-indexed.
    fn query_one(query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, String> {
        database::client()?
            .query_one(query, params)
            .map_err(|error| error.to_string())
    }

    /// Executes a statement, returning the number of rows modified.
    /// 
    /// A statement may contain parameters, specified by $n, where n is the index of the parameter of the list provided, 1-indexed.
    /// 
    /// If the statement does not modify any rows (e.g. SELECT), 0 is returned.
    fn execute(query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, String> {
        database::client()?
            .execute(query, params)
            .map_err(|error| error.to_string())
        }

    /// Executes a statement, returning the number of rows modified.
    /// 
    /// A statement may contain parameters, specified by $n, where n is the index of the parameter of the list provided, 1-indexed.
    /// 
    /// If the statement does not modify any rows (e.g. SELECT), 0 is returned.
    fn delete_by_id(id: i32) -> Result<u64, String> {
        database::client()?
            .execute(
                &format!("DELETE FROM \"{}\" WHERE id = $1", Self::TABLE_NAME),
                &[&id]
            )
            .map_err(|error| error.to_string())
    }

}
