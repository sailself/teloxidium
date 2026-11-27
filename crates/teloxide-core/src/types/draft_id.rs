use serde::{Deserialize, Serialize};

/// Unique identifier of a message draft.
///
/// Must be non-zero. Changes to drafts with the same identifier are animated;
/// otherwise, the draft is replaced without animation.
#[derive(Clone, Copy)]
#[derive(Debug, derive_more::Display)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct DraftId(pub i64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
        struct S {
            draft_id: DraftId,
        }

        let value = S { draft_id: DraftId(42) };
        let json = r#"{"draft_id":42}"#;

        assert_eq!(serde_json::to_string(&value).unwrap(), json);
        assert_eq!(serde_json::from_str::<S>(json).unwrap(), value);
    }
}
