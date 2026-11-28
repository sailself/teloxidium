use crate::types::{
    CallbackQuery, Chat, ChatId, ChatJoinRequest, ChatMemberUpdated, Message, Update, UpdateKind,
};

/// Something that may have a chat ID.
///
/// Guest queries (`UpdateKind::GuestMessage` and messages with
/// `guest_query_id`) return `None` to avoid colliding with chat-keyed
/// dialogues. Note that business messages (`UpdateKind::BusinessMessage` /
/// `UpdateKind::EditedBusinessMessage` and messages carrying
/// `business_connection_id`) currently still return `Some(chat.id)` for
/// backwards compatibility; dedicated business-connection scoping is deferred.
pub trait GetChatId {
    #[must_use]
    fn chat_id(&self) -> Option<ChatId>;
}

impl GetChatId for Message {
    fn chat_id(&self) -> Option<ChatId> {
        if self.guest_query_id.is_some() {
            return None;
        }
        Some(self.chat.id)
    }
}

impl GetChatId for CallbackQuery {
    fn chat_id(&self) -> Option<ChatId> {
        self.message.as_ref().map(|mes| mes.chat().id)
    }
}

impl GetChatId for Update {
    fn chat_id(&self) -> Option<ChatId> {
        if matches!(self.kind, UpdateKind::GuestMessage(_)) {
            return None;
        }
        self.chat().map(|chat| chat.id)
    }
}

impl GetChatId for Chat {
    fn chat_id(&self) -> Option<ChatId> {
        Some(self.id)
    }
}

impl GetChatId for ChatMemberUpdated {
    fn chat_id(&self) -> Option<ChatId> {
        Some(self.chat.id)
    }
}

impl GetChatId for ChatJoinRequest {
    fn chat_id(&self) -> Option<ChatId> {
        Some(self.chat.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guest_messages_are_excluded_from_dialogue_chat_id() {
        let regular_update: Update = serde_json::from_str(
            r#"{
                "update_id": 1,
                "message": {
                    "message_id": 10,
                    "chat": { "id": -100500, "title": "Real Chat", "type": "supergroup" },
                    "date": 1778200000,
                    "text": "hello"
                }
            }"#,
        )
        .unwrap();
        assert_eq!(regular_update.chat_id(), Some(ChatId(-100500)));

        let guest_update: Update = serde_json::from_str(
            r#"{
                "update_id": 2,
                "guest_message": {
                    "message_id": 11,
                    "from": { "id": 2001, "is_bot": false, "first_name": "Guest" },
                    "chat": { "id": -100500, "title": "Guest Room", "type": "supergroup" },
                    "date": 1778200001,
                    "guest_query_id": "gq-1",
                    "text": "@bot hi"
                }
            }"#,
        )
        .unwrap();
        assert_eq!(guest_update.chat_id(), None);
        if let UpdateKind::GuestMessage(msg) = &guest_update.kind {
            assert_eq!(msg.chat_id(), None);
        } else {
            panic!("expected GuestMessage");
        }
    }
}
