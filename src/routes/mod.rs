pub mod menu;
pub mod tables;
pub mod orders;

use crate::http::router::Router;

pub fn create(base: &str) -> Router {
    Router::new(base)
        .merge_from(menu::create())
        .merge_from(tables::create())
        .merge_from(orders::create())
}