use crate::common::string::{BridgeString, pull_string, StringPoint};

#[link(wasm_import_module = "2.0.0")]
extern {
    fn register_permission(point: StringPoint, data_point: StringPoint) -> i32;

    // ext_file
    fn ext_open_file(point: StringPoint, data_point: StringPoint) -> i32;

    // ext_db
    fn ext_db_query(point: StringPoint, data_point: StringPoint) -> i32;
    fn ext_db_execute(point: StringPoint, data_point: StringPoint) -> i32;

    // ext_http
    fn ext_http_request(url_point: StringPoint, method: StringPoint, query_string: StringPoint, body_string: StringPoint, headers: StringPoint, data_point: StringPoint) -> i32;

    // ext_prompt
    fn ext_send_prompt(name_point: StringPoint, prompt_point: StringPoint, data_point: StringPoint) -> i32;
}

pub struct Permission {}

impl Permission {
    pub fn register_permission_subscribe_message<T: ToString>(action: T) -> Result<(), String> {
        unsafe {
            let perm_str = BridgeString::new(format!(r#"{{"SubscribeMessage":{{"action":["{}"]}}}}"#, action.to_string()));
            let perm_point = perm_str.get();

            let data_str = BridgeString::create();
            let data_point = data_str.get();

            // call
            let result = register_permission(perm_point, data_point);
            if result < 0 {
                return Err(data_str.to_string());
            }
        }
        Ok(())
    }
}
