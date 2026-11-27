use serde::{Deserialize, Serialize};

use crate::types::User;

/// Describes a service message about the chat owner leaving the chat.
///
/// [The official docs](https://core.telegram.org/bots/api#chatownerleft).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatOwnerLeft {
    /// The user who will become the new owner of the chat if the previous owner
    /// does not return to the chat.
    pub new_owner: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::UserId;

    #[test]
    fn roundtrip() {
        let with_new_owner = ChatOwnerLeft {
            new_owner: Some(User {
                id: UserId(123),
                is_bot: false,
                first_name: "Alice".to_owned(),
                last_name: None,
                username: Some("alice".to_owned()),
                language_code: None,
                is_premium: false,
                added_to_attachment_menu: false,
            }),
        };
        let json = serde_json::to_string(&with_new_owner).unwrap();
        assert_eq!(
            json,
            r#"{"new_owner":{"id":123,"is_bot":false,"first_name":"Alice","username":"alice"}}"#
        );
        let parsed: ChatOwnerLeft = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, with_new_owner);

        let empty_json = "{}";
        let parsed_empty: ChatOwnerLeft = serde_json::from_str(empty_json).unwrap();
        assert_eq!(parsed_empty, ChatOwnerLeft { new_owner: None });
        assert_eq!(serde_json::to_string(&parsed_empty).unwrap(), empty_json);
    }
}
