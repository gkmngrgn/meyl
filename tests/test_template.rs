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

fn get_email(template_name: &str, test_name: &str) -> template::Email {
    let src_dir = get_test_dir(vec!["examples", "src"]);
    let dst_dir = get_random_test_dir(vec!["examples"], test_name);
    let mut email = template::Email::new(src_dir, dst_dir, template_name.to_string()).unwrap();
    email.render_template().unwrap();
    email
}

#[test]
fn generate_all_templates() {
    let src_dir = get_test_dir(vec!["examples", "src"]);
    let dst_dir = get_random_test_dir(vec!["examples"], "generate_all_templates");
    let result = template::generate_all_templates(src_dir, dst_dir);
    assert!(result.is_ok());
}

#[test]
fn test_subject() {
    let email = get_email("wedding-invitation", "test_subject");
    let expected_subject = "You're invited to the wedding Ayşe Özbükeyoğlu & Mehmet Kırmızıkalem";
    assert_eq!(email.subject, expected_subject);
}

#[test]
fn test_text() {
    let email = get_email("wedding-invitation", "test_text");
    let expected_text = merge_string_lines(&[
        "Ayşe Özbükeyoğlu & Mehmet Kırmızıkalem invite you to join them at the",
        "celebration of their wedding.",
        "",
        "Saturday, the fifth of November, two thousand twenty at past seven o'clock in",
        "the evening.",
        "",
        "Our lady of lourdes church",
        "2167 Sparrow Street",
        "Los Angeles, California",
        "",
        "If you need an accessibility support, please reply this mail or call our number",
        "and tell us what you need. We want to\nsee you among us.",
    ]);
    assert_eq!(email.body_text, expected_text);
}

// TODO: comment out this test after you completed inline-css task.
// #[test]
// fn test_body() {
//     let email = get_email("wedding-invitation", "test_body");
//     let expected_subject = "";
//     assert_eq!(email.subject, expected_subject);
// }

#[test]
fn test_text_without_template() {
    // body_text HTML file MUST NOT be existing in this test.
    assert!(!get_test_dir(vec!["examples", "src", "payroll"])
        .join(constants::FILE_BODY_TEXT)
        .exists());

    let email = get_email("payroll", "test_text_without_template");
    let expected_body_text = merge_string_lines(&[
        "I'll update this template later.",
        "",
        "I want to check if the tags are stripped correctly.",
    ]);
    assert_eq!(email.body_text, expected_body_text);
}
