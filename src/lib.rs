pub mod kplayer;

#[macro_export]
macro_rules! export {
    ($($class_name: ident),*) => {
        #[no_mangle]
        pub extern "C" fn Initialization() -> i32 {
            // export plugin instance
            $(
            kplayer::export_plugin(Box::new($class_name::new()));
            )*

            // register timer
            kplayer::register_task();

            // register subscribe
            kplayer::register_message();

            0
        }
    };
}

#[macro_export]
macro_rules! version {
    () => {
        #[no_mangle]
        pub extern "C" fn GetVersion() -> i32 {
            10500
        }
    };
}
