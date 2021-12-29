pub mod plugin;
pub mod proto;
pub mod util;

#[allow(dead_code)]
extern "C" {
    fn GetValidateArgIterator(main_point: i64) -> i32;
    fn UpdateArg(mp: i64, key: i32, value: i32) -> i32;
    fn NewTimerTask(mp: i64, tid: i32, milliseconds: i32) -> i32;
    fn RegisterMessageAction(mp: i64, action: i32) -> i32;
    fn GetHistoryEventMessage(action: i32) -> i32;
}

static mut ARGS_INDEX: usize = 0;
static mut INSTANCES: Vec<Box<dyn plugin::BasePlugin>> = Vec::new();
static mut MAIN_POINT: i64 = 0;

pub fn export_plugin(p: Box<dyn plugin::BasePlugin>) {
    unsafe {
        INSTANCES.push(p);
    }
}

#[no_mangle]
pub extern "C" fn GetFilterName() -> i32 {
    unsafe {
        util::string::DynamicString::from(INSTANCES[0].get_filter_name().as_bytes()).get_index()
    }
}

#[no_mangle]
pub extern "C" fn GetAuthor() -> i32 {
    unsafe { util::string::DynamicString::from(INSTANCES[0].get_author().as_bytes()).get_index() }
}

#[no_mangle]
pub extern "C" fn GetMediaType() -> i32 {
    unsafe {
        let media_type: plugin::MediaType = INSTANCES[0].get_media_type();
        media_type as i32
    }
}

#[no_mangle]
pub extern "C" fn GetArgIterator() -> i32 {
    unsafe {
        let args = INSTANCES[0].get_args();

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
            let re_index = GetValidateArgIterator(MAIN_POINT);
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

        match INSTANCES[0].validate_user_args(&args) {
            Ok(_s) => 0,
            Err(_err) => util::string::DynamicString::from(_err.as_bytes()).get_index(),
        }
    }
}

#[no_mangle]
pub extern "C" fn SetMainPoint(p: i64) -> i32 {
    unsafe {
        MAIN_POINT = p;
    }
    0
}
