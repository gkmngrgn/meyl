use std::fs;
use std::path::PathBuf;
use tera::{Context, Tera};

use crate::{constants, find_all_templates};

struct Email {
    tera: Tera,
    src_dir: PathBuf,
    dst_dir: PathBuf,
    context_data: Context,
    subject: String,
    body: String,
    body_text: String,
}

impl Email {
    fn new(tera: Tera, src_dir: PathBuf, dst_dir: PathBuf) -> Self {
        Self {
            tera,
            src_dir,
            dst_dir,
            subject: "".to_string(),
            body: "".to_string(),
            body_text: "".to_string(),
            context_data: Context::new(),
        }
    }

    fn add_context_data(&mut self, name: &str, value: &str) {
        self.context_data.insert(name, value);
    }

    fn render_template(&mut self, template_name: &str) -> Result<String, String> {
        let template_name = format!(
            "{}/{}",
            self.src_dir.iter().last().unwrap().to_str().unwrap(), // WTF??
            template_name
        );
        match self.tera.render(&template_name, &self.context_data) {
            Ok(html) => Ok(html),
            Err(e) => {
                return Err(format!(
                    "There are undefined variables in the template. {}",
                    e
                ));
            }
        }
    }

    fn render_templates(&mut self) -> Result<(), String> {
        self.subject = self.render_template(constants::FILE_SUBJECT)?;
        self.add_context_data(constants::VAR_SUBJECT, self.subject.clone().as_str());
        self.body = self.render_template(constants::FILE_BODY)?;
        self.body_text = if self.src_dir.join(constants::FILE_BODY_TEXT).exists() {
            self.render_template(constants::FILE_BODY_TEXT)?
        } else {
            self.strip_tags(self.body.clone().as_str())?
        };
        Ok(())
    }

    fn save_rendered_outputs(self) -> std::io::Result<()> {
        fs::write(self.dst_dir.join(constants::FILE_SUBJECT), self.subject)?;
        fs::write(self.dst_dir.join(constants::FILE_BODY), self.body)?;
        fs::write(self.dst_dir.join(constants::FILE_BODY_TEXT), self.body_text)?;
        Ok(())
    }

    fn strip_tags(&mut self, _text: &str) -> Result<String, String> {
        // TODO: we need to prepare a new function to strip html tags from body.
        Ok("".to_string())
    }
}

pub fn generate_all_templates(src_dir: PathBuf, dst_dir: PathBuf) -> Result<(), String> {
    let templates_dir = src_dir.join("**").join("*.html");
    match Tera::new(templates_dir.to_str().unwrap()) {
        Ok(tera) => {
            let templates = find_all_templates(src_dir);
            if let Err(e) = templates {
                return Err(format!("Template directory error: {}", e));
            }

            for template_src in templates.unwrap() {
                let template_dst = dst_dir.join(template_src.iter().last().unwrap());
                if let Err(e) = fs::create_dir_all(&template_dst) {
                    return Err(format!("Error: directory couldn't be created. {}", e));
                }
                let mut email = Email::new(tera.clone(), template_src, template_dst);
                match email.render_templates() {
                    Ok(_) => {
                        if let Err(e) = email.save_rendered_outputs() {
                            return Err(format!("read error: {}", e));
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("Template parsing error: {}", e)),
    }
}
