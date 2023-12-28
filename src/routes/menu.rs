use crate::http::{router::Router, *};
use crate::database:: models::menu::Menu;

/// Creates a router for `/menu` endpoint
/// 
/// GET `/menu` returns a list of menu items.
pub fn create() -> Router<'static> {
    Router::new("/menu")
        .get("/", get_menu)
}

fn get_menu(_: &str, _: &Vec<&str>) -> (String, String) {
    match Menu::get_all_items() {
        Ok(items) => (OK_RESPONSE.to_string(), serde_json::to_string(&items).unwrap()),
        Err(error) => (INTERNAL_SERVER_ERROR.to_string(), error),
    }
}
