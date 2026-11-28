use crate::types::{OwnedGiftId, UniqueGift};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Describes a service message about a unique gift that was sent or received.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UniqueGiftInfo {
    /// Information about the gift
    pub gift: UniqueGift,

    /// Origin of the gift. Currently, either `Upgrade` for gifts upgraded from
    /// regular gifts, `Transfer` for gifts transferred from other users or
    /// channels, `Resale` for gifts bought from other users, `GiftedUpgrade`
    /// for upgrades purchased after the gift was sent, or `Offer` for gifts
    /// bought or sold through gift purchase offers
    pub origin: UniqueGiftOrigin,

    /// For gifts bought from other users, the currency in which the payment for
    /// the gift was done. Currently, one of `"XTR"` for Telegram Stars or
    /// `"TON"` for TON grams
    pub last_resale_currency: Option<String>,

    /// For gifts bought from other users, the price paid for the gift in either
    /// Telegram Stars or nanograms
    pub last_resale_amount: Option<u64>,

    /// Unique identifier of the received gift for the bot; only present for
    /// gifts received on behalf of business accounts
    pub owned_gift_id: Option<OwnedGiftId>,

    /// Number of Telegram Stars that must be paid to transfer the gift; omitted
    /// if the bot cannot transfer the gift
    pub transfer_star_count: Option<u32>,

    /// Point in time when the gift can be transferred. If it is in the past,
    /// then the gift can be transferred now
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "Option<i64>"))]
    pub next_transfer_date: Option<DateTime<Utc>>,
}

/// Origin of the gift. Currently, either `Upgrade` for gifts upgraded from
/// regular gifts, `Transfer` for gifts transferred from other users or
/// channels, `Resale` for gifts bought from other users, `GiftedUpgrade` for
/// upgrades purchased after the gift was sent, or `Offer` for gifts bought or
/// sold through gift purchase offers.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum UniqueGiftOrigin {
    Upgrade,
    Transfer,
    Resale,
    GiftedUpgrade,
    Offer,
    /// An origin unknown to this version of the library.
    #[serde(other)]
    #[cfg_attr(test, schemars(skip))]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"{
            "gift": {
                "gift_id": "gift-1",
                "base_name": "name",
                "name": "name",
                "number": 123,
                "model": {
                    "name": "name",
                    "sticker": {
                        "file_id": "CAACAgIAAxUAAWMwcTidRlq7bai-xUkcHQLa6vgJAALZBwACwRieC1FFIeQlHsPdKQQ",
                        "file_unique_id": "AgAD2QcAAsEYngs",
                        "file_size": 25734,
                        "width": 463,
                        "height": 512,
                        "type": "regular",
                        "premium_animation": null,
                        "is_animated": false,
                        "is_video": false,
                        "needs_repainting": false
                    },
                    "rarity_per_mille": 123
                },
                "symbol": {
                    "name": "name",
                    "sticker": {
                        "file_id": "CAACAgIAAxUAAWMwcTidRlq7bai-xUkcHQLa6vgJAALZBwACwRieC1FFIeQlHsPdKQQ",
                        "file_unique_id": "AgAD2QcAAsEYngs",
                        "file_size": 25734,
                        "width": 463,
                        "height": 512,
                        "type": "regular",
                        "premium_animation": null,
                        "is_animated": false,
                        "is_video": false,
                        "needs_repainting": false
                    },
                    "rarity_per_mille": 123
                },
                "backdrop": {
                    "name": "name",
                    "colors": {
                        "center_color": 0,
                        "edge_color": 0,
                        "symbol_color": 0,
                        "text_color": 0
                    },
                    "rarity_per_mille": 123
                }
            },
            "origin": "upgrade"
        }"#;

        let unique_gift_info: UniqueGiftInfo = serde_json::from_str(data).unwrap();
        assert_eq!(unique_gift_info.origin, UniqueGiftOrigin::Upgrade);
        assert_eq!(unique_gift_info.gift.name, "name");
        assert_eq!(unique_gift_info.gift.backdrop.rarity_per_mille, 123);
        assert_eq!(unique_gift_info.last_resale_currency, None);
        assert_eq!(unique_gift_info.last_resale_amount, None);

        let offer_ton = data.replace(
            r#""origin": "upgrade""#,
            r#""origin": "offer", "last_resale_currency": "TON", "last_resale_amount": 5000000000"#,
        );
        let parsed_offer: UniqueGiftInfo = serde_json::from_str(&offer_ton).unwrap();
        assert_eq!(parsed_offer.origin, UniqueGiftOrigin::Offer);
        assert_eq!(parsed_offer.last_resale_currency.as_deref(), Some("TON"));
        assert_eq!(parsed_offer.last_resale_amount, Some(5_000_000_000));

        let gifted_upgrade =
            data.replace(r#""origin": "upgrade""#, r#""origin": "gifted_upgrade""#);
        let parsed_gifted: UniqueGiftInfo = serde_json::from_str(&gifted_upgrade).unwrap();
        assert_eq!(parsed_gifted.origin, UniqueGiftOrigin::GiftedUpgrade);
    }
}
