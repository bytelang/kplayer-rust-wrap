use std::collections::BTreeMap;

pub fn args_vec_to_map(
    custom_args: Vec<String>,
) -> std::result::Result<std::collections::HashMap<String, String>, &'static str> {
    let mut args = std::collections::HashMap::new();
    for str in custom_args {
        let sp: Vec<&str> = str.split('=').collect();
        if sp.len() < 2 {
            return Err("args format error");
        }
        args.insert(String::from(sp[0]), String::from(sp[1]));
    }

    Ok(args)
}

pub fn args_map_to_vec(args: std::collections::HashMap<String, String>) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    let mut sort_map = BTreeMap::new();

    for (key, value) in args {
        sort_map.insert(key, value);
    }

    for(key, value) in sort_map{
        vec.push(format!("{}={}", key,value));
    }

    vec
}