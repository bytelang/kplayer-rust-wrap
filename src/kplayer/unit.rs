use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::mem::forget;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use crate::common::error::*;
use crate::common::string::{pull_string, push_string, StringPoint};

const DRIVER_VERSION: &str = "2.0.0";
static mut INSTANCE_PTR: *mut KPPluginUnit = 0x0 as *mut KPPluginUnit;

#[derive(Copy, Clone)]
pub enum KPPluginMediaType {
    Video,
    Audio,
}

#[derive(Copy, Clone)]
pub enum KPPluginFilterType {
    None,
    Main,
    Side,
}

pub trait KPPluginUnitBasic {
    fn get_name(&self) -> String;
    fn get_filter_name(&self) -> String;
    fn get_filter_type(&self) -> KPPluginFilterType;
    fn default_arguments(&self) -> BTreeMap<String, String>;
    fn allow_arguments(&self) -> Vec<String>;
    fn update_arguments(&mut self, key: String, value: String) -> Result<(), String>;

    // This function stage has populated the user-specified parameters and, if necessary, allows for custom parameters to be used for conversion and assignment operations.
    // For example, performing secondary naming operations on filters.
    //
    // Typically, the environment required by this stage plugin has already been set up.
    // Here, you can validate whether the content of the parameters is correct.
    //
    // Example:
    // allow users to set the key for font color as "custom_color".
    // Here, you can check if "custom_color" is set and update the new key that needs to be updated through a re-set operation.
    fn hook_open(&self, arguments: HashMap<String, String>) -> Result<BTreeMap<String, String>, String>;
}

pub struct KPPluginUnit {
    pub app: String,
    pub author: String,
    pub media_type: KPPluginMediaType,
    pub plugins: Vec<Box<dyn KPPluginUnitBasic>>,
}

impl KPPluginUnit {
    pub fn new(app: String, author: String, media_type: KPPluginMediaType) -> KPPluginUnit {
        KPPluginUnit {
            app,
            author,
            media_type,
            plugins: Vec::new(),
        }
    }
    pub fn init<T: ToString, A: ToString>(app: T, author: A, media_type: KPPluginMediaType) {
        unsafe {
            if INSTANCE_PTR == 0x0 as *mut KPPluginUnit {
                let unit = Box::new(KPPluginUnit::new(app.to_string(), author.to_string(), media_type));
                let ptr: &'static mut KPPluginUnit = Box::leak(unit);
                INSTANCE_PTR = ptr as *mut KPPluginUnit;
            }
        }
    }
    pub fn push(basic: Box<dyn KPPluginUnitBasic>) {
        let instance = unsafe { &mut *INSTANCE_PTR };
        instance.plugins.push(basic);
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

// =========================================== plugin basic ======================================= //
#[no_mangle]
pub extern "C" fn get_app(point: StringPoint) -> i32 {
    let instance = unsafe { &mut *INSTANCE_PTR };
    push_string(point, instance.app.to_string());

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

// =========================================== plugin items ======================================= //
#[no_mangle]
pub extern "C" fn get_instance_name(index: i64, point: StringPoint) -> i32 {
    let instance = unsafe { &mut *INSTANCE_PTR };

    match instance.plugins.get(index as usize) {
        None => {
            return RESULT_INSTANCE_INDEX_NOT_FOUND;
        }
        Some(filter) => {
            let name = filter.get_name().clone();
            push_string(point, name);
        }
    };

    RESULT_OK
}

#[no_mangle]
pub extern "C" fn get_instance_filter_name(index: i64, point: StringPoint) -> i32 {
    let instance = unsafe { &mut *INSTANCE_PTR };

    match instance.plugins.get(index as usize) {
        None => {
            return RESULT_INSTANCE_INDEX_NOT_FOUND;
        }
        Some(filter) => {
            let filter_name = filter.get_filter_name().clone();
            push_string(point, filter_name);
        }
    };

    RESULT_OK
}

#[no_mangle]
pub extern "C" fn get_instance_filter_type(index: i64) -> KPPluginFilterType {
    let instance = unsafe { &mut *INSTANCE_PTR };

    match instance.plugins.get(index as usize) {
        None => {
            KPPluginFilterType::None
        }
        Some(filter) => {
            filter.get_filter_type().clone()
        }
    }
}

#[no_mangle]
pub extern "C" fn get_instance_default_arguments_key(index: i64, key_index: i64, key_point: StringPoint, value_point: StringPoint) -> i32 {
    let instance = unsafe { &mut *INSTANCE_PTR };
    if let None = instance.plugins.get(index as usize) {
        return RESULT_INSTANCE_INDEX_NOT_FOUND;
    }
    let filter = instance.plugins.get(index as usize).unwrap();

    let default_arguments = filter.default_arguments();
    match default_arguments.iter().nth(key_index as usize) {
        None => {
            return RESULT_INSTANCE_COLLECTION_OUT_OF_INDEX;
        }
        Some((k, v)) => {
            push_string(key_point, k.clone());
            push_string(value_point, v.clone());
        }
    }

    RESULT_OK
}

#[no_mangle]
pub extern "C" fn get_instance_allow_arguments(index: i64, key_index: i64, value_point: StringPoint) -> i32 {
    let instance = unsafe { &mut *INSTANCE_PTR };
    if let None = instance.plugins.get(index as usize) {
        return RESULT_INSTANCE_INDEX_NOT_FOUND;
    }
    let filter = instance.plugins.get(index as usize).unwrap();

    let allow_arguments = filter.allow_arguments();
    match allow_arguments.iter().nth(key_index as usize) {
        None => {
            return RESULT_INSTANCE_COLLECTION_OUT_OF_INDEX;
        }
        Some(value) => {
            push_string(value_point, value.clone());
        }
    }

    RESULT_OK
}

#[no_mangle]
pub extern "C" fn update_arguments(key_point: StringPoint, value_point: StringPoint) -> i32 {
    let key = pull_string(key_point);
    let value = pull_string(value_point);
    let instance = unsafe { &mut *INSTANCE_PTR };
    for plugin in instance.plugins.iter_mut() {
        match plugin.update_arguments(key.clone(), value.clone()) {
            Ok(_) => {}
            Err(_) => {
                return RESULT_INSTANCE_UPDATE_ARGUMENT_FAILED;
            }
        }
    }
    RESULT_OK
}



