/// Table model
/// 
/// DB table: `table`
/// 
/// - id - i32 - client id
/// - description - String - description of the client: which table it belongs to

pub const TABLE_NAME: &str = "table";

#[derive(Serialize, Deserialize)]
pub struct Table {
    pub id: Option<i32>,
    pub description: String,
}

