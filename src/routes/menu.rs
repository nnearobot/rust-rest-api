use crate::http::router::Router;

pub fn create() -> Router {
    Router::new("/menu")
        .get("/", get_menu)
}

fn get_menu(request: &str) -> (String, String) {
    println!("Called: get_menu");
    ("".to_string(), "".to_string())
}
