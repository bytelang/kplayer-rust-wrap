use std::collections::{BTreeMap, HashMap};
use std::ops::Index;
use crate::kplayer::unit::{KPPluginFilterType, KPPluginMediaType, KPPluginUnit, KPPluginUnitBasic};

pub mod kplayer;
pub mod common;

#[derive(Default)]
struct Example {}

impl KPPluginUnitBasic for Example {
    fn get_filter_name(&self) -> String {
        "drawtext".to_string()
    }

    fn get_filter_type(&self) -> KPPluginFilterType {
        KPPluginFilterType::Main
    }

    fn default_arguments(&self) -> BTreeMap<String, String> {
        let mut hash = HashMap::new();
        hash.insert("text", "hello kplayer");
        hash.insert("fontsize", "17");
        hash.insert("fontcolor", "white");
        hash.insert("x", "200");

        hash.iter().map(|(k, v)| {
            (k.to_string(), v.to_string())
        }).collect()
    }

    fn allow_arguments(&self) -> Vec<String> {
        let vec = vec!["text", "fontsizie", "fontcolor"];
        vec.iter().map(|item| {
            item.to_string()
        }).collect()
    }

    fn load(&self, arguments: HashMap<String, String>) -> Result<(), String> {
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn init() {
    KPPluginUnit::init("text", "example", KPPluginMediaType::Video);
    KPPluginUnit::push(Box::new(Example::default()));
}