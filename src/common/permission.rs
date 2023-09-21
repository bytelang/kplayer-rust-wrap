use crate::common::string::{BridgeString, StringPoint};
use crate::kplayer::unit::INSTANCE_PTR;

#[link(wasm_import_module = "2.0.0")]
extern "C" {
    fn register_permission(point: StringPoint, data_point: StringPoint) -> i32;

    // ext_file
    fn ext_file_open(point: StringPoint, data_point: StringPoint) -> i32;
    fn ext_file_existed(point: StringPoint, data_point: StringPoint) -> i32;

    // ext_db
    fn ext_db_query(point: StringPoint, data_point: StringPoint) -> i32;
    fn ext_db_execute(point: StringPoint, data_point: StringPoint) -> i32;

    // ext_http
    fn ext_http_request(
        url_point: StringPoint,
        method: StringPoint,
        query_string: StringPoint,
        body_string: StringPoint,
        headers: StringPoint,
        data_point: StringPoint,
    ) -> i32;

    // ext_prompt
    fn ext_send_prompt(prompt_point: StringPoint, data_point: StringPoint) -> i32;
}

pub struct Permission {}

pub fn has_created_must() -> Result<(), String> {
    let instance = unsafe { &mut *INSTANCE_PTR };
    if !instance.is_created() {
        return Err(format!(
            "the plugin instance object has not been created or initialized yet."
        ));
    }

    Ok(())
}

impl Permission {
    pub fn register_permission_subscribe_message<T: ToString>(action: T) -> Result<(), String> {
        unsafe {
            let perm_str = BridgeString::new(format!(
                r#"{{"SubscribeMessage":{{"action":["{}"]}}}}"#,
                action.to_string()
            ));
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

    pub fn register_permission_prompt<T: ToString>(prompt: T) -> Result<String, String> {
        unsafe {
            let perm_str = BridgeString::new(format!(
                r#"{{"PublishPrompt":{{"prompt":["{}"]}}}}"#,
                prompt.to_string()
            ));
            let perm_point = perm_str.get();

            let data_str = BridgeString::create();
            let data_point = data_str.get();

            // call
            let result = register_permission(perm_point, data_point);
            if result < 0 {
                return Err(data_str.to_string());
            }

            Ok(data_str.to_string())
        }
    }

    pub fn register_file(path: Vec<String>) -> Result<String, String> {
        unsafe {
            let perm_str = BridgeString::new(format!(
                r#"{{"File":{{"path":["{}"]}}}}"#, ""
            ));
            let perm_point = perm_str.get();

            let data_str = BridgeString::create();
            let data_point = data_str.get();

            // call
            let result = register_permission(perm_point, data_point);
            if result < 0 {
                return Err(data_str.to_string());
            }

            Ok(data_str.to_string())
        }
    }
}

impl Permission {
    pub fn send_permission_prompt(prompt: String) -> Result<String, String> {
        has_created_must()?;

        // send prompt
        unsafe {
            let prompt_str = BridgeString::new(prompt);
            let prompt_point = prompt_str.get();

            let data_str = BridgeString::create();
            let data_point = data_str.get();

            // call
            let result = ext_send_prompt(prompt_point, data_point);
            if result < 0 {
                return Err(data_str.to_string());
            }

            Ok(data_str.to_string())
        }
    }
}

impl Permission {
    pub fn file_existed(path: &String) -> Result<bool, String> {
        unsafe {
            let path_str = BridgeString::new(path);
            let path_point = path_str.get();

            let data_str = BridgeString::create();
            let data_point = data_str.get();

            let result = ext_file_existed(path_point, data_point);
            if result < 0 {
                return Err(data_str.to_string());
            }

            Ok(true)
        }
    }
}
