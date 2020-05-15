mod helpers;

use seg;

#[test]
fn find_all_template_folders() {
    let src_dir = helpers::get_test_dir(&["examples", "src"]);
    let template_folders = seg::find_all_templates(src_dir);
    assert!(template_folders.is_ok());
    assert_eq!(
        template_folders.unwrap(),
        &["payroll".to_string(), "wedding-invitation".to_string()]
    );
}
