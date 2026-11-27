use serde::{Deserialize, Serialize};

use crate::types::User;

/// Describes a service message about an ownership change in the chat.
///
/// [The official docs](https://core.telegram.org/bots/api#chatownerchanged).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatOwnerChanged {
    /// The new owner of the chat.
    pub new_owner: User,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::UserId;

    #[test]
    fn roundtrip() {
        let changed = ChatOwnerChanged {
            new_owner: User {
                id: UserId(456),
                is_bot: false,
                first_name: "Bob".to_owned(),
                last_name: Some("Builder".to_owned()),
                username: None,
                language_code: Some("en".to_owned()),
                is_premium: false,
                added_to_attachment_menu: false,
            },
        };
        let json = serde_json::to_string(&changed).unwrap();
        let parsed: ChatOwnerChanged = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, changed);
    }
}
