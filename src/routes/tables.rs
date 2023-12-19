use crate::http::router::Router;

pub fn create() -> Router {
    Router::new("/tables")
        .get("/", get_tables)
}

fn get_tables(request: &str) -> (String, String) {
    println!("Called: get_tables");
    ("".to_string(), "".to_string())
}
