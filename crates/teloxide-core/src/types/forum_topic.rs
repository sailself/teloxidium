use serde::{Deserialize, Serialize};

use crate::types::{CustomEmojiId, Rgb, ThreadId};

/// This object represents a forum topic.
///
/// [The official docs](https://core.telegram.org/bots/api#forumtopiccreated).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ForumTopic {
    /// Unique identifier of the forum topic
    #[serde(rename = "message_thread_id")]
    pub thread_id: ThreadId,

    /// Name of the topic.
    pub name: String,

    /// Color of the topic icon in RGB format.
    pub icon_color: Rgb,

    /// Unique identifier of the custom emoji shown as the topic icon.
    pub icon_custom_emoji_id: Option<CustomEmojiId>,

    /// `true`, if the name of the topic wasn't specified explicitly by its
    /// creator and likely needs to be changed by the bot.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_name_implicit: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialization() {
        let legacy_json = r#"{"message_thread_id":4,"icon_color":9367192,"icon_custom_emoji_id":"5312536423851630001","name":"General"}"#;
        let legacy: ForumTopic = serde_json::from_str(legacy_json).unwrap();
        assert!(!legacy.is_name_implicit);

        let implicit_json = r#"{"message_thread_id":5,"icon_color":9367192,"name":"Implicit","is_name_implicit":true}"#;
        let implicit: ForumTopic = serde_json::from_str(implicit_json).unwrap();
        assert!(implicit.is_name_implicit);
        let roundtrip: ForumTopic =
            serde_json::from_str(&serde_json::to_string(&implicit).unwrap()).unwrap();
        assert_eq!(implicit, roundtrip);
    }
}
