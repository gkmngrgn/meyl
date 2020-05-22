mod seg_tests;

use seg::{constants, template};
use seg_tests::{get_random_test_dir, get_test_dir};

fn merge_string_lines(lines: &[&str]) -> String {
    lines
        .iter()
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

#[test]
fn generate_all_templates() {
    let src_dir = get_test_dir(vec!["examples", "src"]);
    let dst_dir = get_random_test_dir(vec!["examples"], "generate_all_templates");
    let result = template::generate_all_templates(src_dir, dst_dir);
    assert!(result.is_ok());
}

#[test]
fn test_body_text_without_template() {
    let template_name = "payroll";
    let src_dir = get_test_dir(vec!["examples", "src"]);

    // body_text HTML file MUST NOT be existing in this test.
    assert!(!src_dir
        .join(template_name)
        .join(constants::FILE_BODY_TEXT)
        .exists());

    let dst_dir = get_random_test_dir(vec!["examples"], "body_text_without_template");
    let mut email = template::Email::new(src_dir, dst_dir, template_name.to_string()).unwrap();
    email.render_template().unwrap();
    let expected_body_text = merge_string_lines(&[
        "I'll update this template later.",
        "",
        "I want to check if the tags are stripped correctly.",
    ]);
    assert_eq!(email.body_text, expected_body_text);
}
