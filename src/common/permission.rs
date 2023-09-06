use crate::common::string::{pull_string, StringPoint};

#[link(wasm_import_module = "2.0.0")]
extern {
    fn register_permission(point: u64) -> u64;

    // ext_file
    fn ext_open_file(point: u64) -> u64;

    // ext_db
    fn ext_db_query(point: u64) -> u64;
    fn ext_db_execute(point: u64) -> u64;

    // ext_http
    fn ext_http_request(point: u64) -> u64;

    // ext_prompt
    fn ext_send_prompt(point: u64) -> u64;
}

pub struct Permission {}

impl Permission {
    pub fn register_permission(perm: String) {
    }
}
