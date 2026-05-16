use webkit::prelude::*;

#[test]
fn website_data_types_and_records_are_structured() -> Result<(), Box<dyn std::error::Error>> {
    let record = WebsiteDataRecord {
        display_name: "example.test".to_owned(),
        data_types: vec![WebsiteDataType::cookies(), WebsiteDataType::local_storage()],
    };
    let json = serde_json::to_string(&record)?;
    let roundtrip: WebsiteDataRecord = serde_json::from_str(&json)?;
    assert_eq!(WebsiteDataType::cookies().as_str(), "WKWebsiteDataTypeCookies");
    assert_eq!(roundtrip.display_name, "example.test");
    assert_eq!(roundtrip.data_types.len(), 2);
    Ok(())
}
