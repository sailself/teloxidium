use serde::{Deserialize, Serialize};

use crate::types::{CallbackQueryId, UserId};

/// Identifier of an ephemeral message within a chat.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct EphemeralMessageId(pub i32);

/// Parameters for sending an [ephemeral message](https://core.telegram.org/bots/api#ephemeralmessageparameters).
///
/// Sending with these parameters requires Bot API 10.3 or newer. Callback-only
/// replies require Bot API 10.4 or newer. Older self-hosted servers can ignore
/// these parameters and send a regular message.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(try_from = "RawEphemeralMessageParameters")]
pub struct EphemeralMessageParameters {
    receiver_user_id: Option<UserId>,
    callback_query_id: Option<CallbackQueryId>,
    /// Whether the reply temporarily replaces the message that contained the
    /// pressed button. Only valid when replying to a non-ephemeral message.
    pub replace_callback_query_message: Option<bool>,
}

impl EphemeralMessageParameters {
    /// Sends the message to a user in the specified chat.
    #[must_use]
    pub const fn for_user(receiver_user_id: UserId) -> Self {
        Self {
            receiver_user_id: Some(receiver_user_id),
            callback_query_id: None,
            replace_callback_query_message: None,
        }
    }

    /// Sends an ephemeral reply to the user who pressed the button.
    /// Telegram ignores the send request's chat ID in this mode.
    /// The callback query must still be acknowledged via
    /// `answer_callback_query`.
    #[must_use]
    pub fn for_callback(callback_query_id: CallbackQueryId) -> Self {
        Self {
            receiver_user_id: None,
            callback_query_id: Some(callback_query_id),
            replace_callback_query_message: None,
        }
    }

    /// Sets the recipient. For a callback reply, this must be the user who
    /// pressed the button. Telegram uses the send request's chat ID when a
    /// recipient is set.
    #[must_use]
    pub const fn with_receiver_user_id(mut self, receiver_user_id: UserId) -> Self {
        self.receiver_user_id = Some(receiver_user_id);
        self
    }

    /// Returns the explicitly targeted user, if set.
    #[must_use]
    pub const fn receiver_user_id(&self) -> Option<UserId> {
        self.receiver_user_id
    }

    /// Returns the callback query that triggered the reply, if set.
    #[must_use]
    pub fn callback_query_id(&self) -> Option<&CallbackQueryId> {
        self.callback_query_id.as_ref()
    }

    /// Shows the reply in place of the message that contained the button.
    #[must_use]
    pub const fn replace_callback_query_message(mut self) -> Self {
        self.replace_callback_query_message = Some(true);
        self
    }
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
struct RawEphemeralMessageParameters {
    receiver_user_id: Option<UserId>,
    callback_query_id: Option<CallbackQueryId>,
    replace_callback_query_message: Option<bool>,
}

impl TryFrom<RawEphemeralMessageParameters> for EphemeralMessageParameters {
    type Error = &'static str;

