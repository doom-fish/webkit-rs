use webkit::prelude::*;

#[test]
fn website_data_types_and_records_are_structured() -> Result<(), Box<dyn std::error::Error>> {
    let record = WebsiteDataRecord {
        display_name: "example.test".to_owned(),
        data_types: vec![WebsiteDataType::cookies(), WebsiteDataType::local_storage()],
    };
    let json = serde_json::to_string(&record)?;
    let roundtrip: WebsiteDataRecord = serde_json::from_str(&json)?;
    assert_eq!(
        WebsiteDataType::cookies().as_str(),
        "WKWebsiteDataTypeCookies"
    );
    assert_eq!(roundtrip.display_name, "example.test");
    assert_eq!(roundtrip.data_types.len(), 2);
    Ok(())
}

#[test]
fn website_data_store_proxy_configuration_api_is_typed() -> Result<(), Box<dyn std::error::Error>> {
    let summary = ProxyConfigurationSummary {
        description: "SOCKSv5 proxy".to_owned(),
        failover_allowed: true,
        match_domains: vec!["example.test".to_owned()],
        excluded_domains: vec!["static.example.test".to_owned()],
    };
    let json = serde_json::to_string(&summary)?;
    let roundtrip: ProxyConfigurationSummary = serde_json::from_str(&json)?;

    let _: fn(&str, u16) -> Result<ProxyConfiguration, WebKitError> =
        ProxyConfiguration::http_connect;
    let _: fn(&str, u16) -> Result<ProxyConfiguration, WebKitError> = ProxyConfiguration::socks_v5;
    let _: fn(&ProxyConfiguration, &str, Option<&str>) -> Result<(), WebKitError> =
        ProxyConfiguration::set_username_and_password;
    let _: fn(&ProxyConfiguration, bool) -> Result<(), WebKitError> =
        ProxyConfiguration::set_failover_allowed;
    let _: fn(&ProxyConfiguration, &str) -> Result<(), WebKitError> =
        ProxyConfiguration::add_match_domain;
    let _: fn(&ProxyConfiguration) -> Result<(), WebKitError> =
        ProxyConfiguration::clear_match_domains;
    let _: fn(&ProxyConfiguration, &str) -> Result<(), WebKitError> =
        ProxyConfiguration::add_excluded_domain;
    let _: fn(&ProxyConfiguration) -> Result<(), WebKitError> =
        ProxyConfiguration::clear_excluded_domains;
    let _: fn(&ProxyConfiguration) -> Result<ProxyConfigurationSummary, WebKitError> =
        ProxyConfiguration::summary;
    let _: fn(&WebsiteDataStore) -> Result<Vec<ProxyConfiguration>, WebKitError> =
        WebsiteDataStore::proxy_configurations;
    let _: fn(&WebsiteDataStore, &[ProxyConfiguration]) -> Result<(), WebKitError> =
        WebsiteDataStore::set_proxy_configurations;
    let _: fn(&WebsiteDataStore) -> Result<(), WebKitError> =
        WebsiteDataStore::clear_proxy_configurations;

    assert_eq!(roundtrip, summary);
    Ok(())
}

#[test]
#[ignore = "WKWebsiteDataStore proxy bridge tests must run on the process main thread; examples cover live validation"]
fn website_data_store_proxy_configurations_roundtrip_when_supported(
) -> Result<(), Box<dyn std::error::Error>> {
    webkit::init_app();
    let store = WebsiteDataStore::non_persistent();
    let proxy = match ProxyConfiguration::socks_v5("127.0.0.1", 1080) {
        Ok(proxy) => proxy,
        Err(WebKitError::Unsupported(_)) => return Ok(()),
        Err(error) => return Err(Box::new(error)),
    };
    proxy.set_failover_allowed(true)?;
    proxy.add_match_domain("example.test")?;
    proxy.add_excluded_domain("static.example.test")?;

    store.set_proxy_configurations(&[proxy])?;
    let proxy_configurations = store.proxy_configurations()?;
    assert_eq!(proxy_configurations.len(), 1);

    let summary = proxy_configurations[0].summary()?;
    assert!(summary.failover_allowed);
    assert_eq!(summary.match_domains, vec!["example.test"]);
    assert_eq!(summary.excluded_domains, vec!["static.example.test"]);

    store.clear_proxy_configurations()?;
    assert!(store.proxy_configurations()?.is_empty());
    Ok(())
}
