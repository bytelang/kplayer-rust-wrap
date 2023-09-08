use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::common::string::{CallBackPoint, StringPoint};
use crate::kplayer::unit::INSTANCE_PTR;

pub type ClosureFunction = Box<dyn Fn(Result<String, String>)>;

pub fn register_callback(callback: ClosureFunction) -> CallBackPoint {
    let instance = unsafe { &mut *INSTANCE_PTR };
    instance.register_callback(callback)
}

pub fn launch_callback(point: CallBackPoint, params: Result<String, String>) {
    let instance = unsafe { &mut *INSTANCE_PTR };
    instance.launch_callback(point, params);
}