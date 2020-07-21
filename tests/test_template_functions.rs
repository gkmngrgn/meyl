mod meyl_tests;

use meyl_tests::*;

#[test]
fn test_static_function() {
    let email = get_email("src_simple", "newsletter", "test_static_function");
    let expected_subject = "A link in subject: http://gokmengorgen.net/en/";
    let expected_body_text = "A link in body text: https://gokmengorgen.net/live/";
    let expected_body = normalize_html(
        r#"
        <!DOCTYPE html>
        <html xmlns="http://www.w3.org/1999/xhtml">
            <head>
                <meta content="text/html; charset=utf-8" http-equiv="Content-Type">
                <meta content="width=device-width" name="viewport">
                <title>A link in subject: http://gokmengorgen.net/en/</title>
            </head>
            <body>
                <p><a href="http://www.gokmengorgen.net/en/license/">A link</a> in body.</p>
            </body>
        </html>
        "#,
    );
    assert_eq!(email.subject, expected_subject);
    assert_eq!(email.body_text, expected_body_text);
    assert_eq!(email.body, expected_body);
}
