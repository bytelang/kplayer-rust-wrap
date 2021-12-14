#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum MediaType {
    MediaTypeNone,
    MediaTypeVideo,
    MediaTypeAudio,
}

pub trait BasePlugin {
    // get plugin name
    fn get_name(&self) -> String;

    // get plugin author
    fn get_author(&self) -> String;

    // get plugin args
    fn get_args(&self) -> Vec<String>;

    // get plugin filter name
    fn get_filter_name(&self) -> String;

    // get plugin media type
    fn get_media_type(&self) -> MediaType;

    // validate args
    fn validate_user_args(&self, args: &Vec<String>) -> Result<bool, &'static str>;

    // print plugin log
    fn print_log(&self, level: super::util::os::PrintLogLevel, log: &str) {
        super::util::os::print_log(level, format!("[{}]", self.get_name()).to_string() + log)
    }
}
