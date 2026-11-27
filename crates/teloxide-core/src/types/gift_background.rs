use serde::{Deserialize, Serialize};

use crate::types::Rgb;

/// This object describes the background of a gift.
///
/// [The official docs](https://core.telegram.org/bots/api#giftbackground).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct GiftBackground {
    /// Center color of the background in RGB format
    pub center_color: Rgb,

    /// Edge color of the background in RGB format
    pub edge_color: Rgb,

    /// Text color of the background in RGB format
    pub text_color: Rgb,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let background = GiftBackground {
            center_color: Rgb { r: 0x11, g: 0x22, b: 0x33 },
            edge_color: Rgb { r: 0x44, g: 0x55, b: 0x66 },
            text_color: Rgb { r: 0xFF, g: 0xFF, b: 0xFF },
        };
        let json = serde_json::to_string(&background).unwrap();
        assert_eq!(json, r#"{"center_color":1122867,"edge_color":4478310,"text_color":16777215}"#);
        assert_eq!(serde_json::from_str::<GiftBackground>(&json).unwrap(), background);
    }
}
