


pub mod common;
pub mod kplayer;

#[macro_export]
macro_rules! export {
    ($app:expr, $author:expr, $media:expr, $clock:expr $(, $push:expr)*) => {
        #[no_mangle]
        pub extern "C" fn init() {
            KPPluginUnit::init($app, $author, $media, $clock);
            $(KPPluginUnit::push(Box::new($push));)*
        }
    };
}
