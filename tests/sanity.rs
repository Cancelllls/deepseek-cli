use deepseek_cli::Config;

#[test]
fn test_config_valid() {
    let config = Config::new("key123", "deepseek-chat").unwrap();
    assert_eq!(config.api_key, "key123");
    assert_eq!(config.model, "deepseek-chat");
}

#[test]
fn test_config_empty_key() {
    assert!(Config::new("", "deepseek-chat").is_err());
}

#[test]
fn test_config_empty_model() {
    assert!(Config::new("key123", "").is_err());
}

#[test]
fn test_config_both_empty() {
    assert!(Config::new("", "").is_err());
}
