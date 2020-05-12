mod helpers;

use seg;
use std::path::PathBuf;

#[test]
fn find_all_template_folders() {
    let src_dir = helpers::get_test_dir(&["examples", "src"]);
    let template_folders = seg::find_all_templates(src_dir);
    assert!(template_folders.is_ok());
    assert_eq!(
        template_folders.unwrap(),
        &[
            PathBuf::from("examples/src/payroll"),
            PathBuf::from("examples/src/wedding-invitation")
        ]
    );
}
