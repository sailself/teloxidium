use serde::{Deserialize, Serialize};

/// This object describes the types of gifts that can be gifted to a user or a
/// chat.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct AcceptedGiftTypes {
    /// `true`, if unlimited regular gifts are accepted
    pub unlimited_gifts: bool,

    /// `true`, if limited regular gifts are accepted
    pub limited_gifts: bool,

    /// `true`, if unique gifts or gifts that can be upgraded to unique for free
    /// are accepted
    pub unique_gifts: bool,

    /// `true`, if a Telegram Premium subscription is accepted
    pub premium_subscription: bool,

    /// `true`, if transfers of unique gifts from channels are accepted
    #[serde(default)]
    pub gifts_from_channels: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_and_legacy_fallback() {
        let legacy_json = r#"{
            "unlimited_gifts": true,
            "limited_gifts": false,
            "unique_gifts": true,
            "premium_subscription": true
        }"#;
        let legacy: AcceptedGiftTypes = serde_json::from_str(legacy_json).unwrap();
        assert!(!legacy.gifts_from_channels);

        let modern = AcceptedGiftTypes {
            unlimited_gifts: true,
            limited_gifts: false,
            unique_gifts: true,
            premium_subscription: false,
            gifts_from_channels: true,
        };
        let serialized = serde_json::to_string(&modern).unwrap();
        assert_eq!(
            serialized,
            r#"{"unlimited_gifts":true,"limited_gifts":false,"unique_gifts":true,"premium_subscription":false,"gifts_from_channels":true}"#
        );
        assert_eq!(serde_json::from_str::<AcceptedGiftTypes>(&serialized).unwrap(), modern);
    }
}
