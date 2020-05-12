use std::fs;
use std::path::PathBuf;

const BODY_FILENAME: &str = "body.html";
// const BODY_TEXT_FILENAME: &str = "body_text.html";
const SUBJECT_FILENAME: &str = "subject.html";

fn is_valid_template_dir(template_dir: &PathBuf) -> bool {
    [BODY_FILENAME, SUBJECT_FILENAME]
        .iter()
        .all(|filename| template_dir.join(filename).exists())
}

pub fn find_all_templates(template_dir: PathBuf) -> std::io::Result<Vec<PathBuf>> {
    let mut template_dirs = vec![];
    for entry in fs::read_dir(&template_dir)? {
        if !entry.is_ok() {
            continue;
        }
        let template_dir = entry.unwrap().path();
        if !is_valid_template_dir(&template_dir) {
            continue;
        }
        template_dirs.push(template_dir);
    }
    Ok(template_dirs)
}
