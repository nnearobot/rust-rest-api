pub mod method;
pub mod router;

pub const OK_RESPONSE: &str = "200 OK\r\nContent-Type: application/json";
pub const NOT_FOUND: &str = "404 NOT FOUND";
pub const BAD_REQUEST: &str = "400 BAD REQUEST";
pub const INTERNAL_SERVER_ERROR: &str = "500 INTERNAL SERVER ERROR";

