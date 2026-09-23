use std::ffi::CString;

use serde::{Deserialize, Serialize};

use crate::private::to_cstring;

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "camelCase")]
pub enum ContentWorld {
    #[default]
    Page,
    DefaultClient,
    Named(String),
}

impl ContentWorld {
    pub(crate) fn to_ffi(&self) -> (i32, Option<CString>) {
        match self {
            Self::Page => (0, None),
            Self::DefaultClient => (1, None),
            Self::Named(name) => (2, Some(to_cstring(name))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContentWorld;

    #[test]
    fn content_world_round_trips_through_bridge_json() -> Result<(), serde_json::Error> {
        for (world, json) in [
            (ContentWorld::Page, r#"{"kind":"page"}"#),
            (ContentWorld::DefaultClient, r#"{"kind":"defaultClient"}"#),
            (
                ContentWorld::Named("host".to_owned()),
                r#"{"kind":"named","name":"host"}"#,
            ),
        ] {
            assert_eq!(serde_json::to_string(&world)?, json);
            assert_eq!(serde_json::from_str::<ContentWorld>(json)?, world);
        }
        Ok(())
    }

    #[test]
    fn content_world_ffi_encoding_matches_the_bridge() {
        assert_eq!(ContentWorld::Page.to_ffi().0, 0);
        assert_eq!(ContentWorld::DefaultClient.to_ffi().0, 1);
        let (kind, name) = ContentWorld::Named("isolated".to_owned()).to_ffi();
        assert_eq!(kind, 2);
        assert_eq!(
            name.as_deref().and_then(|n| n.to_str().ok()),
            Some("isolated")
        );
        assert!(ContentWorld::Page.to_ffi().1.is_none());
        assert_eq!(ContentWorld::default(), ContentWorld::Page);
    }
}
