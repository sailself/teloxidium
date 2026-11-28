use derive_more::derive::From;
use serde::{Deserialize, Serialize};

/// Unique identifier of a guest mode query.
#[derive(
    Default,
    Clone,
    Debug,
    derive_more::Display,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    From
)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(transparent)]
#[from(&'static str, String)]
pub struct GuestQueryId(pub String);

/// Describes an inline message sent to a guest user in response to a
/// guest-mode query.
///
/// [The official docs](https://core.telegram.org/bots/api#sentguestmessage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct SentGuestMessage {
    /// Identifier of the sent inline message.
    pub inline_message_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let id = GuestQueryId::from("guest-q-1");
        let id_json = serde_json::to_string(&id).unwrap();
        assert_eq!(id_json, r#""guest-q-1""#);
        assert_eq!(serde_json::from_str::<GuestQueryId>(&id_json).unwrap(), id);

        let sent = SentGuestMessage { inline_message_id: "inline-msg-42".to_owned() };
        let sent_json = serde_json::to_string(&sent).unwrap();
        assert_eq!(sent_json, r#"{"inline_message_id":"inline-msg-42"}"#);
        assert_eq!(serde_json::from_str::<SentGuestMessage>(&sent_json).unwrap(), sent);
    }
}
