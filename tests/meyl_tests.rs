#![allow(dead_code)]

use meyl::template;
use std::path::PathBuf;

pub fn get_test_dir(sub_dirs: Vec<&str>) -> PathBuf {
    let mut dir = PathBuf::from(file!());
    dir.pop();
    dir.pop();
    sub_dirs.iter().for_each(|d| dir.push(d));
    dir
}

pub fn get_random_test_dir(sub_dirs: Vec<&str>, name: &str) -> PathBuf {
    // I don't want to use remove_dir_all in this project because of two cases:
    // 1. It's not safe. All developers including me can do any
    //    mistake. Remember the story of Nvidia / Bumblebee project.
    // 2. There's no good solution for supporting both Windows and *nix systems.
    let mut dir;
    let mut number = 0;
    loop {
        let dir_name = format!("dst-{}-{}", name, number);
        let mut dirs = sub_dirs.to_vec();
        dirs.push(&dir_name);
        dir = get_test_dir(dirs);
        if !dir.exists() {
            break;
        }
        number += 1;
    }
    dir
}

pub fn normalize_html(body: &str) -> String {
    body.trim_matches(|c| c == '\n' || c == ' ')
        .split('\n')
        .map(|l| l.trim_start().to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn get_email(src_name: &str, template_name: &str, test_name: &str) -> template::Email {
    let src_dir = get_test_dir(vec!["examples", src_name]);
    let dst_dir = get_random_test_dir(vec!["examples"], test_name);
    let mut email = template::Email::new(src_dir, dst_dir, template_name.to_string()).unwrap();
    email.render_all().unwrap();
    email
}
