use steam_proxy::config::Config;
use std::fs::File;
use std::io::Write;

#[test]
fn test_config_load() {
    let config_content = r#"
[server]
address = "127.0.0.1"
port = 8080

[domain_fronting]
front_domain = "front.example.com"
target_domain = "target.example.com"
cdn_provider = "cloudflare"
"#;

    let mut file = File::create("test_config.toml").unwrap();
    file.write_all(config_content.as_bytes()).unwrap();

    let config = Config::load("test_config.toml").unwrap();
    assert_eq!(config.server.address, "127.0.0.1");
    assert_eq!(config.server.port, 8080);
    assert_eq!(config.domain_fronting.front_domain, "front.example.com");
    assert_eq!(config.domain_fronting.target_domain, "target.example.com");
    assert_eq!(config.domain_fronting.cdn_provider.unwrap(), "cloudflare");

    std::fs::remove_file("test_config.toml").unwrap();
}
