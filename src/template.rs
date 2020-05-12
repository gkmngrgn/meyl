use std::fs;
use std::path::PathBuf;
use tera::{Context, Tera};

use crate::{constants, find_all_templates};

pub fn get_context_data(_template_dir: &PathBuf) -> Context {
    let mut context = Context::new();
    context.insert(constants::VAR_SUBJECT, "Hello, world!");
    context
}

pub fn generate_all_templates(src_dir: PathBuf, dst_dir: PathBuf) -> Result<(), String> {
    let templates_dir = src_dir.join("**").join("*.html");
    match Tera::new(templates_dir.to_str().unwrap()) {
        Ok(tera) => {
            let templates = find_all_templates(src_dir);
            if let Err(e) = templates {
                return Err(format!("Template directory error: {}", e));
            }

            for template in templates.unwrap() {
                let template_dir = template.iter().last().unwrap();
                let output_dir = dst_dir.join(template_dir);
                match fs::create_dir_all(output_dir) {
                    Ok(_) => {
                        let context_data = get_context_data(&template);
                        let template_name = format!(
                            "{}/{}",
                            template_dir.to_str().unwrap(),
                            constants::FILE_BODY
                        );
                        let output = tera.render(&template_name, &context_data);
                        println!("DEBUG: {:?}", output);
                    }
                    Err(e) => {
                        return Err(format!("Error: directory couldn't be created. {}", e));
                    }
                }
            }

            Ok(())
        }
        Err(e) => Err(format!("Template parsing error: {}", e)),
    }
}
