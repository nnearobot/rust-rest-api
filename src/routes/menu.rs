use crate::http::{router::Router, *};

pub fn create() -> Router<'static> {
    Router::new("/menu")
        .get("/", get_menu)
}

fn get_menu(request: &str, params: &Vec<&str>) -> (String, String) {
    (OK_RESPONSE.to_string(), format!("Called: get_menu no params"))
}
