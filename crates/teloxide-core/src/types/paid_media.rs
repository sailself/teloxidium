use serde::{Deserialize, Serialize};

use crate::types::{LivePhoto, PhotoSize, Seconds, Video};

/// Describes the paid media added to a message.
///
/// [The official docs](https://core.telegram.org/bots/api#paidmediainfo).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PaidMediaInfo {
    /// The number of Telegram Stars that must be paid to buy access to the
    /// media.
    pub star_count: u32,

    /// Information about the paid media.
    pub paid_media: Vec<PaidMedia>,
}

/// This object describes paid media.
///
/// [The official docs](https://core.telegram.org/bots/api#paidmedia).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum PaidMedia {
    Preview(PaidMediaPreview),
    Photo(PaidMediaPhoto),
    LivePhoto(Box<PaidMediaLivePhoto>),
    Video(Box<PaidMediaVideo>),
}

/// The paid media isn't available before the payment.
///
/// [The official docs](https://core.telegram.org/bots/api#paidmediapreview).
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PaidMediaPreview {
    /// Media width as defined by the sender.
    pub width: Option<u32>,

    /// Media height as defined by the sender.
    pub height: Option<u32>,

    /// Duration of the media in seconds as defined by the sender.
    pub duration: Option<Seconds>,
}

/// The paid media is a photo.
///
/// [The official docs](https://core.telegram.org/bots/api#paidmediaphoto).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PaidMediaPhoto {
    pub photo: Vec<PhotoSize>,
}

/// The paid media is a live photo.
///
/// [The official docs](https://core.telegram.org/bots/api#paidmedialivephoto).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PaidMediaLivePhoto {
    pub live_photo: LivePhoto,
}

/// The paid media is a video.
///
/// [The official docs](https://core.telegram.org/bots/api#paidmediavideo).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PaidMediaVideo {
    pub video: Video,
}

impl PaidMedia {
    pub fn preview(&self) -> Option<PaidMediaPreview> {
        match self {
            Self::Preview(preview) => Some(*preview),
            _ => None,
        }
    }

    pub fn photo(&self) -> Option<PhotoSize> {
        match self {
            Self::Photo(photo) => Some(photo.photo.last()?.clone()),
            _ => None,
        }
    }

    pub fn live_photo(&self) -> Option<&LivePhoto> {
        match self {
            Self::LivePhoto(live_photo) => Some(&live_photo.live_photo),
            _ => None,
        }
    }

    pub fn video(&self) -> Option<&Video> {
        match self {
            Self::Video(video) => Some(&video.video),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FileId, FileUniqueId, Seconds};

    #[test]
    fn live_photo_roundtrip() {
        let json = r#"{
            "type": "live_photo",
            "live_photo": {
                "file_id": "lp-vid",
                "file_unique_id": "lp-uniq",
                "width": 1080,
                "height": 1920,
                "duration": 3
            }
        }"#;
        let parsed: PaidMedia = serde_json::from_str(json).unwrap();
        let lp = parsed.live_photo().expect("expected PaidMedia::LivePhoto");
        assert_eq!(lp.file_id, FileId("lp-vid".to_owned()));
        assert_eq!(lp.file_unique_id, FileUniqueId("lp-uniq".to_owned()));
        assert_eq!(lp.duration, Seconds::from_seconds(3));

        let roundtrip: PaidMedia =
            serde_json::from_str(&serde_json::to_string(&parsed).unwrap()).unwrap();
        assert_eq!(parsed, roundtrip);
    }
}
