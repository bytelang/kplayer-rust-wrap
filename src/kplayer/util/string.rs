extern "C" {
    fn NewString() -> i32;
    fn DeleteString(i: i32) -> i32;
    fn AppendChar(p: i32, i: i32) -> i32;
    fn GetString(p: i32, i: i32) -> i32;
}

pub struct DynamicString {
    index: i32,
}

impl DynamicString {
    pub fn new() -> DynamicString {
        DynamicString { index: 0 }
    }

    pub fn get_index(&self) -> i32 {
        self.index
    }

    pub fn receive(&self, index: i32) -> Result<String, &'static str> {
        let mut str: String = String::new();
        unsafe {
            let c: i32 = 0;
            loop {
                let char_u8 = GetString(index, c) as u8;
                if char_u8 == 0 {
                    break;
                }
                str.push(char_u8 as char);
            }
        }

        Ok(str)
    }

    fn append(&mut self, d: &[u8]) {
        unsafe {
            for i in d {
                self.index = AppendChar(self.index, (*i) as i32)
            }
        }
    }
}

impl Drop for DynamicString {
    fn drop(&mut self) {
        unsafe {
            DeleteString(self.index);
        }
    }
}

impl From<&[u8]> for DynamicString {
    fn from(d: &[u8]) -> DynamicString {
        let mut str = DynamicString {
            index: unsafe { NewString() },
        };
        str.append(d);

        str
    }
}
