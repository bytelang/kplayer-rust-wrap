



#[link(wasm_import_module = "2.0.0")]
extern {
    fn string_push(point: u64, chr: u32) -> u64;
    fn string_get(point: u64, index: u64) -> i32;
}

pub type StringPoint = u64;

pub fn pull_string(point: StringPoint) -> String {
    let mut str = String::new();
    let mut index = 0;
    loop {
        unsafe {
            let chr = string_get(point, index);
            if chr < 0 {
                break;
            }

            str.push(std::char::from_u32(chr as u32).unwrap());
            index += 1;
        }
    }

    str
}

pub fn push_string<T: ToString>(point: StringPoint, str: T) {
    let get_str = str.to_string();
    let get_vec = get_str.as_bytes();
    for s in get_vec {
        unsafe {
            string_push(point, s.clone() as u32);
        }
    }
}