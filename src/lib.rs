use std::collections::{BTreeMap, HashMap};
use crate::kplayer::unit::{KPPluginUnit, KPPluginUnitBasic};

pub mod kplayer;
pub mod common;

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