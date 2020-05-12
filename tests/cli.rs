use seg;
use std::path::PathBuf;

fn get_base_dir() -> PathBuf {
    let mut folder = PathBuf::from(file!());
    folder.pop();
    folder.pop();
    folder
}

#[test]
fn find_all_template_folders() {
    let mut template_dir = get_base_dir();
    template_dir.push("examples");
    template_dir.push("src");

    let template_folders = seg::find_all_templates(template_dir);
    assert!(template_folders.is_ok());
    assert_eq!(
        template_folders.unwrap(),
        &[
            PathBuf::from("examples/src/payroll"),
            PathBuf::from("examples/src/wedding-invitation")
        ]
    );
}
