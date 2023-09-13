#[link(wasm_import_module = "2.0.0")]
extern "C" {
    fn string_make() -> StringPoint;
    fn string_free(point: StringPoint);
    fn string_push(point: StringPoint, chr: i64) -> i64;
    fn string_get(point: StringPoint, index: i64) -> i32;
}

pub struct BridgeString {
    point: StringPoint,
}

impl BridgeString {
    pub fn new<T: ToString>(str: T) -> Self {
        unsafe {
            let point = string_make();
            push_string(point, str.to_string());
            BridgeString { point }
        }
    }

    pub fn create() -> Self {
        Self::new(String::new())
    }

    pub fn get(&self) -> StringPoint {
        self.point
    }
}

impl ToString for BridgeString {
    fn to_string(&self) -> String {
        pull_string(self.point)
    }
}

impl Drop for BridgeString {
    fn drop(&mut self) {
        unsafe {
            string_free(self.point);
        }
    }
}

pub type StringPoint = i64;

pub fn pull_string(point: StringPoint) -> String {
    let mut str = Vec::new();
    let mut index = 0;
    loop {
        unsafe {
            let chr = string_get(point, index);
            if chr < 0 {
                break;
            }

            str.push(chr as u8);
            index += 1;
        }
    }

    String::from_utf8(str).unwrap()
}

pub fn push_string<T: ToString>(point: StringPoint, str: T) {
    let get_str = str.to_string();
    let get_vec = get_str.as_bytes();
    for s in get_vec {
        unsafe {
            string_push(point, s.clone() as i64);
        }
    }
}
