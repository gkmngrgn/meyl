use std::path::PathBuf;

fn get_base_dir() -> PathBuf {
    let mut folder = PathBuf::from(file!());
    folder.pop();
    folder.pop();
    folder
}

pub fn get_test_dir(sub_dirs: &[&str]) -> PathBuf {
    let mut template_dir = get_base_dir();
    sub_dirs.iter().for_each(|dir| template_dir.push(dir));
    template_dir
}
