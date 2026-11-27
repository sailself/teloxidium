use serde::{Deserialize, Serialize};

use crate::types::{CustomEmojiId, Rgb};

/// This object contains information about the color scheme for a user's name,
/// message replies and link previews based on a unique gift.
///
/// [The official docs](https://core.telegram.org/bots/api#uniquegiftcolors).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UniqueGiftColors {
    /// Custom emoji identifier of the unique gift's model
    pub model_custom_emoji_id: CustomEmojiId,

    /// Custom emoji identifier of the unique gift's symbol
    pub symbol_custom_emoji_id: CustomEmojiId,

    /// Main color used in light themes; RGB format
    pub light_theme_main_color: Rgb,

    /// List of 1-3 additional colors used in light themes; RGB format
    pub light_theme_other_colors: Vec<Rgb>,

    /// Main color used in dark themes; RGB format
    pub dark_theme_main_color: Rgb,

    /// List of 1-3 additional colors used in dark themes; RGB format
    pub dark_theme_other_colors: Vec<Rgb>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let colors = UniqueGiftColors {
            model_custom_emoji_id: CustomEmojiId("5312536423851630001".to_owned()),
            symbol_custom_emoji_id: CustomEmojiId("5312536423851630002".to_owned()),
            light_theme_main_color: Rgb { r: 0x10, g: 0x20, b: 0x30 },
            light_theme_other_colors: vec![
                Rgb { r: 0x40, g: 0x50, b: 0x60 },
                Rgb { r: 0x70, g: 0x80, b: 0x90 },
            ],
            dark_theme_main_color: Rgb { r: 0xAA, g: 0xBB, b: 0xCC },
            dark_theme_other_colors: vec![Rgb { r: 0xDD, g: 0xEE, b: 0xFF }],
        };

        let json = serde_json::to_string(&colors).unwrap();
        assert_eq!(serde_json::from_str::<UniqueGiftColors>(&json).unwrap(), colors);
    }
}
