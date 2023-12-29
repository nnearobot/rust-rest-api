pub mod menu;
pub mod tables;
pub mod orders;

use crate::http::{router::Router, *};

pub fn create(base: &'static str) -> Router<'static> {
    Router::new(base)
        .get("/", health_check)

        .merge_from(menu::create())
        .merge_from(tables::create())
        .merge_from(orders::create())
}

fn health_check(_: &str, _: &Vec<&str>) -> (String, String) {
    (OK_RESPONSE.to_string(), "Version 1 is running".to_string())
}
