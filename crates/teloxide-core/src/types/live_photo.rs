use mime::Mime;
use serde::{Deserialize, Serialize};

use crate::types::{FileId, FileUniqueId, PhotoSize, Seconds};

/// This object represents a live photo (a short looping video paired with a
/// static photo).
///
/// [The official docs](https://core.telegram.org/bots/api#livephoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct LivePhoto {
    /// Available sizes of the corresponding static photo.
    pub photo: Option<Vec<PhotoSize>>,

    /// Identifier for the video file, which can be used to download or reuse
    /// the file.
    pub file_id: FileId,

    /// Unique identifier for the video file, which is supposed to be the same
    /// over time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: FileUniqueId,

    /// Video width as defined by the sender.
    pub width: u32,

    /// Video height as defined by the sender.
    pub height: u32,

    /// Duration of the video in seconds as defined by the sender.
    pub duration: Seconds,

    /// MIME type of the file as defined by the sender.
    #[serde(default, with = "crate::types::non_telegram_types::mime::opt_deser")]
    #[cfg_attr(test, schemars(with = "Option<String>"))]
    pub mime_type: Option<Mime>,

    /// File size in bytes.
    pub file_size: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::FileMeta;

    #[test]
    fn roundtrip_and_large_file_size() {
        let json = r#"{
            "photo": [
                {
                    "file_id": "photo-id-1",
                    "file_unique_id": "photo-uniq-1",
                    "width": 1280,
                    "height": 720,
                    "file_size": 54321
                }
            ],
            "file_id": "live-vid-1",
            "file_unique_id": "live-uniq-1",
            "width": 1280,
            "height": 720,
            "duration": 3,
            "mime_type": "video/mp4",
            "file_size": 4294967296
        }"#;
        let parsed: LivePhoto = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.file_id, FileId("live-vid-1".to_owned()));
        assert_eq!(parsed.file_unique_id, FileUniqueId("live-uniq-1".to_owned()));
        assert_eq!(parsed.duration, Seconds::from_seconds(3));
        assert_eq!(
            parsed.mime_type.as_ref().map(ToString::to_string).as_deref(),
            Some("video/mp4")
        );
        assert_eq!(parsed.file_size, Some(4_294_967_296));
        assert_eq!(
            parsed.photo.as_ref().unwrap()[0],
            PhotoSize {
                file: FileMeta {
                    id: FileId("photo-id-1".to_owned()),
                    unique_id: FileUniqueId("photo-uniq-1".to_owned()),
                    size: 54321,
                },
                width: 1280,
                height: 720,
            }
        );

        let encoded = serde_json::to_string(&parsed).unwrap();
        let roundtrip: LivePhoto = serde_json::from_str(&encoded).unwrap();
        assert_eq!(parsed, roundtrip);
    }
}
