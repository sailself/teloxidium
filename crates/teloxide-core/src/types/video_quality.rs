use serde::{Deserialize, Serialize};

use crate::types::{FileId, FileUniqueId};

/// This object represents a video file of a specific quality.
///
/// [The official docs](https://core.telegram.org/bots/api#videoquality).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct VideoQuality {
    /// Identifier for this file, which can be used to download or reuse the
    /// file.
    pub file_id: FileId,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: FileUniqueId,

    /// Video width.
    pub width: u32,

    /// Video height.
    pub height: u32,

    /// Codec that was used to encode the video, for example, `"h264"`,
    /// `"h265"`, or `"av01"`.
    pub codec: String,

    /// File size in bytes.
    pub file_size: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_large_and_omitted_file_size() {
        let large_file_size: u64 = 4_294_967_296;
        let with_large_size = VideoQuality {
            file_id: FileId("video-q-1".to_owned()),
            file_unique_id: FileUniqueId("uniq-q-1".to_owned()),
            width: 3840,
            height: 2160,
            codec: "h265".to_owned(),
            file_size: Some(large_file_size),
        };
        let json = r#"{"file_id":"video-q-1","file_unique_id":"uniq-q-1","width":3840,"height":2160,"codec":"h265","file_size":4294967296}"#;
        let parsed: VideoQuality = serde_json::from_str(json).unwrap();
        assert_eq!(parsed, with_large_size);
        let reserialized = serde_json::to_string(&with_large_size).unwrap();
        assert_eq!(reserialized, json);
        assert_eq!(serde_json::from_str::<VideoQuality>(&reserialized).unwrap(), with_large_size);

        let without_size_json = r#"{"file_id":"video-q-2","file_unique_id":"uniq-q-2","width":1280,"height":720,"codec":"av01"}"#;
        let parsed_no_size: VideoQuality = serde_json::from_str(without_size_json).unwrap();
        let expected_no_size = VideoQuality {
            file_id: FileId("video-q-2".to_owned()),
            file_unique_id: FileUniqueId("uniq-q-2".to_owned()),
            width: 1280,
            height: 720,
            codec: "av01".to_owned(),
            file_size: None,
        };
        assert_eq!(parsed_no_size, expected_no_size);
        let reserialized_no_size = serde_json::to_string(&parsed_no_size).unwrap();
        assert_eq!(reserialized_no_size, without_size_json);
        assert!(!reserialized_no_size.contains("file_size"));
    }
}
