#[allow(dead_code)]
extern "C" {
    fn GetPluginVersion() -> i32;
}

#[allow(dead_code)]
pub fn get_plugin_version() -> String {
    super::string::DynamicString::new()
        .receive(GetPluginVersion())
        .nowrap();
}
