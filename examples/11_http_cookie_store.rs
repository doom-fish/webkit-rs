mod common;

use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    let website_data_store = config.website_data_store().expect("website data store");
    let cookie_store = website_data_store.http_cookie_store()?;
    cookie_store.start_observing();

    let cookie = Cookie::new("session", "abc123", "example.test")
        .with_path("/")
        .with_http_only(true);
    cookie_store.set_cookie(&cookie)?;
    let cookies = cookie_store.all_cookies()?;
    assert!(cookies.iter().any(|candidate| candidate.name == "session"));
    let _ = cookie_store.set_cookie_policy(CookiePolicy::Allow);
    let _ = cookie_store.cookie_policy();
    cookie_store.delete_cookie(&cookie)?;
    let cookies_after_delete = cookie_store.all_cookies()?;
    assert!(!cookies_after_delete
        .iter()
        .any(|candidate| candidate.name == "session"));

    println!(
        "cookie observer events: {}",
        cookie_store.drain_events().len()
    );
    Ok(())
}
