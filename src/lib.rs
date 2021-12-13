pub mod kplayer;

#[macro_export]
macro_rules! export {
    ($fmt: expr) => {
        #[no_mangle]
        pub extern "C" fn Initialization() -> i32 {
            kplayer::export_plugin(Box::new($fmt {}));
            0
        }
    };
}
