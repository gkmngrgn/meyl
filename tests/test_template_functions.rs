mod meyl_tests;

#[test]
fn test_static_function() {
    let email = get_email("src-simple", "newsletter", "test_static_function");
    let expected_subject = "A link in subject: http://gokmengorgen.net/en/";
    let expected_text = indoc!(
    );
    assert_eq!(email.subject, expected_subject);
}
