#[allow(dead_code)]
extern "C" {
    fn GetPluginVersion() -> i32;
    fn UpdateArg(key: i32, value: i32) -> i32;
}

#[allow(dead_code)]
pub fn get_plugin_version() -> String {
    unsafe {
        let version = super::string::DynamicString::receive(GetPluginVersion()).unwrap();
        version
    }
}

#[allow(dead_code)]
pub fn update_args(key: String, value: String) -> Result<bool, String> {
    unsafe {
        let key_index = super::string::DynamicString::from(key.as_bytes()).get_index();
        let value_index = super::string::DynamicString::from(value.as_bytes()).get_index();
        let result = UpdateArg(key_index, value_index);
        if result != 0 {
            let err = super::string::DynamicString::receive(result).unwrap();
            return Err(err);
        }
        Ok(true)
    }
}
