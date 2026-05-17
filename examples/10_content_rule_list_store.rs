mod common;

use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = ContentRuleListStore::default_store();
    let identifier = "webkit-rs-example-rule-list";
    let _ = store.remove(identifier);
    let encoded_rule_list = r#"[{"trigger":{"url-filter":"ads"},"action":{"type":"block"}}]"#;
    let rule_list = store.compile(identifier, encoded_rule_list)?;
    assert_eq!(rule_list.identifier(), identifier);
    assert!(store
        .available_identifiers()?
        .iter()
        .any(|item| item == identifier));
    assert!(store.lookup(identifier)?.is_some());

    let config = common::base_config();
    config.add_content_rule_list(&rule_list);
    config.remove_content_rule_list(&rule_list);
    store.remove(identifier)?;

    println!("content rule list compiled and removed successfully");
    Ok(())
}
