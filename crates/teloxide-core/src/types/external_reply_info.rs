use serde::{Deserialize, Serialize};

use crate::types::{
    Animation, Audio, Chat, Checklist, Contact, Dice, Document, Game, Giveaway, GiveawayWinners,
    Invoice, LinkPreviewOptions, LivePhoto, Location, MessageId, MessageOrigin, PaidMediaInfo,
    PhotoSize, Poll, Sticker, Story, Venue, Video, VideoNote, Voice,
};

/// This object contains information about a message that is being replied to,
/// which may come from another chat or forum topic.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ExternalReplyInfo {
    /// Origin of the message replied to by the given message.
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a
    /// supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if
    /// the original chat is a supergroup or a channel.
    #[serde(default, with = "crate::types::option_msg_id_as_int")]
    #[cfg_attr(test, schemars(with = "Option<i32>"))]
    pub message_id: Option<MessageId>,
    /// Options used for link preview generation for the original message, if it
    /// is a text message.
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// _true_, if the message media is covered by a spoiler animation.
    #[serde(default)]
    pub has_media_spoiler: bool,

    /// Content that Telegram includes about the original message, if any.
    #[serde(flatten)]
    pub kind: Option<ExternalReplyInfoKind>,
}

#[derive(Deserialize)]
struct ExternalReplyInfoHeader {
    origin: MessageOrigin,
    chat: Option<Chat>,
    #[serde(default, with = "crate::types::option_msg_id_as_int")]
    message_id: Option<MessageId>,
    link_preview_options: Option<LinkPreviewOptions>,
    #[serde(default)]
    has_media_spoiler: bool,
}

