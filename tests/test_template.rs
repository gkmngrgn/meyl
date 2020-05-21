mod seg_tests;

use seg::template;
use seg_tests::{get_random_test_dir, get_test_dir};

#[test]
fn generate_all_templates() {
    let src_dir = get_test_dir(vec!["examples", "src"]);
    let dst_dir = get_random_test_dir(vec!["examples"], "generate_all_templates");
    let result = template::generate_all_templates(src_dir, dst_dir);
    assert!(result.is_ok());
}
