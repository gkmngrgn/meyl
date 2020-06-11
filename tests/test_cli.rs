mod meyl_tests;

use meyl;
use meyl_tests::get_test_dir;

#[test]
fn find_all_template_folders() {
    let src_dir = get_test_dir(vec!["examples", "src"]);
    let template_dirs = meyl::find_all_templates(src_dir);
    assert!(template_dirs.is_ok());
    assert_eq!(
        template_dirs.unwrap(),
        &["payroll".to_string(), "wedding-invitation".to_string()]
    );
}
