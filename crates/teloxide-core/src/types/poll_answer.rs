use serde::{Deserialize, Deserializer, Serialize};

use crate::types::{Chat, MaybeAnonymousUser, PollId, User};

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollAnswer {
    /// Unique poll identifier.
    pub poll_id: PollId,

    /// If the voter is anonymous, stores the chat that changed the answer to
    /// the poll.
    ///
    /// If the voter isn't anonymous, stores the user that changed
    /// the answer to the poll
    #[serde(deserialize_with = "deserialize_voter", flatten)]
    pub voter: MaybeAnonymousUser,

    /// 0-based identifiers of answer options, chosen by the user.
    ///
    /// May be empty if the user retracted their vote.
    pub option_ids: Vec<u8>,

    /// Persistent identifiers of the chosen answer options.
    ///
    /// May be empty if the vote was retracted.
    #[serde(default)]
    pub option_persistent_ids: Vec<String>,
}

/// These fields `chat` and `user` from the original [`PollAnswer`] should be
/// exclusive, but in cases when the `voter_chat` is presented the `user` isn't
/// `None`, but rather actual value for backward compatibility, the field `user`
/// in such objects will contain the user 136817688 (@Channel_Bot).
#[derive(Deserialize)]
struct VoterDe {
    /// The chat that changed the answer to the poll, if the voter is anonymous
    pub voter_chat: Option<Chat>,

    /// The user that changed the answer to the poll, if the voter isn't
    /// anonymous
    pub user: Option<User>,
}

fn deserialize_voter<'d, D: Deserializer<'d>>(d: D) -> Result<MaybeAnonymousUser, D::Error> {
    let VoterDe { voter_chat, user } = VoterDe::deserialize(d)?;
    voter_chat
        .map(MaybeAnonymousUser::Chat)
        .or_else(|| user.map(MaybeAnonymousUser::User))
        .ok_or_else(|| serde::de::Error::missing_field("user or voter_chat"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poll_answer_with_user_de() {
        let json = r#"{
            "poll_id": "POLL_ID",
            "user": {"id": 42,"is_bot": false,"first_name": "blah"},
            "option_ids": []
        }"#;

        let poll_answer: PollAnswer = serde_json::from_str(json).unwrap();

        assert!(poll_answer.voter.is_user());
    }

    #[test]
    fn poll_answer_with_voter_chat_de() {
        let json = r#"{
            "poll_id": "POLL_ID",
            "voter_chat": {
                "id": -1001160242915,
                "title": "a",
                "type": "group"
            },
            "option_ids": []
        }"#;

        let poll_answer: PollAnswer = serde_json::from_str(json).unwrap();
        assert!(poll_answer.voter.is_chat());
    }

    #[test]
    fn poll_answer_with_both_user_and_voter_chat_de() {
        let json = r#"{
            "poll_id":"POLL_ID",
            "voter_chat": {
                "id": -1001160242915,
                "title": "a",
                "type": "group"
            },
            "user": {"id": 136817688,"is_bot": true,"first_name": "Channel_Bot"},
            "option_ids": []
        }"#;

        let poll_answer: PollAnswer = serde_json::from_str(json).unwrap();
        assert!(poll_answer.voter.is_chat());
    }

    #[test]
    fn poll_answer_option_persistent_ids_and_legacy_fallback() {
        let legacy_json = r#"{
            "poll_id": "POLL_ID",
            "user": {"id": 42, "is_bot": false, "first_name": "Alice"},
            "option_ids": [0]
        }"#;
        let legacy: PollAnswer = serde_json::from_str(legacy_json).unwrap();
        assert!(legacy.option_persistent_ids.is_empty());

        let modern_json = r#"{
            "poll_id": "POLL_ID",
            "user": {"id": 42, "is_bot": false, "first_name": "Alice"},
            "option_ids": [0, 2],
            "option_persistent_ids": ["pid-0", "pid-2"]
        }"#;
        let modern: PollAnswer = serde_json::from_str(modern_json).unwrap();
        assert_eq!(modern.option_ids, vec![0, 2]);
        assert_eq!(modern.option_persistent_ids, vec!["pid-0", "pid-2"]);

        let serialized = serde_json::to_value(&modern).unwrap();
        assert_eq!(serialized["option_ids"], serde_json::json!([0, 2]));
        assert_eq!(serialized["option_persistent_ids"], serde_json::json!(["pid-0", "pid-2"]));
    }

    #[test]
    fn poll_answer_missing_voter_returns_error_not_panic() {
        let missing_voter = r#"{"poll_id":"POLL_ID","option_ids":[0]}"#;
        let err = serde_json::from_str::<PollAnswer>(missing_voter).unwrap_err();
        assert!(err.to_string().contains("user or voter_chat"));
    }
}
