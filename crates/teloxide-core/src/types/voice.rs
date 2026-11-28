use mime::Mime;
use serde::{Deserialize, Serialize};

use crate::types::{FileMeta, Seconds};

/// This object represents a voice note.
///
/// [The official docs](https://core.telegram.org/bots/api#voice).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Voice {
    /// Metadata of the voice file.
    #[serde(flatten)]
    pub file: FileMeta,

    /// Duration of the audio in seconds as defined by sender.
    pub duration: Seconds,

    /// MIME type of the file as defined by sender.
    #[serde(default, with = "crate::types::non_telegram_types::mime::opt_deser")]
    #[cfg_attr(test, schemars(with = "Option<String>"))]
    pub mime_type: Option<Mime>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_without_mime_type() {
        let json = r#"{
            "file_id": "voice-no-mime",
            "file_unique_id": "u-voice-no-mime",
            "duration": 7
        }"#;
        let parsed: Voice = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.mime_type, None);

        let update_json = r#"{
            "update_id": 4,
            "message": {
                "message_id": 13,
                "chat": { "id": 100, "first_name": "Alice", "type": "private" },
                "date": 1700000000,
                "voice": {
                    "file_id": "voice-no-mime",
                    "file_unique_id": "u-voice-no-mime",
                    "duration": 7
                }
            }
        }"#;
        let update: crate::types::Update = serde_json::from_str(update_json).unwrap();
        match &update.kind {
            crate::types::UpdateKind::Message(msg) => {
                assert!(msg.voice().is_some());
                assert_eq!(msg.voice().and_then(|v| v.mime_type.as_ref()), None);
            }
            other => panic!("expected Message update, got {other:?}"),
        }
    }
}
