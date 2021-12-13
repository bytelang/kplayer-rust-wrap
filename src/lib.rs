pub mod kplayer;

#[macro_export]
macro_rules! export {
    ($class_name: ident) => {
        #[no_mangle]
        pub extern "C" fn Initialization() -> i32 {
            kplayer::export_plugin(Box::new($class_name {}));
            0
        }
    };
}
