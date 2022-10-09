#[allow(dead_code)]
extern "C" {
    fn GetPluginVersion() -> i32;
    fn UpdateArg(key: i32, value: i32) -> i32;
    fn SetEnable(enable: i32) -> i32;
    fn Reload() -> i32;
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

#[allow(dead_code)]
pub fn set_enable(enable: bool) -> Result<bool, String> {
    unsafe {
        let mut enable_prams = 1;
        if !enable {
            enable_prams = 0;
        }
        let result = SetEnable(enable_prams);
        if result != 0 {
            let err = super::string::DynamicString::receive(result).unwrap();
            return Err(err);
        }
        Ok(true)
    }
}

#[allow(dead_code)]
pub fn reload() -> Result<bool, String> {
    unsafe {
        let result = Reload();
        if result != 0 {
            let err = super::string::DynamicString::receive(result).unwrap();
            return Err(err);
        }
        Ok(true)
    }
}
