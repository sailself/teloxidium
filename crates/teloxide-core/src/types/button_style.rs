use serde::{Deserialize, Serialize};

/// Style of a [`KeyboardButton`] or [`InlineKeyboardButton`].
///
/// [`KeyboardButton`]: crate::types::KeyboardButton
/// [`InlineKeyboardButton`]: crate::types::InlineKeyboardButton
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ButtonStyle {
    /// Blue button style.
    Primary,
    /// Green button style.
    Success,
    /// Red button style.
    Danger,
    /// A style value unknown to this version of the library.
    #[serde(other)]
    #[cfg_attr(test, schemars(skip))]
    Unknown,
}

impl ButtonStyle {
    pub(crate) fn is_none_or_unknown(style: &Option<Self>) -> bool {
        matches!(style, None | Some(Self::Unknown))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        for (style, expected) in [
            (ButtonStyle::Primary, r#""primary""#),
            (ButtonStyle::Success, r#""success""#),
            (ButtonStyle::Danger, r#""danger""#),
        ] {
            let json = serde_json::to_string(&style).unwrap();
            assert_eq!(json, expected);
            let parsed: ButtonStyle = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, style);
        }
    }
}
