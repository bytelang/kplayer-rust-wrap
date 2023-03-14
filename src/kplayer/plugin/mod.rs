#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum MediaType {
    MediaTypeNone,
    MediaTypeVideo,
    MediaTypeAudio,
}

pub enum InstanceType {
    InstanceTypeSide,
    InstanceTypeLink,
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Timer {
    tid: i32,
    milliseconds: i32,
}
impl Timer {
    pub fn get_tid(&self) -> i32 {
        self.tid
    }
    pub fn get_milliseconds(&self) -> i32 {
        self.milliseconds
    }
    pub fn new(_tid: i32, _milliseconds: i32) -> Self {
        Timer {
            tid: _tid,
            milliseconds: _milliseconds,
        }
    }
}

pub trait BasePlugin {
    // get plugin name
    fn get_name(&self) -> String;

    // get plugin author
    fn get_author(&self) -> String;

    // get plugin args
    fn get_args(&mut self, args: std::collections::HashMap<String, String>) -> Vec<String>;

    // get plugin allow custom args
    fn get_allow_custom_args(&self) -> Vec<&'static str> {
        let vec: Vec<&'static str> = Vec::new();
        vec
    }

    // get plugin filter name
    fn get_filter_name(&self) -> String;

    // get plugin media type
    fn get_media_type(&self) -> MediaType;

    // validate args
    fn validate_user_args(
        &mut self,
        args: std::collections::HashMap<String, String>,
    ) -> Result<bool, &'static str>;

    // print plugin log
    fn print_log(&self, level: super::util::os::PrintLogLevel, log: &str) {
        super::util::os::print_log(level, &format!("[{}] {}", self.get_name(), log).to_string())
    }

    // get timer
    fn register_task(&self) -> Vec<Timer> {
        let empty: Vec<Timer> = Vec::new();
        empty
    }

    // get subscribe key
    fn register_message_keys(&self) -> Vec<super::proto::keys::EventMessageAction> {
        let empty: Vec<super::proto::keys::EventMessageAction> = Vec::new();
        empty
    }

    // execute timer
    #[warn(unused_variables)]
    fn execute_task(&mut self, _tid: u32) {}

    // execute message
    #[allow(unused_variables)]
    fn execute_message(&mut self, action: i32, body: String) {}

    // get instance type
    #[allow(unused_variables)]
    fn get_instance_type(&mut self) -> InstanceType {
        InstanceType::InstanceTypeLink
    }

    // hook created
    #[allow(unused_variables)]
    fn hook_created(&mut self) -> i32 {
        0
    }
}
