use mime::Mime;
use serde::{Deserialize, Serialize};

use crate::types::{FileMeta, PhotoSize, Seconds, VideoQuality};

/// This object represents a video file.
///
/// [The official docs](https://core.telegram.org/bots/api#video).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Video {
    /// Metadata of the video file.
    #[serde(flatten)]
    pub file: FileMeta,

    /// Video width as defined by sender.
    pub width: u32,

    /// Video height as defined by sender.
    pub height: u32,

    /// Duration of the video in seconds as defined by sender.
    pub duration: Seconds,

    /// Video thumbnail.
    pub thumbnail: Option<PhotoSize>,

    /// Available sizes of the cover of the video in the message
    pub cover: Option<Vec<PhotoSize>>,

    /// Timestamp in seconds from which the video will play in the message
    pub start_timestamp: Option<Seconds>,

    /// List of available qualities of the video
    pub qualities: Option<Vec<VideoQuality>>,

    /// Original filename as defined by sender
    pub file_name: Option<String>,

    /// Mime type of a file as defined by sender.
    #[serde(with = "crate::types::non_telegram_types::mime::opt_deser")]
    #[cfg_attr(test, schemars(with = "Option<String>"))]
    pub mime_type: Option<Mime>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FileId, FileUniqueId};

    #[test]
    fn qualities_and_legacy_roundtrip() {
        let legacy_json = r#"{
            "file_id": "vid-1",
            "file_unique_id": "u-vid-1",
            "file_size": 1024,
            "width": 1280,
            "height": 720,
            "duration": 10,
            "mime_type": "video/mp4"
        }"#;
        let legacy: Video = serde_json::from_str(legacy_json).unwrap();
        assert!(legacy.qualities.is_none());

        let with_qualities_json = r#"{
            "file_id": "vid-1",
            "file_unique_id": "u-vid-1",
            "file_size": 2048,
            "width": 1920,
            "height": 1080,
            "duration": 15,
            "mime_type": "video/mp4",
            "qualities": [
                {
                    "file_id": "q-720",
                    "file_unique_id": "uq-720",
                    "file_size": 1024,
                    "width": 1280,
                    "height": 720,
                    "codec": "h264"
                }
            ]
        }"#;
        let parsed: Video = serde_json::from_str(with_qualities_json).unwrap();
        assert_eq!(
            parsed.qualities,
            Some(vec![VideoQuality {
                file_id: FileId("q-720".to_owned()),
                file_unique_id: FileUniqueId("uq-720".to_owned()),
                width: 1280,
                height: 720,
                codec: "h264".to_owned(),
                file_size: Some(1024),
            }])
        );
        let roundtrip: Video =
            serde_json::from_str(&serde_json::to_string(&parsed).unwrap()).unwrap();
        assert_eq!(parsed, roundtrip);
    }
}
