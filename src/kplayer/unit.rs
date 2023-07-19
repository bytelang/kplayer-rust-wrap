use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::mem::forget;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use crate::common::error::*;
use crate::common::string::{push_string, StringPoint};

const DRIVER_VERSION: &str = "2.0.0";
static mut INSTANCE_PTR: *mut KPPluginUnit = 0x0 as *mut KPPluginUnit;

#[derive(Copy, Clone)]
pub enum KPPluginMediaType {
    Video,
    Audio,
}

pub trait KPPluginUnitBasic {
    fn get_filter(&self) -> String;
    fn default_arguments(&self) -> HashMap<String, String>;
    fn allow_arguments(&self) -> Vec<String>;
    fn load(&self, arguments: HashMap<String, String>) -> Result<(), String>;
}

pub struct KPPluginUnit {
    pub name: String,
    pub author: String,
    pub media_type: KPPluginMediaType,
    pub index: i64,
    pub plugins: Vec<Vec<Box<dyn KPPluginUnitBasic>>>,
}

impl KPPluginUnit {
    pub fn new(name: String, author: String, media_type: KPPluginMediaType) -> KPPluginUnit {
        KPPluginUnit {
            name,
            author,
            media_type,
            index: -1,
            plugins: Vec::new(),
        }
    }
    pub fn init<T: ToString, A: ToString>(name: T, author: A, media_type: KPPluginMediaType) {
        unsafe {
            if INSTANCE_PTR == 0x0 as *mut KPPluginUnit {
                let unit = Box::new(KPPluginUnit::new(name.to_string(), author.to_string(), media_type));
                let ptr: &'static mut KPPluginUnit = Box::leak(unit);
                INSTANCE_PTR = ptr as *mut KPPluginUnit;
            }
        }
    }
    pub fn push(basic: Box<dyn KPPluginUnitBasic>) {
        let instance = unsafe { &mut *INSTANCE_PTR };

        unsafe {
            println!("{} ###{:?}", instance.index, INSTANCE_PTR);
        }
        println!("###{}", instance.index);
        if instance.index == -1 {
            instance.plugins.push(vec![basic]);
            instance.index = 0;
        } else {
            instance.plugins[instance.index as usize].push(basic);
        }
    }

    pub fn step() {
        let instance = unsafe { &mut *INSTANCE_PTR };

        instance.index += 1;
    }
}

// =========================================== common ======================================= //
#[no_mangle]
pub extern "C" fn version() -> i64 {
    let parts: Vec<u32> = DRIVER_VERSION.split('.').map(|part| part.parse::<u32>().unwrap()).collect();
    assert_eq!(parts.len(), 3);

    let version = parts[0] * 1_000_000 + parts[1] * 1_000 + parts[2];
    version as i64
}

// =========================================== instance ======================================= //
#[no_mangle]
pub extern "C" fn get_instance_count() -> i64 {
    let instance = unsafe { &mut *INSTANCE_PTR };

    instance.plugins.len() as i64
}

#[no_mangle]
pub extern "C" fn get_sub_instance_count(root_index: i64) -> i64 {
    let instance = unsafe { &mut *INSTANCE_PTR };

    match instance.plugins.get(root_index as usize) {
        None => -1,
        Some(vec) => {
            vec.len() as i64
        }
    }
}

// =========================================== plugin basic ======================================= //
#[no_mangle]
pub extern "C" fn get_name(point: StringPoint) -> i32 {
    let instance = unsafe { &mut *INSTANCE_PTR };

    // if instance.index < root_index {
    //     return RESULT_ROOT_INDEX_NOT_FOUND;
    // }
    // if instance.plugins[root_index as usize].len() - 1 > sub_index as usize {
    //     return RESULT_SUB_INDEX_NOT_FOUND;
    // }
    //
    // let basic = &instance.plugins[root_index as usize][sub_index as usize];
    push_string(point, instance.name.to_string());

    RESULT_OK
}

#[no_mangle]
pub extern "C" fn get_author(point: StringPoint) -> i32 {
    let instance = unsafe { &mut *INSTANCE_PTR };
    push_string(point, instance.author.to_string());

    RESULT_OK
}

#[no_mangle]
pub extern "C" fn get_media_type() -> u32 {
    let instance = unsafe { &mut *INSTANCE_PTR };
    instance.media_type as u32
}
