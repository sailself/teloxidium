use serde::{Deserialize, Serialize};

use crate::types::{InputPollOptionMedia, MessageEntity, ParseMode};

/// This object contains information about one answer option in a poll to send.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpolloption).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InputPollOption {
    /// Option text, 1-100 characters.
    pub text: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub formatting: Option<InputPollOptionFormatting>,

    /// Media added to the poll option.
    pub media: Option<InputPollOptionMedia>,
}

#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum InputPollOptionFormatting {
    /// Mode for parsing entities in the text. See [formatting options] for more
    /// details. Currently, only custom emoji entities are allowed.
    ///
    /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
    TextParseMode(ParseMode),

    /// A JSON-serialized list of special entities that appear in the poll
    /// option text. It can be specified instead of _text\_parse\_mode_.
    TextEntities(Vec<MessageEntity>),
}

impl InputPollOption {
    pub fn new<S>(text: S) -> Self
    where
        S: Into<String>,
    {
        Self { text: text.into(), formatting: None, media: None }
    }

    pub fn text_parse_mode(self, text_parse_mode: ParseMode) -> Self {
        Self { formatting: Some(InputPollOptionFormatting::TextParseMode(text_parse_mode)), ..self }
    }

    pub fn text_entities(self, text_entities: Vec<MessageEntity>) -> Self {
        Self { formatting: Some(InputPollOptionFormatting::TextEntities(text_entities)), ..self }
    }

    #[must_use]
    pub fn media<M>(self, media: M) -> Self
    where
        M: Into<InputPollOptionMedia>,
    {
        Self { media: Some(media.into()), ..self }
    }
}

impl From<String> for InputPollOption {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for InputPollOption {
    fn from(value: &str) -> Self {
        Self::new(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{CustomEmojiId, MessageEntityKind};

    use super::*;

    #[test]
    fn serialize_text_parse_mode() {
        let expected = r#"{"text":"Yay","text_parse_mode":"MarkdownV2"}"#;
        let actual = serde_json::to_string(
            &InputPollOption::new("Yay").text_parse_mode(ParseMode::MarkdownV2),
        )
        .unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn serialize_text_entities() {
        let expected = r#"{"text":"Yay🐧","text_entities":[{"type":"custom_emoji","custom_emoji_id":"5852631516261125005","offset":3,"length":2}]}"#;
        let actual = serde_json::to_string(&InputPollOption::new("Yay🐧").text_entities(vec![
            MessageEntity::new(
                MessageEntityKind::CustomEmoji {
                    custom_emoji_id: CustomEmojiId("5852631516261125005".to_owned()),
                },
                3,
                2,
            ),
        ]))
        .unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn serialize_media() {
        use crate::types::{InputFile, InputMediaSticker};

        let opt = InputPollOption::new("Sticker option")
            .media(InputMediaSticker::new(InputFile::file_id("stk-1".into())).emoji("🔥"));
        let json = serde_json::to_string(&opt).unwrap();
        assert_eq!(
            json,
            r#"{"text":"Sticker option","media":{"type":"sticker","media":"stk-1","emoji":"🔥"}}"#
        );
        let opt_pm = InputPollOptionFormatting::TextParseMode(ParseMode::MarkdownV2);
        let pm_json = serde_json::to_string(&opt_pm).unwrap();
        assert_eq!(serde_json::from_str::<InputPollOptionFormatting>(&pm_json).unwrap(), opt_pm);
    }
}
