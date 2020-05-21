use html2text::from_read as strip_tags;
use inline_assets::{inline_html_string, Config as InlinerConfig};
use minifier::html::minify as minify_html;
use std::fs;
use std::path::PathBuf;
use tera::{Context, Tera};

use crate::{constants, find_all_templates};

#[derive(Debug)]
pub enum ErrorKind {
    Style,
    MissingContext,
    InvalidDirectory,
    DirectoryAccess,
}

struct Email {
    template: Tera,
    template_name: String,
    src_dir: PathBuf,
    dst_dir: PathBuf,
    context_data: Context,
    subject: String,
    body: String,
    body_text: String,
}

impl Email {
    fn new(src_dir: PathBuf, dst_dir: PathBuf, template_name: String) -> Result<Self, ErrorKind> {
        let template_path = format!(
            "{}/**/*.html",
            src_dir
                .iter()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<&str>>()
                .join("/")
        );
        match Tera::new(&template_path) {
            Ok(template) => {
                let email = Self {
                    template,
                    template_name,
                    src_dir,
                    dst_dir,
                    context_data: Context::new(),
                    subject: "".to_string(),
                    body: "".to_string(),
                    body_text: "".to_string(),
                };
                Ok(email)
            }
            Err(_) => Err(ErrorKind::InvalidDirectory),
        }
    }

    fn add_context_data(&mut self, name: &str, value: &str) {
        self.context_data.insert(name, value);
    }

    fn render(&mut self, file_name: &str, is_html: bool) -> Result<String, ErrorKind> {
        let template_name = format!("{}/{}", self.template_name, file_name);
        match self.template.render(&template_name, &self.context_data) {
            Ok(mut rendered) => {
                rendered = if is_html {
                    self.embed_styles(&rendered)?
                } else {
                    self.strip_tags(&rendered)?
                };
                Ok(rendered)
            }
            Err(_) => Err(ErrorKind::MissingContext),
        }
    }

    fn render_template(&mut self) -> Result<(), ErrorKind> {
        self.subject = self.render(constants::FILE_SUBJECT, false)?;
        self.add_context_data(constants::VAR_SUBJECT, self.subject.clone().as_str());
        self.body = self.render(constants::FILE_BODY, true)?;
        self.body_text = if self
            .src_dir
            .join(&self.template_name)
            .join(constants::FILE_BODY_TEXT)
            .exists()
        {
            self.render(constants::FILE_BODY_TEXT, false)?
        } else {
            self.strip_tags(self.body.clone().as_str())?
        };
        Ok(())
    }

    fn save_rendered_outputs(self) -> std::io::Result<()> {
        let dst_dir = self.dst_dir.join(&self.template_name);
        fs::write(dst_dir.join(constants::FILE_SUBJECT), self.subject)?;
        fs::write(dst_dir.join(constants::FILE_BODY), self.body)?;
        fs::write(dst_dir.join(constants::FILE_BODY_TEXT), self.body_text)?;
        Ok(())
    }

    fn strip_tags(&mut self, text: &str) -> Result<String, ErrorKind> {
        let stripped = strip_tags(text.as_bytes(), constants::TEXT_WIDTH);
        Ok(stripped)
    }

    fn embed_styles(&mut self, text: &str) -> Result<String, ErrorKind> {
        match inline_html_string(text, &self.src_dir, InlinerConfig::default()) {
            Ok(embedded) => Ok(minify_html(&embedded)),
            Err(_) => Err(ErrorKind::Style),
        }
    }
}

pub fn generate_all_templates(src_dir: PathBuf, dst_dir: PathBuf) -> Result<(), ErrorKind> {
    match find_all_templates(src_dir.clone()) {
        Ok(template_names) => {
            for template_name in template_names {
                let template_dst = dst_dir.join(&template_name);
                if let Err(_) = fs::create_dir_all(&template_dst) {
                    return Err(ErrorKind::InvalidDirectory);
                }
                // TODO: do we need to clone these paths?
                let mut email =
                    Email::new(src_dir.clone(), dst_dir.clone(), template_name.clone())?;
                email.render_template()?;
                if let Err(_) = email.save_rendered_outputs() {
                    return Err(ErrorKind::DirectoryAccess);
                }
            }
            Ok(())
        }
        Err(_) => Err(ErrorKind::InvalidDirectory),
    }
}