    fn try_from(raw: RawEphemeralMessageParameters) -> Result<Self, Self::Error> {
        if raw.receiver_user_id.is_none() && raw.callback_query_id.is_none() {
            return Err("ephemeral messages require receiver_user_id or callback_query_id");
        }

        Ok(Self {
            receiver_user_id: raw.receiver_user_id,
            callback_query_id: raw.callback_query_id,
            replace_callback_query_message: raw.replace_callback_query_message,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::{
        prelude::*,
        requests::{HasPayload, Payload},
        types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
    };

    #[test]
    fn ephemeral_menu_requests_use_the_bot_api_fields() {
        let bot = Bot::new("test-token");
        let chat = ChatId(-123);
        let receiver = UserId(5_000_000_000);
        let id = EphemeralMessageId(i32::MAX);
        let keyboard =
            InlineKeyboardMarkup::new([[InlineKeyboardButton::callback("Done", "done")]]);
        let send = bot
            .send_message(chat, "Settings")
            .ephemeral_message_parameters(EphemeralMessageParameters::for_user(receiver))
            .reply_markup(keyboard.clone());
        assert_eq!(
            serde_json::to_value(send.payload_ref()).unwrap(),
            json!({
                "chat_id": -123,
                "text": "Settings",
                "reply_markup": {"inline_keyboard": [[{"text": "Done", "callback_data": "done"}]]},
                "ephemeral_message_parameters": {"receiver_user_id": 5_000_000_000_u64}
            })
        );

        let overlay = bot.send_message(chat, "Settings").ephemeral_message_parameters(
            EphemeralMessageParameters::for_callback("12345".into())
                .replace_callback_query_message(),
        );
        assert_eq!(
            serde_json::to_value(overlay.payload_ref()).unwrap()["ephemeral_message_parameters"],
            json!({"callback_query_id": "12345", "replace_callback_query_message": true})
        );

        let edit =
            bot.edit_ephemeral_message_reply_markup(chat, receiver, id).reply_markup(keyboard);
        assert_eq!(
            crate::payloads::EditEphemeralMessageReplyMarkup::NAME,
            "EditEphemeralMessageReplyMarkup"
        );
        assert_eq!(
            serde_json::to_value(edit.payload_ref()).unwrap(),
            json!({
                "chat_id": -123, "receiver_user_id": 5_000_000_000_u64,
                "ephemeral_message_id": i32::MAX,
                "reply_markup": {"inline_keyboard": [[{"text": "Done", "callback_data": "done"}]]}
            })
        );

        let delete = bot.delete_ephemeral_message(chat, receiver, id);
        assert_eq!(crate::payloads::DeleteEphemeralMessage::NAME, "DeleteEphemeralMessage");
        assert_eq!(
            serde_json::to_value(delete.payload_ref()).unwrap(),
            json!({"chat_id": -123, "receiver_user_id": 5_000_000_000_u64, "ephemeral_message_id": i32::MAX})
        );
    }

    #[test]
    fn ephemeral_parameters_require_a_target() {
        for raw in [json!({}), json!({"replace_callback_query_message": true})] {
            assert!(serde_json::from_value::<EphemeralMessageParameters>(raw).is_err());
        }

        let params = EphemeralMessageParameters::for_callback("12345".into())
            .with_receiver_user_id(UserId(5_000_000_000));
        let raw = json!({"callback_query_id": "12345", "receiver_user_id": 5_000_000_000_u64});
        assert_eq!(serde_json::to_value(&params).unwrap(), raw);
        assert_eq!(serde_json::from_value::<EphemeralMessageParameters>(raw).unwrap(), params);
        assert!(serde_json::from_value::<EphemeralMessageId>(json!(5_000_000_000_u64)).is_err());
    }

    #[test]
    fn ephemeral_message_keeps_its_id_and_recipient() {
        let mut raw = json!({
            "message_id": 0, "date": 1700000000, "text": "Settings",
            "chat": {"id": -123, "type": "group", "title": "Test"},
            "ephemeral_message_id": i32::MAX,
            "receiver_user": {"id": 5_000_000_000_u64, "is_bot": false, "first_name": "User"}
        });
        let message: Message = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(message.id.0, 0);
        assert_eq!(message.ephemeral_message_id, Some(EphemeralMessageId(i32::MAX)));
        assert_eq!(message.receiver_user().unwrap().id, UserId(5_000_000_000));
        assert_eq!(serde_json::to_value(message).unwrap(), raw);

        raw.as_object_mut().unwrap().remove("ephemeral_message_id");
        raw.as_object_mut().unwrap().remove("receiver_user");
        raw["message_id"] = json!(1);
        let ordinary: Message = serde_json::from_value(raw).unwrap();
        assert!(ordinary.ephemeral_message_id.is_none());
        assert!(ordinary.receiver_user.is_none());
    }
}
