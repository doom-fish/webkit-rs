use webkit::prelude::*;

#[test]
#[ignore = "WKContentRuleListStore bridge tests must run on the process main thread; examples cover live validation"]
fn content_rule_list_store_compiles_and_looks_up_rules() -> Result<(), Box<dyn std::error::Error>> {
    let store = ContentRuleListStore::default_store();
    let identifier = "webkit-rs-test-rule-list";
    let _ = store.remove(identifier);
    let rule_list = store.compile(
        identifier,
        r#"[{"trigger":{"url-filter":"ads"},"action":{"type":"block"}}]"#,
    )?;
    assert_eq!(rule_list.identifier(), identifier);
    assert!(store.lookup(identifier)?.is_some());
    assert!(store
        .available_identifiers()?
        .iter()
        .any(|item| item == identifier));
    store.remove(identifier)?;
    Ok(())
}
