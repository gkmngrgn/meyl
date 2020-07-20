use crate::constants::CTX_BASE_URL;
use std::collections::HashMap;
use tera::{from_value, to_value, Result, Tera, Value};

type GlobalFn = Box<dyn Fn(HashMap<String, Value>) -> Result<Value> + Sync + Send>;

fn get_static_url(context_data: HashMap<String, String>) -> GlobalFn {
    Box::new(move |args| -> Result<Value> {
        let base_url = match context_data.get(CTX_BASE_URL) {
            Some(val) => val.as_str().unwrap(),
            None => "http://localhost",
        };
        match args.get("path") {
            Some(val) => Ok(to_value(match from_value::<String>(val.clone()) {
                Ok(v) => format!("{}{}", base_url, v),
                Err(_) => base_url.to_string(),
            })
            .unwrap()),
            None => Err("oops".into()),
        }
    })
}

pub(crate) fn register_functions(template: &mut Tera, _context_data: Value) {
    let context_data2: HashMap<String, String> = HashMap::new();
    template.register_function("static", get_static_url(context_data2));
}
