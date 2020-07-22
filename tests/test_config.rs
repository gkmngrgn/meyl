mod meyl_tests;

use meyl::config::{Config, Table};
use meyl_tests::*;

#[test]
fn test_config_invalid_path() {
    let template_dir = get_test_dir(vec!["examples", "src_simple", "there_is_nothing_here"]);
    let config = Config::new(template_dir);
    assert_eq!(true, config.is_err());
}

#[test]
fn test_config_no_context_data() {
    let template_dir = get_test_dir(vec!["examples", "src_simple", "config_no_context_data"]);
    let config = Config::new(template_dir).unwrap();
    assert_eq!("http://duckduckgo.com", config.get_base_url());
    assert_eq!(Table::new(), config.get_context_data());
}

#[test]
fn test_config_full_featured() {
    let template_dir = get_test_dir(vec!["examples", "src_simple", "config_full_featured"]);
    let config = Config::new(template_dir).unwrap();
    assert_eq!("https://gokmengorgen.net", config.get_base_url());
}
