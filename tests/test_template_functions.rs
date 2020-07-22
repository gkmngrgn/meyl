mod meyl_tests;

use indoc::indoc;
use meyl_tests::*;

#[test]
fn test_static_function() {
    let email = get_email("src_simple", "newsletter", "test_static_function");
    let expected_subject = "A link in subject: https://gokmengorgen.net/en/";
    let expected_body_text = indoc!(
        r#"
        A link in body text: https://gokmengorgen.net/live/
        "#
    );
    let expected_body = normalize_html(
        r#"
        <!DOCTYPE html>
        <html xmlns="http://www.w3.org/1999/xhtml">
            <head>
                <meta content="text/html; charset=utf-8" http-equiv="Content-Type">
                <meta content="width=device-width" name="viewport">
                <title>A link in subject: https://gokmengorgen.net/en/</title>
            </head>
            <body>
                <p>
                    <a href="https://gokmengorgen.net/en/license/">A link</a> in body.
                </p>
            </body>
        </html>
        "#,
    );
    assert_eq!(expected_subject, email.subject);
    assert_eq!(expected_body_text, email.body_text);
    assert_eq!(expected_body, email.body);
}
