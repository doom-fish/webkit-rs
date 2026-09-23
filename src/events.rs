use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrainedEvents<T> {
    pub events: Vec<T>,
    pub dropped: u64,
}

impl<T> Default for DrainedEvents<T> {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            dropped: 0,
        }
    }
}

#[derive(Deserialize)]
struct RawDrainedEvents {
    #[serde(default)]
    dropped: u64,
    #[serde(default)]
    events: Vec<Value>,
}

impl<T: DeserializeOwned> DrainedEvents<T> {
    pub(crate) fn from_json(json: &str) -> Self {
        let Ok(raw) = serde_json::from_str::<RawDrainedEvents>(json) else {
            return Self::default();
        };
        let mut dropped = raw.dropped;
        let mut events = Vec::with_capacity(raw.events.len());
        for value in raw.events {
            match serde_json::from_value(value) {
                Ok(event) => events.push(event),
                Err(_) => dropped = dropped.saturating_add(1),
            }
        }
        Self { events, dropped }
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::DrainedEvents;

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    struct Event {
        kind: String,
    }

    #[test]
    fn drained_events_report_queue_drops_and_decode_failures() {
        let drained = DrainedEvents::<Event>::from_json(
            r#"{"dropped":3,"events":[{"kind":"a"},{"unexpected":true},{"kind":"b"}]}"#,
        );
        assert_eq!(
            drained.events,
            vec![
                Event {
                    kind: "a".to_owned()
                },
                Event {
                    kind: "b".to_owned()
                }
            ]
        );
        assert_eq!(drained.dropped, 4);
    }

    #[test]
    fn drained_events_tolerate_malformed_payloads() {
        assert_eq!(
            DrainedEvents::<Event>::from_json("not json"),
            DrainedEvents::default()
        );
        let empty = DrainedEvents::<Event>::from_json(r#"{"dropped":0,"events":[]}"#);
        assert!(empty.events.is_empty());
        assert_eq!(empty.dropped, 0);
        let saturated =
            DrainedEvents::<Event>::from_json(r#"{"dropped":18446744073709551615,"events":[1]}"#);
        assert_eq!(saturated.dropped, u64::MAX);
    }
}
