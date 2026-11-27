use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::types::{Chat, GiftBackground, Sticker};

/// This object represent a list of gifts.
///
/// [The official docs](https://core.telegram.org/bots/api#gifts).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Gifts {
    /// The list of gifts
    pub gifts: Vec<Gift>,
}

/// A unique identifier of the gift.
#[derive(Clone, Debug, derive_more::Display)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize, From)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(transparent)]
#[from(&'static str, String)]
pub struct GiftId(pub String);

/// This object represents a gift that can be sent by the bot.
///
/// [The official docs](https://core.telegram.org/bots/api#gift).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Gift {
    /// Unique identifier of the gift
    pub id: GiftId,

    /// The sticker that represents the gift
    pub sticker: Sticker,

    /// The number of Telegram Stars that must be paid to send the sticker
    pub star_count: u32,

    /// The number of Telegram Stars that must be paid to upgrade the gift to a
    /// unique one
    pub upgrade_star_count: Option<u32>,

    /// `true`, if the gift can only be purchased by Telegram Premium
    /// subscribers
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_premium: bool,

    /// `true`, if the gift can be used (after being upgraded) to customize a
    /// user's appearance
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_colors: bool,

    /// The total number of gifts of this type that can be sent by all users;
    /// for limited gifts only
    pub total_count: Option<u32>,

    /// The number of remaining gifts of this type that can be sent by all
    /// users; for limited gifts only
    pub remaining_count: Option<u32>,

    /// The total number of gifts of this type that can be sent by the bot; for
    /// limited gifts only
    pub personal_total_count: Option<u32>,

    /// The number of remaining gifts of this type that can be sent by the bot;
    /// for limited gifts only
    pub personal_remaining_count: Option<u32>,

    /// Background of the gift
    pub background: Option<GiftBackground>,

    /// The total number of different unique gifts that can be obtained by
    /// upgrading the gift
    pub unique_gift_variant_count: Option<u32>,

    /// Information about the chat that published the gift
    pub publisher_chat: Option<Chat>,
}

impl Gift {
    pub fn is_limited(&self) -> bool {
        self.total_count.is_some()
    }

    /// Returns [`None`] if Gift isn't limited or a tuple where first element is
    /// [`Self::remaining_count`] and second is [`Self::total_count`]
    pub fn limited_count(&self) -> Option<(u32, u32)> {
        match (self.remaining_count, self.total_count) {
            (Some(remaining_count), Some(total_count)) => Some((remaining_count, total_count)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deser() {
        let gift_id = S { gift_id: GiftId("id".to_owned()) };
        let json = r#"{"gift_id":"id"}"#;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct S {
            gift_id: GiftId,
        }

        assert_eq!(serde_json::to_string(&gift_id).unwrap(), json);
        assert_eq!(gift_id, serde_json::from_str(json).unwrap());
    }

    #[test]
    fn roundtrip_and_legacy_fallback() {
        use crate::types::Rgb;

        let legacy_json = r#"{
            "id": "gift-1",
            "sticker": {
                "file_id": "CAACAgIAAxUAAWMwcTidRlq7bai-xUkcHQLa6vgJAALZBwACwRieC1FFIeQlHsPdKQQ",
                "file_unique_id": "AgAD2QcAAsEYngs",
                "file_size": 25734,
                "width": 512,
                "height": 512,
                "type": "regular",
                "is_animated": true,
                "is_video": false,
                "needs_repainting": false
            },
            "star_count": 25
        }"#;
        let legacy: Gift = serde_json::from_str(legacy_json).unwrap();
        assert!(!legacy.is_premium);
        assert!(!legacy.has_colors);
        assert_eq!(legacy.personal_total_count, None);
        assert_eq!(legacy.personal_remaining_count, None);
        assert_eq!(legacy.background, None);
        assert_eq!(legacy.unique_gift_variant_count, None);

        let full_json = r#"{
            "id": "gift-2",
            "sticker": {
                "file_id": "CAACAgIAAxUAAWMwcTidRlq7bai-xUkcHQLa6vgJAALZBwACwRieC1FFIeQlHsPdKQQ",
                "file_unique_id": "AgAD2QcAAsEYngs",
                "file_size": 25734,
                "width": 512,
                "height": 512,
                "type": "regular",
                "is_animated": true,
                "is_video": false,
                "needs_repainting": false
            },
            "star_count": 50,
            "upgrade_star_count": 100,
            "is_premium": true,
            "has_colors": true,
            "total_count": 1000,
            "remaining_count": 250,
            "personal_total_count": 5,
            "personal_remaining_count": 3,
            "background": {
                "center_color": 1118481,
                "edge_color": 2236962,
                "text_color": 16777215
            },
            "unique_gift_variant_count": 42
        }"#;
        let full: Gift = serde_json::from_str(full_json).unwrap();
        assert!(full.is_premium);
        assert!(full.has_colors);
        assert_eq!(full.personal_total_count, Some(5));
        assert_eq!(full.personal_remaining_count, Some(3));
        assert_eq!(full.unique_gift_variant_count, Some(42));
        assert_eq!(
            full.background,
            Some(GiftBackground {
                center_color: Rgb::from_u32(1118481),
                edge_color: Rgb::from_u32(2236962),
                text_color: Rgb::from_u32(16777215),
            })
        );

        let roundtrip: Gift = serde_json::from_str(&serde_json::to_string(&full).unwrap()).unwrap();
        assert_eq!(full, roundtrip);
    }
}
