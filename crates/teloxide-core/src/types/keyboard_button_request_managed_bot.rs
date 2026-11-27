use serde::{Deserialize, Serialize};

use crate::types::RequestId;

/// This object defines the parameters for the creation of a managed bot.
///
/// Information about the created bot will be shared with the bot using the
/// update `managed_bot` and a [`Message`] with the field `managed_bot_created`.
///
/// [The official docs](https://core.telegram.org/bots/api#keyboardbuttonrequestmanagedbot).
///
/// [`Message`]: crate::types::Message
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct KeyboardButtonRequestManagedBot {
    /// Signed 32-bit identifier of the request. Must be unique within the
    /// message.
    pub request_id: RequestId,

    /// Suggested name for the bot.
    pub suggested_name: Option<String>,

    /// Suggested username for the bot. The username must end with `"bot"`.
    pub suggested_username: Option<String>,
}

impl KeyboardButtonRequestManagedBot {
    /// Creates a new [`KeyboardButtonRequestManagedBot`].
    #[must_use]
    pub fn new(request_id: RequestId) -> Self {
        Self { request_id, suggested_name: None, suggested_username: None }
    }

    /// Setter for `suggested_name` field.
    #[must_use]
    pub fn suggested_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.suggested_name = Some(value.into());
        self
    }

    /// Setter for `suggested_username` field.
    #[must_use]
    pub fn suggested_username<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.suggested_username = Some(value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_minimal_and_full() {
        let minimal = KeyboardButtonRequestManagedBot::new(RequestId(42));
        let minimal_json = serde_json::to_string(&minimal).unwrap();
        assert_eq!(minimal_json, r#"{"request_id":42}"#);
        let parsed_minimal: KeyboardButtonRequestManagedBot =
            serde_json::from_str(&minimal_json).unwrap();
        assert_eq!(parsed_minimal, minimal);

        let full = KeyboardButtonRequestManagedBot::new(RequestId(-7))
            .suggested_name("Helper Bot")
            .suggested_username("my_helper_bot");
        let full_json = serde_json::to_string(&full).unwrap();
        assert_eq!(
            full_json,
            r#"{"request_id":-7,"suggested_name":"Helper Bot","suggested_username":"my_helper_bot"}"#
        );
        let parsed_full: KeyboardButtonRequestManagedBot =
            serde_json::from_str(&full_json).unwrap();
        assert_eq!(parsed_full, full);
    }
}
