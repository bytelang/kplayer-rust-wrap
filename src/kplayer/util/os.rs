#[allow(dead_code)]
extern "C" {
    fn FileExist(p: i32) -> i32;
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
