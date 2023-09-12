use crate::common::error::*;
use crate::common::string::{pull_string, push_string, StringPoint};
use crate::error;
use serde_derive::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

const DRIVER_VERSION: &str = "2.0.0";
pub static mut APP: Option<Mutex<String>> = None;
pub static mut INSTANCE_PTR: *mut KPPluginUnit = 0x0 as *mut KPPluginUnit;

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
    fn update_arguments(
        &mut self,
        arguments: HashMap<String, String>,
    ) -> Result<HashMap<String, String>, String>;
    fn notify_subscribe(&self, action: String, message: String) {}
    fn created(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn mounted(&mut self) -> Result<(), String> {
        Ok(())
    }
}

pub struct KPPluginUnit {
    pub(self) is_created: bool,
    pub(self) is_mounted: bool,
    pub app: String,
    pub author: String,
    pub media_type: KPPluginMediaType,
    pub plugins: Vec<Box<dyn KPPluginUnitBasic>>,
}

impl KPPluginUnit {
    pub fn new(app: String, author: String, media_type: KPPluginMediaType) -> KPPluginUnit {
        KPPluginUnit {
            is_created: false,
            is_mounted: false,
            app,
            author,
            media_type,
            plugins: Vec::new(),
        }
    }

    pub fn init<T: ToString, A: ToString>(app: T, author: A, media_type: KPPluginMediaType) {
        unsafe {
            if APP.is_none() {
                APP = Some(Mutex::new(app.to_string()));
            }

            if INSTANCE_PTR == 0x0 as *mut KPPluginUnit {
                let unit = Box::new(KPPluginUnit::new(
                    app.to_string(),
                    author.to_string(),
                    media_type,
                ));
                let ptr: &'static mut KPPluginUnit = Box::leak(unit);
                INSTANCE_PTR = ptr as *mut KPPluginUnit;
            }
        }
    }

    pub fn push(basic: Box<dyn KPPluginUnitBasic>) {
        let instance = unsafe { &mut *INSTANCE_PTR };
        instance.plugins.push(basic);
    }

    pub fn is_created(&self) -> bool {
        self.is_created.clone()
    }

    pub fn is_mounted(&self) -> bool {
        self.is_mounted.clone()
    }
}

// =========================================== common ======================================= //
#[no_mangle]
pub extern "C" fn version() -> i64 {
    let parts: Vec<u32> = DRIVER_VERSION
        .split('.')
        .map(|part| part.parse::<u32>().unwrap())
        .collect();
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
        None => KPPluginFilterType::None,
        Some(filter) => filter.get_filter_type().clone(),
    }
}

#[no_mangle]
pub extern "C" fn get_instance_default_arguments_key(
    index: i64,
    key_index: i64,
    key_point: StringPoint,
    value_point: StringPoint,
) -> i32 {
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
pub extern "C" fn get_instance_allow_arguments(
    index: i64,
    key_index: i64,
    value_point: StringPoint,
) -> i32 {
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
pub extern "C" fn update_arguments(
    name: StringPoint,
    update_argument_point: StringPoint,
    data_point: StringPoint,
) -> i32 {
    let update_argument = pull_string(update_argument_point);
    let name = pull_string(name);

    let arguments = match serde_json::from_str::<HashMap<String, String>>(update_argument.as_str())
    {
        Ok(data) => data,
        Err(err) => {
            push_string(data_point, format!("parse argument failed, error: {}", err));
            return RESULT_INSTANCE_UPDATE_ARGUMENT_FAILED;
        }
    };

    // transform argument
    let instance = unsafe { &mut *INSTANCE_PTR };
    for plugin in instance.plugins.iter_mut() {
        if plugin.get_name() == name {
            return match plugin.update_arguments(arguments.clone()) {
                Ok(result) => match serde_json::to_string(&result) {
                    Ok(transfer_data) => {
                        push_string(data_point, transfer_data);
                        0
                    }
                    Err(err) => {
                        push_string(
                            data_point,
                            format!(
                                "invalid argument json format. transfer_data: {:?}, error: {}",
                                result, err
                            ),
                        );
                        RESULT_INSTANCE_UPDATE_ARGUMENT_FAILED
                    }
                },
                Err(err) => {
                    push_string(
                        data_point,
                        format!(
                            "update argument failed. arguments: {:?}, error: {}",
                            arguments, err
                        ),
                    );
                    RESULT_INSTANCE_UPDATE_ARGUMENT_FAILED
                }
            };
        }
    }
    RESULT_OK
}

#[derive(Deserialize)]
struct SubscribeMessage {
    action: String,
    message: String,
}

#[no_mangle]
pub extern "C" fn notify_subscribe(message_point: StringPoint) -> i32 {
    let message = pull_string(message_point);
    let subscribe_message = match serde_json::from_str::<SubscribeMessage>(message.as_str()) {
        Ok(msg) => msg,
        Err(err) => {
            error!("invalid json format message. error: {}", err);
            return -1;
        }
    };

    let instance = unsafe { &mut *INSTANCE_PTR };
    for plugin in instance.plugins.iter_mut() {
        plugin.notify_subscribe(
            subscribe_message.action.clone(),
            subscribe_message.message.clone(),
        );
    }
    RESULT_OK
}

#[no_mangle]
pub extern "C" fn notify_created() -> i32 {
    let instance = unsafe { &mut *INSTANCE_PTR };
    for plugin in instance.plugins.iter_mut() {
        if let Err(err) = plugin.created() {
            error!("notify created failed. error: {}", err);
            return RESULT_INSTANCE_NOTIFY_CREATED;
        }
    }
    instance.is_created = true;
    RESULT_OK
}

#[no_mangle]
pub extern "C" fn notify_mounted() -> i32 {
    let instance = unsafe { &mut *INSTANCE_PTR };
    for plugin in instance.plugins.iter_mut() {
        if let Err(err) = plugin.mounted() {
            error!("notify created failed. error: {}", err);
            return RESULT_INSTANCE_NOTIFY_CREATED;
        }
    }
    instance.is_mounted = true;
    RESULT_OK
}
