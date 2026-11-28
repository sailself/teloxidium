use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::SuggestedPostPrice;

/// Contains information about a suggested post.
///
/// [The official docs](https://core.telegram.org/bots/api#suggestedpostinfo).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct SuggestedPostInfo {
    /// State of the suggested post
    pub state: SuggestedPostState,

    /// Proposed price of the post. If the field is omitted, then the post is
    /// unpaid.
    pub price: Option<SuggestedPostPrice>,

    /// Proposed send date of the post. If the field is omitted, then the post
    /// can be published at any time within 30 days at the sole discretion of
    /// the user or administrator who approves it.
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "Option<i64>"))]
    pub send_date: Option<DateTime<Utc>>,
}

/// State of the suggested post
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub enum SuggestedPostState {
    Pending,
    Approved,
    Declined,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_without_send_date() {
        let json = r#"{"state":"pending"}"#;
        let info: SuggestedPostInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.state, SuggestedPostState::Pending);
        assert_eq!(info.price, None);
        assert_eq!(info.send_date, None);

        let update_json = r#"{
            "update_id": 10,
            "message": {
                "message_id": 20,
                "chat": { "id": -100123, "title": "Channel DM", "type": "supergroup" },
                "date": 1700000000,
                "suggested_post_info": { "state": "pending" },
                "text": "Suggested post without send date"
            }
        }"#;
        let update: crate::types::Update = serde_json::from_str(update_json).unwrap();
        match &update.kind {
            crate::types::UpdateKind::Message(msg) => {
                assert_eq!(msg.suggested_post_info.as_ref().and_then(|s| s.send_date), None);
                assert_eq!(msg.text(), Some("Suggested post without send date"));
            }
            other => panic!("expected Message update, got {other:?}"),
        }
    }
}
