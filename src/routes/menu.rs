use crate::http::{router::Router, *};
use crate::database:: models::menu::Menu;

pub fn create() -> Router<'static> {
    Router::new("/menu")
        .get("/", get_menu)
}

fn get_menu(request: &str, params: &Vec<&str>) -> (String, String) {
    match Menu::get_all_items() {
        Ok(items) => (OK_RESPONSE.to_string(), serde_json::to_string(&items).unwrap()),
        Err(error) => (INTERNAL_SERVER_ERROR.to_string(), error.to_string()),
    }
}
