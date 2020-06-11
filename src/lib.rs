pub mod config;
pub mod constants;
pub mod template;

use std::fs;
use std::path::PathBuf;

fn is_valid_template_dir(template_dir: &PathBuf) -> bool {
    [constants::FILE_BODY, constants::FILE_SUBJECT]
        .iter()
        .all(|filename| template_dir.join(filename).exists())
}

pub fn find_all_templates(template_dir: PathBuf) -> std::io::Result<Vec<String>> {
    let mut template_dirs = vec![];
    for entry in fs::read_dir(&template_dir)? {
        if !entry.is_ok() {
            continue;
        }
        let template_dir = entry.unwrap().path();
        if !is_valid_template_dir(&template_dir) {
            continue;
        }
        template_dirs.push(
            template_dir
                .iter()
                .last()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
    }
    template_dirs.sort();
    Ok(template_dirs)
}
