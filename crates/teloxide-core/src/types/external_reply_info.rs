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
struct ExternalReplyInfoRaw {
    origin: MessageOrigin,
    chat: Option<Chat>,
    #[serde(default, with = "crate::types::option_msg_id_as_int")]
    message_id: Option<MessageId>,
    link_preview_options: Option<LinkPreviewOptions>,
    #[serde(default)]
    has_media_spoiler: bool,
    animation: Option<Animation>,
    audio: Option<Audio>,
    contact: Option<Contact>,
    dice: Option<Dice>,
    document: Option<Document>,
    paid_media: Option<PaidMediaInfo>,
    game: Option<Game>,
    venue: Option<Venue>,
    location: Option<Location>,
    live_photo: Option<LivePhoto>,
    photo: Option<Vec<PhotoSize>>,
    poll: Option<Poll>,
    checklist: Option<Checklist>,
    sticker: Option<Sticker>,
    story: Option<Story>,
    giveaway: Option<Giveaway>,
    giveaway_winners: Option<GiveawayWinners>,
    video: Option<Video>,
    video_note: Option<VideoNote>,
    voice: Option<Voice>,
    invoice: Option<Invoice>,
}

impl<'de> Deserialize<'de> for ExternalReplyInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = ExternalReplyInfoRaw::deserialize(deserializer)?;
        // Check overlapping wrapper keys in explicit precedence order regardless of
        // JSON key order (`Animation` > `Document`, `Venue` > `Location`, `LivePhoto` >
        // `Photo`).
        let kind = if let Some(animation) = raw.animation {
            Some(ExternalReplyInfoKind::Animation(animation))
        } else if let Some(audio) = raw.audio {
            Some(ExternalReplyInfoKind::Audio(audio))
        } else if let Some(contact) = raw.contact {
            Some(ExternalReplyInfoKind::Contact(contact))
        } else if let Some(dice) = raw.dice {
            Some(ExternalReplyInfoKind::Dice(dice))
        } else if let Some(paid_media) = raw.paid_media {
            Some(ExternalReplyInfoKind::PaidMedia(paid_media))
        } else if let Some(game) = raw.game {
            Some(ExternalReplyInfoKind::Game(game))
        } else if let Some(venue) = raw.venue {
            Some(ExternalReplyInfoKind::Venue(venue))
        } else if let Some(location) = raw.location {
            Some(ExternalReplyInfoKind::Location(location))
        } else if let Some(live_photo) = raw.live_photo {
            Some(ExternalReplyInfoKind::LivePhoto(live_photo))
        } else if let Some(photo) = raw.photo {
            Some(ExternalReplyInfoKind::Photo(photo))
        } else if let Some(document) = raw.document {
            Some(ExternalReplyInfoKind::Document(document))
        } else if let Some(poll) = raw.poll {
            Some(ExternalReplyInfoKind::Poll(poll))
        } else if let Some(checklist) = raw.checklist {
            Some(ExternalReplyInfoKind::Checklist(checklist))
        } else if let Some(sticker) = raw.sticker {
            Some(ExternalReplyInfoKind::Sticker(sticker))
        } else if let Some(story) = raw.story {
            Some(ExternalReplyInfoKind::Story(story))
        } else if let Some(giveaway) = raw.giveaway {
            Some(ExternalReplyInfoKind::Giveaway(giveaway))
        } else if let Some(giveaway_winners) = raw.giveaway_winners {
            Some(ExternalReplyInfoKind::GiveawayWinners(giveaway_winners))
        } else if let Some(video) = raw.video {
            Some(ExternalReplyInfoKind::Video(video))
        } else if let Some(video_note) = raw.video_note {
            Some(ExternalReplyInfoKind::VideoNote(video_note))
        } else if let Some(voice) = raw.voice {
            Some(ExternalReplyInfoKind::Voice(voice))
        } else {
            raw.invoice.map(ExternalReplyInfoKind::Invoice)
        };

        Ok(Self {
            origin: raw.origin,
            chat: raw.chat,
            message_id: raw.message_id,
            link_preview_options: raw.link_preview_options,
            has_media_spoiler: raw.has_media_spoiler,
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
}
