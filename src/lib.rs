use std::collections::{BTreeMap, HashMap};

use crate::kplayer::unit::{KPPluginFilterType, KPPluginMediaType, KPPluginUnit, KPPluginUnitBasic};

pub mod kplayer;
pub mod common;

#[derive(Default)]
struct Example1 {}

impl KPPluginUnitBasic for Example1 {
    fn get_name(&self) -> String {
        "drawtext".to_string()
    }

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
        let vec = vec!["text", "fontsize", "fontcolor", "x", "y"];
        vec.iter().map(|item| {
            item.to_string()
        }).collect()
    }

    fn update_arguments(&mut self, _key: String, _value: String) -> Result<(), String> {
        Ok(())
    }

    fn hook_open(&self, _arguments: HashMap<String, String>) -> Result<BTreeMap<String, String>, String> {
        Ok(BTreeMap::new())
    }
}

#[derive(Default)]
struct Example2 {}

impl KPPluginUnitBasic for Example2 {
    fn get_name(&self) -> String {
        "drawtext2".to_string()
    }

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
        hash.insert("y", "200");

        hash.iter().map(|(k, v)| {
            (k.to_string(), v.to_string())
        }).collect()
    }

    fn allow_arguments(&self) -> Vec<String> {
        let vec = vec!["text", "fontsize", "fontcolor", "x", "y"];
        vec.iter().map(|item| {
            item.to_string()
        }).collect()
    }

    fn update_arguments(&mut self, _key: String, _value: String) -> Result<(), String> {
        Ok(())
    }

    fn hook_open(&self, _arguments: HashMap<String, String>) -> Result<BTreeMap<String, String>, String> {
        Ok(BTreeMap::new())
    }
}

#[no_mangle]
pub extern "C" fn init() {
    KPPluginUnit::init("text", "example", KPPluginMediaType::Video);
    KPPluginUnit::push(Box::new(Example1::default()));
    KPPluginUnit::push(Box::new(Example2::default()));
}