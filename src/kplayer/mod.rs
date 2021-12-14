pub mod plugin;
pub mod util;

extern "C" {
    fn GetValidateArgIterator() -> i32;
}

static mut ARGS_INDEX: usize = 0;
static mut INSTANCES: Vec<Box<dyn plugin::BasePlugin>> = Vec::new();

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
            let re_index = GetValidateArgIterator();
            if re_index == 0 {
                break;
            }

            match util::string::DynamicString::new().receive(re_index) {
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
