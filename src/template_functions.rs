use std::collections::HashMap;
use tera::{from_value, to_value, Result, Tera, Value};

fn get_static_url(args: &HashMap<String, Value>) -> Result<Value> {
    match args.get("name") {
        Some(val) => match from_value::<String>(val.clone()) {
            Ok(v) => Ok(to_value(&v).unwrap()),
            Err(_) => Err("oops".into()),
        },
        None => Err("oops".into()),
    }
}

pub(crate) fn register_functions(template: &mut Tera) {
    template.register_function("static", get_static_url)
}
