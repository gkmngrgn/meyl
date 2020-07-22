use crate::config::Config;
use std::collections::HashMap;
use tera;

type TeraTable = HashMap<String, tera::Value>;

fn get_static_url(args: &TeraTable, base_url: String) -> tera::Result<tera::Value> {
    let url = match args.get("path") {
        Some(p) => [base_url, p.as_str().expect("str expected.").to_string()]
            .iter()
            .map(|i| i.trim_start_matches('/'))
            .collect::<Vec<&str>>()
            .join("/"),
        None => "None".to_string(),
    };
    Ok(tera::Value::from(url))
}

pub fn register_functions(template: &mut tera::Tera, config: Config) {
    template.register_function("static", move |args: &TeraTable| {
        let base_url = config.get_base_url();
        get_static_url(args, base_url)
    });
}
