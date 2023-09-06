use std::collections::{BTreeMap, HashMap};
use crate::kplayer::unit::{KPPluginUnit, KPPluginUnitBasic};

pub mod kplayer;
pub mod common;

#[macro_export]
macro_rules! export {
    ($text:expr, $example:expr, $media:expr $(, $push:expr)*) => {
        #[no_mangle]
        pub extern "C" fn init() {
            KPPluginUnit::init($text, $example, $media);
            $(KPPluginUnit::push(Box::new($push));)*
        }
    };
}