fn take_non_null(
    map: &mut serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Option<serde_json::Value> {
    map.remove(key).filter(|val| !val.is_null())
}

fn parse_variant<T, F>(
    val: serde_json::Value,
    wrap: F,
) -> Result<ExternalReplyInfoKind, serde_json::Error>
where
    T: serde::de::DeserializeOwned,
    F: FnOnce(T) -> ExternalReplyInfoKind,
{
    T::deserialize(val).map(wrap)
}

impl<'de> Deserialize<'de> for ExternalReplyInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut map = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;

        // Probe and extract only the highest-precedence content key first so a
        // malformed or changed lower-precedence compatibility field cannot break
        // decoding of the primary media variant (`Animation` > `Document`,
        // `Venue` > `Location`, `LivePhoto` > `Photo`).
        let kind = if let Some(v) = take_non_null(&mut map, "animation") {
            Some(parse_variant(v, ExternalReplyInfoKind::Animation))
        } else if let Some(v) = take_non_null(&mut map, "audio") {
            Some(parse_variant(v, ExternalReplyInfoKind::Audio))
        } else if let Some(v) = take_non_null(&mut map, "contact") {
            Some(parse_variant(v, ExternalReplyInfoKind::Contact))
        } else if let Some(v) = take_non_null(&mut map, "dice") {
            Some(parse_variant(v, ExternalReplyInfoKind::Dice))
        } else if let Some(v) = take_non_null(&mut map, "paid_media") {
            Some(parse_variant(v, ExternalReplyInfoKind::PaidMedia))
        } else if let Some(v) = take_non_null(&mut map, "game") {
            Some(parse_variant(v, ExternalReplyInfoKind::Game))
        } else if let Some(v) = take_non_null(&mut map, "venue") {
            Some(parse_variant(v, ExternalReplyInfoKind::Venue))
        } else if let Some(v) = take_non_null(&mut map, "location") {
            Some(parse_variant(v, ExternalReplyInfoKind::Location))
        } else if let Some(v) = take_non_null(&mut map, "live_photo") {
            Some(parse_variant(v, ExternalReplyInfoKind::LivePhoto))
        } else if let Some(v) = take_non_null(&mut map, "photo") {
            Some(parse_variant(v, ExternalReplyInfoKind::Photo))
        } else if let Some(v) = take_non_null(&mut map, "document") {
            Some(parse_variant(v, ExternalReplyInfoKind::Document))
        } else if let Some(v) = take_non_null(&mut map, "poll") {
            Some(parse_variant(v, ExternalReplyInfoKind::Poll))
        } else if let Some(v) = take_non_null(&mut map, "checklist") {
            Some(parse_variant(v, ExternalReplyInfoKind::Checklist))
        } else if let Some(v) = take_non_null(&mut map, "sticker") {
            Some(parse_variant(v, ExternalReplyInfoKind::Sticker))
        } else if let Some(v) = take_non_null(&mut map, "story") {
            Some(parse_variant(v, ExternalReplyInfoKind::Story))
        } else if let Some(v) = take_non_null(&mut map, "giveaway") {
            Some(parse_variant(v, ExternalReplyInfoKind::Giveaway))
        } else if let Some(v) = take_non_null(&mut map, "giveaway_winners") {
            Some(parse_variant(v, ExternalReplyInfoKind::GiveawayWinners))
        } else if let Some(v) = take_non_null(&mut map, "video") {
            Some(parse_variant(v, ExternalReplyInfoKind::Video))
        } else if let Some(v) = take_non_null(&mut map, "video_note") {
            Some(parse_variant(v, ExternalReplyInfoKind::VideoNote))
        } else if let Some(v) = take_non_null(&mut map, "voice") {
            Some(parse_variant(v, ExternalReplyInfoKind::Voice))
        } else {
            take_non_null(&mut map, "invoice")
                .map(|v| parse_variant(v, ExternalReplyInfoKind::Invoice))
        }
        .transpose()
        .map_err(serde::de::Error::custom)?;

        let header = ExternalReplyInfoHeader::deserialize(serde_json::Value::Object(map))
            .map_err(serde::de::Error::custom)?;

        Ok(Self {
            origin: header.origin,
            chat: header.chat,
            message_id: header.message_id,
            link_preview_options: header.link_preview_options,
            has_media_spoiler: header.has_media_spoiler,
            kind,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ExternalReplyInfoKind {
    // Note:
    // - `Venue` must be in front of `Location`
    // - `Animation` must be in front of `Document`
    // - `LivePhoto` must be in front of `Photo`
    //
    // This is needed so serde doesn't parse `Venue` as `Location`, `Animation` as `Document`,
    // or `LivePhoto` as `Photo` (for backward compatibility telegram duplicates some fields).
    //
    // See <https://github.com/teloxide/teloxide/issues/481>
    Animation(Animation),
    Audio(Audio),
    Contact(Contact),
    Dice(Dice),
    Document(Document),
    PaidMedia(PaidMediaInfo),
    Game(Game),
    Venue(Venue),
    Location(Location),
    LivePhoto(LivePhoto),
    Photo(Vec<PhotoSize>),
    Poll(Poll),
    Checklist(Checklist),
    Sticker(Sticker),
    Story(Story),
    Giveaway(Giveaway),
    GiveawayWinners(GiveawayWinners),
    Video(Video),
    VideoNote(VideoNote),
    Voice(Voice),
    Invoice(Invoice),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FileId, FileUniqueId};

    #[test]
    fn live_photo_takes_precedence_over_photo() {
        for json in [
            r#"{
                "origin": { "type": "user", "date": 1778200000, "sender_user": { "id": 1, "is_bot": false, "first_name": "A" } },
                "has_media_spoiler": true,
                "live_photo": {
                    "photo": [{ "file_id": "p1", "file_unique_id": "pu1", "width": 640, "height": 480 }],
                    "file_id": "lp1",
                    "file_unique_id": "lpu1",
                    "width": 640,
                    "height": 480,
                    "duration": 2
                },
                "photo": [{ "file_id": "p1", "file_unique_id": "pu1", "width": 640, "height": 480 }]
            }"#,
            r#"{
                "origin": { "type": "user", "date": 1778200000, "sender_user": { "id": 1, "is_bot": false, "first_name": "A" } },
                "has_media_spoiler": true,
                "photo": [{ "file_id": "p1", "file_unique_id": "pu1", "width": 640, "height": 480 }],
                "live_photo": {
                    "photo": [{ "file_id": "p1", "file_unique_id": "pu1", "width": 640, "height": 480 }],
                    "file_id": "lp1",
                    "file_unique_id": "lpu1",
                    "width": 640,
                    "height": 480,
                    "duration": 2
                }
            }"#,
        ] {
            let info: ExternalReplyInfo = serde_json::from_str(json).unwrap();
            match info.kind {
                Some(ExternalReplyInfoKind::LivePhoto(lp)) => {
                    assert_eq!(lp.file_id, FileId("lp1".to_owned()));
                    assert_eq!(lp.file_unique_id, FileUniqueId("lpu1".to_owned()));
                }
                other => panic!("expected LivePhoto variant, got {other:?}"),
            }
        }
    }

    #[test]
    fn overlapping_variants_ignore_json_key_order() {
        let doc_before_anim = r#"{
            "origin": { "type": "user", "date": 1778200000, "sender_user": { "id": 1, "is_bot": false, "first_name": "A" } },
            "document": { "file_id": "d1", "file_unique_id": "du1" },
            "animation": { "file_id": "a1", "file_unique_id": "au1", "width": 100, "height": 100, "duration": 1, "mime_type": "video/mp4" }
        }"#;
        let info: ExternalReplyInfo = serde_json::from_str(doc_before_anim).unwrap();
        assert!(matches!(info.kind, Some(ExternalReplyInfoKind::Animation(_))));

        let loc_before_venue = r#"{
            "origin": { "type": "user", "date": 1778200000, "sender_user": { "id": 1, "is_bot": false, "first_name": "A" } },
            "location": { "latitude": 1.0, "longitude": 2.0 },
            "venue": { "location": { "latitude": 1.0, "longitude": 2.0 }, "title": "T", "address": "A" }
        }"#;
        let info: ExternalReplyInfo = serde_json::from_str(loc_before_venue).unwrap();
        assert!(matches!(info.kind, Some(ExternalReplyInfoKind::Venue(_))));
    }

    #[test]
    fn malformed_lower_precedence_field_does_not_fail_primary_variant() {
        let json = r#"{
            "origin": { "type": "user", "date": 1778200000, "sender_user": { "id": 1, "is_bot": false, "first_name": "A" } },
            "live_photo": {
                "file_id": "lp1",
                "file_unique_id": "lpu1",
                "width": 640,
                "height": 480,
                "duration": 2
            },
            "photo": "unexpected-legacy-shape"
        }"#;
        let info: ExternalReplyInfo = serde_json::from_str(json).unwrap();
        assert!(matches!(info.kind, Some(ExternalReplyInfoKind::LivePhoto(_))));
    }
}
