mod meyl_tests;

use indoc::indoc;
use meyl::{constants, template};
use meyl_tests::{get_random_test_dir, get_test_dir};

fn normalize_html(body: &str) -> String {
    body.trim_matches(|c| c == '\n' || c == ' ')
        .split("\n")
        .map(|l| l.trim_start().to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn get_email(src_name: &str, template_name: &str, test_name: &str) -> template::Email {
    let src_dir = get_test_dir(vec!["examples", src_name]);
    let dst_dir = get_random_test_dir(vec!["examples"], test_name);
    let mut email = template::Email::new(src_dir, dst_dir, template_name.to_string()).unwrap();
    email.render_all().unwrap();
    email
}

#[test]
fn test_generate_all_templates() {
    let src_dirs = &["src", "src-simple"];
    src_dirs.iter().for_each(|dir_name| {
        let src_dir = get_test_dir(vec!["examples", dir_name]);
        let dst_dir = get_random_test_dir(vec!["examples"], "generate_all_templates-1");
        let result = template::generate_all_templates(src_dir, dst_dir);
        assert!(result.is_ok());
    });
}

#[test]
fn test_subject() {
    let email = get_email("src", "wedding-invitation", "test_subject");
    let expected_subject = "You're invited to the wedding Ayşe Özbükeyoğlu & Mehmet Kırmızıkalem";
    assert_eq!(email.subject, expected_subject);
}

#[test]
fn test_text() {
    let email = get_email("src", "wedding-invitation", "test_text");
    let expected_text = indoc!(
        r#"
        Ayşe Özbükeyoğlu & Mehmet Kırmızıkalem invite you to join them at the
        celebration of their wedding.

        Saturday, the fifth of November, two thousand twenty at past seven o'clock in
        the evening.

        Our lady of lourdes church
        2167 Sparrow Street
        Los Angeles, California

        If you need an accessibility support, please reply this mail or call our number
        and tell us what you need. We want to see you among us.
        "#
    );
    assert_eq!(email.body_text, expected_text);
}

#[test]
fn test_body() {
    let email = get_email("src-simple", "new-article", "test_body");
    let expected_body = normalize_html(
        r#"
        <!DOCTYPE html>
        <html xmlns="http://www.w3.org/1999/xhtml">
            <head>
                <meta content="text/html; charset=utf-8" http-equiv="Content-Type">
                <meta content="width=device-width" name="viewport">
                <title>A new article was published!</title>
            </head>
            <body>
                <h1>Hello followers</h1>
                <blockquote>
                    Lorem ipsum dolor sit amet, consectetur adipiscing
                    elit. Phasellus in diam diam. Nunc faucibus egestas
                    nisl, et fringilla ex tristique nec. Morbi nisl magna,
                    blandit sit amet congue in, iaculis at massa. Sed
                    placerat sapien at quam dignissim elementum. Donec est
                    massa, vestibulum eget porttitor in, porttitor eget
                    tortor. Nullam pharetra quam in eleifend
                    tempus. Vestibulum varius condimentum tortor ac mattis.
                </blockquote>
                <p>
                    Happy hacking,<br>
                    Yamamura
                </p>
            </body>
        </html>
        "#,
    );
    assert_eq!(email.body, expected_body);
}

#[test]
fn test_text_without_template() {
    // body_text HTML file MUST NOT be existing in this test.
    assert!(!get_test_dir(vec!["examples", "src", "payroll"])
        .join(constants::FILE_BODY_TEXT)
        .exists());

    let email = get_email("src", "payroll", "test_text_without_template");
    let expected_body_text = indoc!(
        r#"
        # Dear John Doe,

        I've added you as an accountant in our office organization. Just click on the
        button below and you will gain instant access to our account.

        [ Accept invitation ][1]

        Regards,
        HANKA Precision Instruments

        [1] http://localhost/invitation-link/
        "#
    );
    assert_eq!(email.body_text, expected_body_text);
}
