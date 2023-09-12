use crate::kplayer::unit::{KPPluginUnit, KPPluginUnitBasic};
use std::collections::{BTreeMap, HashMap};

pub mod common;
pub mod kplayer;

#[macro_export]
macro_rules! export {
    ($app:expr, $author:expr, $media:expr $(, $push:expr)*) => {
        #[no_mangle]
        pub extern "C" fn init() {
            KPPluginUnit::init($app, $author, $media);
            $(KPPluginUnit::push(Box::new($push));)*
        }
    };
}
