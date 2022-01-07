pub mod plugin;
pub mod proto;
pub mod util;

#[allow(dead_code)]
extern "C" {
    fn GetValidateArgIterator() -> i32;
    fn NewTimerTask(tid: i32, milliseconds: i32) -> i32;
    fn RegisterMessageAction(action: i32) -> i32;
    fn GetHistoryEventMessage(action: i32) -> i32;
}

static mut ARGS_INDEX: usize = 0;
static mut INSTANCES: Vec<Box<dyn plugin::BasePlugin>> = Vec::new();
static mut INSTANCES_INDEX: usize = 0;

pub fn export_plugin(p: Box<dyn plugin::BasePlugin>) {
    unsafe {
        INSTANCES.push(p);
    }
}

pub fn register_task() {
    unsafe {
        let task = INSTANCES[INSTANCES_INDEX].register_task();
        for item in task {
            NewTimerTask(item.get_tid(), item.get_milliseconds());
        }
    }
}

pub fn register_message() {
    unsafe {
        let task = INSTANCES[INSTANCES_INDEX].register_message_keys();
        for item in task {
            RegisterMessageAction(item as i32);
        }
    }
}

#[no_mangle]
pub extern "C" fn GetFilterName() -> i32 {
    unsafe {
        util::string::DynamicString::from(INSTANCES[INSTANCES_INDEX].get_filter_name().as_bytes())
            .get_index()
    }
}

#[no_mangle]
pub extern "C" fn GetAuthor() -> i32 {
    unsafe {
        util::string::DynamicString::from(INSTANCES[INSTANCES_INDEX].get_author().as_bytes())
            .get_index()
    }
}

#[no_mangle]
pub extern "C" fn GetMediaType() -> i32 {
    unsafe {
        let media_type: plugin::MediaType = INSTANCES[INSTANCES_INDEX].get_media_type();
        media_type as i32
    }
}

#[no_mangle]
pub extern "C" fn GetArgIterator() -> i32 {
    unsafe {
        let args = INSTANCES[INSTANCES_INDEX].get_args();

        if ARGS_INDEX >= args.len() {
            return 0;
        }

        let iterator = util::string::DynamicString::from(args[ARGS_INDEX].as_bytes()).get_index();
        ARGS_INDEX = ARGS_INDEX + 1;

        iterator
    }
}

#[no_mangle]
pub extern "C" fn ValidateUserArgs() -> i32 {
    let mut args: Vec<String> = Vec::new();

    unsafe {
        // get warp args
        loop {
            let re_index = GetValidateArgIterator();
            if re_index == 0 {
                break;
            }

            match util::string::DynamicString::receive(re_index) {
                Ok(_ok) => args.push(_ok),
                Err(_err) => {
                    return util::string::DynamicString::from(
                        String::from("plugin receive args error").as_bytes(),
                    )
                    .get_index()
                }
            }
        }

        match INSTANCES[INSTANCES_INDEX].validate_user_args(&args) {
            Ok(_s) => 0,
            Err(_err) => util::string::DynamicString::from(_err.as_bytes()).get_index(),
        }
    }
}

#[no_mangle]
pub extern "C" fn NotifyTask(_tid: i32) -> i32 {
    unsafe {
        INSTANCES[INSTANCES_INDEX].execute_task(_tid as u32);
    }
    0
}

#[no_mangle]
pub extern "C" fn NotifyMessage(action: i32, message: i32) -> i32 {
    unsafe {
        let body = util::string::DynamicString::receive(message).unwrap();
        INSTANCES[INSTANCES_INDEX].execute_message(action, body);
    }
    0
}

#[no_mangle]
pub extern "C" fn GetInstanceCount() -> i64 {
    unsafe { INSTANCES.len() as i64 }
}

#[no_mangle]
pub extern "C" fn SetInstanceIndex(index: i64) -> i64 {
    unsafe {
        let count = INSTANCES.len() as i64;
        if index > count - 1 {
            return -1;
        }

        INSTANCES_INDEX = index as usize;
        index
    }
}
