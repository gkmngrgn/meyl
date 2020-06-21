use html2text;
use inline_assets::{inline_html_string, Config as InlinerConfig};
use regex::Regex;
use std::env;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use tera::{Context, Tera};

use crate::{config, constants, find_all_templates};
use config::get_context_data;

#[derive(Debug)]
pub enum ErrorKind {
    Style(String),
    MissingContext(String),
    InvalidDirectory(String),
    DirectoryAccess(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ErrorKind::Style(msg) => format!("Style error: {}", msg),
            ErrorKind::MissingContext(msg) => format!("Context data error: {}", msg),
            ErrorKind::InvalidDirectory(msg) => format!("Directory error: {}", msg),
            ErrorKind::DirectoryAccess(msg) => format!("Access problem: {}", msg),
        };
        write!(f, "{}", msg)
    }
}

pub struct HTMLBody {
    rendered_body: String,
    src_dir: PathBuf,
}

impl HTMLBody {
    fn new(src_dir: PathBuf) -> Self {
        Self {
            rendered_body: String::new(),
            src_dir,
        }
    }

    fn get_head(&self, subject: &str) -> String {
        let mut head_items = vec![format!("<title>{}</title>", subject)];
        if self.src_dir.join(constants::FILE_STYLE).exists() {
            head_items.push(format!(
                "<link rel=\"stylesheet\" href=\"{}\" />",
                constants::FILE_STYLE
            ));
        }
        head_items.join("")
    }

    fn render_html(&mut self, subject: &str, body: &str) -> Result<(), ErrorKind> {
        let html_template = r###"
            <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd">
            <html xmlns="http://www.w3.org/1999/xhtml">
                <head>
                    <meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
                    <meta name="viewport" content="width=device-width" />
                    {{ HEAD }}
                </head>
                <body>
                    {{ BODY }}
                </body>
            </html>
        "###;
        let html_rendered = html_template
            .replace("{{ BODY }}", body)
            .replace("{{ HEAD }}", &self.get_head(subject));
        match inline_html_string(&html_rendered, &self.src_dir, InlinerConfig::default()) {
            Ok(mut embedded) => {
                embedded = embedded
                    .trim_matches(|c| c == '\n' || c == ' ')
                    .split("\n")
                    .map(|l| l.trim_start().to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                let re = Regex::new(r"(> {0,}<)").unwrap();
                self.rendered_body = re.replace_all(&embedded, "> <").to_string();
                Ok(())
            }
            Err(_) => {
                let msg = "The style path is not correct.".to_string();
                Err(ErrorKind::Style(msg))
            }
        }
    }
}

pub struct Email {
    template: Tera,
    template_name: String,
    src_dir: PathBuf,
    dst_dir: PathBuf,
    context_data: Context,
    pub subject: String,
    pub body: String,
    pub body_text: String,
}

impl Email {
    pub fn new(
        src_dir: PathBuf,
        dst_dir: PathBuf,
        template_name: String,
    ) -> Result<Self, ErrorKind> {
        let template_path = format!(
            "{}/**/*.html",
            src_dir
                .iter()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<&str>>()
                .join("/")
        );
        match Tera::new(&template_path) {
            Ok(mut template) => {
                // tera settings
                template.autoescape_on(vec![constants::FILE_BODY]);
                // TODO: register tera filters here.

                // template struct
                let template_dir = src_dir.join(&template_name);
                let email = Self {
                    template,
                    template_name,
                    src_dir,
                    dst_dir,
                    context_data: get_context_data(template_dir),
                    subject: "".to_string(),
                    body: "".to_string(),
                    body_text: "".to_string(),
                };
                Ok(email)
            }
            Err(e) => {
                let msg = format!("{}", e);
                Err(ErrorKind::InvalidDirectory(msg))
            }
        }
    }

    fn render_text(&mut self, file_name: &str) -> Result<String, ErrorKind> {
        let template_name = format!("{}/{}", self.template_name, file_name);
        match self.template.render(&template_name, &self.context_data) {
            Ok(mut rendered) => {
                rendered = rendered
                    .replace("\n\n\n", "<br/><br/>")
                    .replace("\n\n", "<br/>");
                self.strip_tags(&rendered)
            }
            Err(e) => {
                let msg = format!("{}", e);
                Err(ErrorKind::MissingContext(msg))
            }
        }
    }

    fn render_html(&mut self, subject: &str, file_name: &str) -> Result<String, ErrorKind> {
        let template_name = format!("{}/{}", self.template_name, file_name);
        match self.template.render(&template_name, &self.context_data) {
            Ok(rendered) => {
                let mut html_body = HTMLBody::new(self.src_dir.clone());
                html_body.render_html(subject, &rendered)?;
                Ok(html_body.rendered_body)
            }
            Err(e) => {
                let msg = format!("{}", e);
                Err(ErrorKind::MissingContext(msg))
            }
        }
    }

    pub fn render_all(&mut self) -> Result<(), ErrorKind> {
        self.subject = self.render_text(constants::FILE_SUBJECT)?;
        self.body = self.render_html(&self.subject.clone(), constants::FILE_BODY)?;
        self.body_text = self
            .render_text(constants::FILE_BODY_TEXT)
            .unwrap_or(self.strip_tags(self.body.clone().as_str())?);
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
        // TODO: is it possible to trim unwanted chars from `html2text`?
        let stripped = html2text::from_read(text.as_bytes(), constants::TEXT_WIDTH);
        let trimmed: &[_] = &['─', '\n'];
        let normalized = stripped
            .trim_matches(trimmed)
            .split('\n')
            .map(|l| l.trim_end().to_string())
            .collect::<Vec<String>>()
            .join("\n");
        Ok(normalized)
    }
}

pub fn generate_all_templates(src_dir: PathBuf, dst_dir: PathBuf) -> Result<(), ErrorKind> {
    let src_dir = env::current_dir().expect("CURDIR").join(&src_dir);
    let dst_dir = env::current_dir().expect("CURDIR").join(&dst_dir);
    match find_all_templates(src_dir.clone()) {
        Ok(template_names) => {
            for template_name in template_names {
                let template_dst = dst_dir.join(&template_name);
                if let Err(e) = fs::create_dir_all(&template_dst) {
                    let msg = format!("{}", e);
                    return Err(ErrorKind::InvalidDirectory(msg));
                }
                // TODO: do we need to clone these paths?
                let mut email =
                    Email::new(src_dir.clone(), dst_dir.clone(), template_name.clone())?;
                email.render_all()?;
                if let Err(e) = email.save_rendered_outputs() {
                    let msg = format!("{}", e);
                    return Err(ErrorKind::DirectoryAccess(msg));
                }
            }
            Ok(())
        }
        Err(e) => {
            let msg = format!("{}", e);
            Err(ErrorKind::InvalidDirectory(msg))
        }
    }
}
