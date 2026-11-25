use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::MessageEntity;

/// This object contains information about the quoted part of a message that is
/// replied to by the given message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct TextQuote {
    /// Text of the quoted part of a message that is replied to by the given
    /// message
    pub text: String,
    /// Special entities that appear in the quote. Currently, only _bold_,
    /// _italic_, _underline_, _strikethrough_, _spoiler_, and
    /// _custom_emoji_ entities are kept in quotes.
    #[serde(default)]
    pub entities: Vec<MessageEntity>,
    /// The original rich text fragments if the quote was selected from a
    /// rich message. In this case, [`Self::text`] may be empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    // The baseline Bot API schema predates rich quotes. The wire fixture
    // covers the raw fragments.
    #[cfg_attr(test, schemars(skip))]
    pub rich_texts: Option<Vec<Value>>,
    /// Approximate quote position in the original message in UTF-16 code units
    /// as specified by the sender
    pub position: u32,
    /// True, if the quote was chosen manually by the message sender. Otherwise,
    /// the quote was added automatically by the server.
    #[serde(default)]
    pub is_manual: bool,
}
