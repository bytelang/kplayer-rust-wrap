pub mod plugin;
pub mod proto;
pub mod util;

super::version!();

#[allow(dead_code)]
extern "C" {
    fn GetValidateArgIterator() -> i32;
    fn ResetValidateArgIterator() -> i32;
    fn NewTimerTask(tid: i32, milliseconds: i32) -> i32;
    fn RegisterMessageAction(action: i32) -> i32;
    fn GetHistoryEventMessage(action: i32) -> i32;
}

static mut ARGS_INDEX: usize = 0;
static mut ALLOW_ARGS_INDEX: usize = 0;
static mut CULL_ARGS_INDEX: usize = 0;
static mut INSTANCES: Vec<Box<dyn plugin::IBasePlugin>> = Vec::new();
static mut INSTANCES_INDEX: usize = 0;

pub fn export_plugin(p: Box<dyn plugin::IBasePlugin>) {
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

pub fn get_history_message(action: proto::keys::EventMessageAction) -> String {
    unsafe {
        let msg_index = GetHistoryEventMessage(action as i32);
        util::string::DynamicString::receive(msg_index).unwrap()
    }
}

#[no_mangle]
pub extern "C" fn GetName() -> i32 {
    unsafe {
        util::string::DynamicString::from(INSTANCES[INSTANCES_INDEX].get_name().as_bytes())
            .get_index()
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
        // get custom args
        loop {
            let re_index = GetValidateArgIterator();
            if re_index == 0 {
                break;
            }

            match util::string::DynamicString::receive(re_index) {
                Ok(_value) => {
                    let sp: Vec<&str> = _value.split('=').collect();
                    if sp.len() < 2 {
                        return -1;
                    }

                    // must fill args
                    if INSTANCES[INSTANCES_INDEX].get_base_plugin().get_fill_args().contains(&sp[0].to_string()) {
                        INSTANCES[INSTANCES_INDEX].get_base_plugin().set_arg(sp[0].to_string(), sp[1].to_string());
                    }
                }
                Err(_err) => {
                    return util::string::DynamicString::from(
                        String::from("plugin receive args error").as_bytes(),
                    ).get_index();
                }
            }
        }

        // get plugin args
        let args = util::argument::args_map_to_vec(INSTANCES[INSTANCES_INDEX].get_base_plugin().get_args());

        if ARGS_INDEX >= args.len() {
            ARGS_INDEX = 0;
            return 0;
        }

        let iterator = util::string::DynamicString::from(args[ARGS_INDEX].as_bytes()).get_index();
        ARGS_INDEX = ARGS_INDEX + 1;

        iterator
    }
}

#[no_mangle]
pub extern "C" fn GetAllowArgIterator() -> i32 {
    unsafe {
        // get plugin allow override args
        let args = INSTANCES[INSTANCES_INDEX].get_base_plugin().get_fill_args();

        if ALLOW_ARGS_INDEX >= args.len() {
            return 0;
        }

        let iterator =
            util::string::DynamicString::from(args[ALLOW_ARGS_INDEX].as_bytes()).get_index();
        ALLOW_ARGS_INDEX = ALLOW_ARGS_INDEX + 1;

        iterator
    }
}

#[no_mangle]
pub extern "C" fn GetCullArgIterator() -> i32 {
    unsafe {
        // get plugin cull override args
        let args = INSTANCES[INSTANCES_INDEX].get_base_plugin().get_guard_args();

        if CULL_ARGS_INDEX >= args.len() {
            return 0;
        }

        let iterator =
            util::string::DynamicString::from(args[CULL_ARGS_INDEX].as_bytes()).get_index();
        CULL_ARGS_INDEX = CULL_ARGS_INDEX + 1;

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
                        .get_index();
                }
            }
        }

        match INSTANCES[INSTANCES_INDEX]
            .validate_user_args(util::argument::args_vec_to_map(args).unwrap())
        {
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

#[no_mangle]
pub extern "C" fn GetInstanceType(index: i64) -> i32 {
    unsafe {
        let count = INSTANCES.len() as i64;
        if index > count - 1 {
            return -1;
        }

        let instance_type = INSTANCES[index as usize].get_instance_type();
        instance_type as i32
    }
}

#[no_mangle]
pub extern "C" fn HookCreated() -> i32 {
    unsafe { INSTANCES[INSTANCES_INDEX].hook_created() }
}
