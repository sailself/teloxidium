use serde::{Deserialize, Serialize};

use crate::types::Audio;

/// This object represents the audios displayed on a user's profile.
///
/// [The official docs](https://core.telegram.org/bots/api#userprofileaudios).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UserProfileAudios {
    /// Total number of profile audios for the target user.
    pub total_count: u32,

    /// Requested profile audios.
    pub audios: Vec<Audio>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FileId, FileMeta, FileUniqueId, Seconds};

    #[test]
    fn roundtrip() {
        let profile_audios = UserProfileAudios {
            total_count: 1,
            audios: vec![Audio {
                file: FileMeta {
                    id: FileId("audio-1".to_owned()),
                    unique_id: FileUniqueId("uniq-audio-1".to_owned()),
                    size: 4096,
                },
                duration: Seconds::from_seconds(180),
                performer: Some("Artist".to_owned()),
                title: Some("Track".to_owned()),
                file_name: Some("track.mp3".to_owned()),
                mime_type: Some("audio/mpeg".parse().unwrap()),
                thumbnail: None,
            }],
        };

        let json = serde_json::to_string(&profile_audios).unwrap();
        let parsed: UserProfileAudios = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, profile_audios);
    }
}
