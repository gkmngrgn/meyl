mod helpers;

use seg;
use std::fs;

#[test]
fn generate_all_templates() {
    let src_dir = helpers::get_test_dir(&["examples", "src"]);
    let dst_dir = helpers::get_test_dir(&["examples", "dst-generate_all_templates"]);
    if dst_dir.exists() {
        if let Err(_) = fs::remove_dir_all(&dst_dir) {
            assert!(false); // You need to remove the existing directory before testing it carefully.
        }
    }

    let result = seg::template::generate_all_templates(src_dir, dst_dir);
    assert!(result.is_ok());
}
