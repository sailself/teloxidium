use serde::{Deserialize, Serialize};

use crate::types::User;

/// Contains information about users that have access to a managed bot.
///
/// [The official docs](https://core.telegram.org/bots/api#botaccesssettings).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BotAccessSettings {
    /// `true`, if only selected users can access the bot. The bot's owner can
    /// always access it.
    pub is_access_restricted: bool,

    /// The list of other users who have access to the bot if the access is
    /// restricted.
    pub added_users: Option<Vec<User>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::UserId;

    #[test]
    fn roundtrip() {
        let json = r#"{
            "is_access_restricted": true,
            "added_users": [
                {
                    "id": 77,
                    "is_bot": false,
                    "first_name": "Alice"
                }
            ]
        }"#;
        let parsed: BotAccessSettings = serde_json::from_str(json).unwrap();
        assert!(parsed.is_access_restricted);
        assert_eq!(parsed.added_users.as_ref().unwrap()[0].id, UserId(77));

        let roundtrip: BotAccessSettings =
            serde_json::from_str(&serde_json::to_string(&parsed).unwrap()).unwrap();
        assert_eq!(parsed, roundtrip);

        let unrestricted_json = r#"{"is_access_restricted":false}"#;
        let unrestricted: BotAccessSettings = serde_json::from_str(unrestricted_json).unwrap();
        assert!(!unrestricted.is_access_restricted);
        assert_eq!(unrestricted.added_users, None);
    }
}
