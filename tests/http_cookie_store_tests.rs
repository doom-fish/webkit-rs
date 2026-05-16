use std::time::{Duration, UNIX_EPOCH};

use webkit::prelude::*;

#[test]
fn cookie_builder_and_policy_roundtrip() {
    let cookie = Cookie::new("token", "abc", "example.test")
        .with_path("/")
        .with_http_only(true)
        .with_secure(true)
        .with_session_only(false)
        .with_expires(UNIX_EPOCH + Duration::from_secs(60));

    assert_eq!(cookie.name, "token");
    assert_eq!(cookie.path, "/");
    assert!(cookie.http_only);
    assert!(cookie.secure);
    assert_eq!(cookie.expires, Some(60));
    assert!(matches!(CookiePolicy::from_raw(1), CookiePolicy::Disallow));
}
