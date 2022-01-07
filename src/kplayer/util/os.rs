#[allow(dead_code)]
extern "C" {
    fn FileExist(p: i32) -> i32;
    fn PrintLog(level: i32, p: i32) -> i32;
    fn NowTimestamp() -> i64;
}

#[allow(dead_code)]
pub fn file_exist(file_path: String) -> bool {
    let str_i = super::string::DynamicString::from(file_path.as_bytes()).get_index();
    unsafe {
        if FileExist(str_i) == 0 {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
pub enum PrintLogLevel {
    INFO,
    ERROR,
    DEBUG,
}

#[allow(dead_code)]
pub fn now_timestamp() -> i64 {
    unsafe { NowTimestamp() }
}

#[allow(dead_code)]
pub fn print_log(level: PrintLogLevel, str: String) {
    let str_i = super::string::DynamicString::from(str.as_bytes()).get_index();
    unsafe {
        PrintLog(level as i32, str_i);
    }
}
