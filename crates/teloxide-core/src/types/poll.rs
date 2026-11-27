use crate::types::{Chat, MessageEntity, PollType, Seconds, User};

use chrono::{DateTime, Utc};
use derive_more::derive::From;
use serde::{Deserialize, Serialize};

/// Unique poll identifier.
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
pub struct PollId(pub String);

/// This object contains information about a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#poll).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Poll {
    /// Unique poll identifier.
    pub id: PollId,

    /// Poll question, 1-300 characters.
    pub question: String,

    /// Special entities that appear in the question. Currently, only custom
    /// emoji entities are allowed in poll questions
    pub question_entities: Option<Vec<MessageEntity>>,

    /// List of poll options.
    pub options: Vec<PollOption>,

    /// `true`, if the poll is closed.
    pub is_closed: bool,

    /// Total number of users that voted in the poll
    pub total_voter_count: u32,

    /// True, if the poll is anonymous
    pub is_anonymous: bool,

    /// Poll type, currently can be “regular” or “quiz”
    #[serde(rename = "type")]
    pub poll_type: PollType,

    /// True, if the poll allows multiple answers
    pub allows_multiple_answers: bool,

    /// `true`, if the poll allows changing the chosen answer options.
    #[serde(default)]
    pub allows_revoting: bool,

    /// Array of 0-based identifiers of the correct answer options. Available
    /// only for polls in quiz mode which are closed or were sent (not
    /// forwarded) by the bot or to the private chat with the bot.
    pub correct_option_ids: Option<Vec<u8>>,

    /// Text that is shown when a user chooses an incorrect answer or taps on
    /// the lamp icon in a quiz-style poll, 0-200 characters.
    pub explanation: Option<String>,

    /// Special entities like usernames, URLs, bot commands, etc. that appear in
    /// the explanation.
    pub explanation_entities: Option<Vec<MessageEntity>>,

    /// Amount of time in seconds the poll will be active after creation.
    pub open_period: Option<Seconds>,

    /// Point in time when the poll will be automatically closed.
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "Option<i64>"))]
    pub close_date: Option<DateTime<Utc>>,

    /// Description of the poll; for polls inside the [`Message`] object only.
    ///
    /// [`Message`]: crate::types::Message
    pub description: Option<String>,

    /// Special entities like usernames, URLs, bot commands, etc. that appear in
    /// the description.
    pub description_entities: Option<Vec<MessageEntity>>,
}

#[derive(Deserialize)]
struct PollDe {
    id: PollId,
    question: String,
    question_entities: Option<Vec<MessageEntity>>,
    options: Vec<PollOption>,
    is_closed: bool,
    total_voter_count: u32,
    is_anonymous: bool,
    #[serde(rename = "type")]
    poll_type: PollType,
    allows_multiple_answers: bool,
    #[serde(default)]
    allows_revoting: Option<bool>,
    correct_option_id: Option<u8>,
    correct_option_ids: Option<Vec<u8>>,
    explanation: Option<String>,
    explanation_entities: Option<Vec<MessageEntity>>,
    open_period: Option<Seconds>,
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    close_date: Option<DateTime<Utc>>,
    description: Option<String>,
    description_entities: Option<Vec<MessageEntity>>,
}

impl<'de> Deserialize<'de> for Poll {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let de = PollDe::deserialize(deserializer)?;
        let allows_revoting =
            de.allows_revoting.unwrap_or_else(|| matches!(de.poll_type, PollType::Regular));
        let correct_option_ids =
            de.correct_option_ids.or_else(|| de.correct_option_id.map(|id| vec![id]));

        Ok(Self {
            id: de.id,
            question: de.question,
            question_entities: de.question_entities,
            options: de.options,
            is_closed: de.is_closed,
            total_voter_count: de.total_voter_count,
            is_anonymous: de.is_anonymous,
            poll_type: de.poll_type,
            allows_multiple_answers: de.allows_multiple_answers,
            allows_revoting,
            correct_option_ids,
            explanation: de.explanation,
            explanation_entities: de.explanation_entities,
            open_period: de.open_period,
            close_date: de.close_date,
            description: de.description,
            description_entities: de.description_entities,
        })
    }
}

/// This object contains information about one answer option in a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#polloption).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollOption {
    /// Unique identifier of the option, persistent on option addition and
    /// deletion.
    #[serde(default)]
    pub persistent_id: String,

    /// Option text, 1-100 characters.
    pub text: String,

    /// Special entities that appear in the option text. Currently, only custom
    /// emoji entities are allowed in poll option texts
    pub text_entities: Option<Vec<MessageEntity>>,

    /// Number of users that voted for this option.
    pub voter_count: u32,

    /// User who added the option; omitted if the option wasn't added by a user
    /// after poll creation.
    pub added_by_user: Option<User>,

    /// Chat that added the option; omitted if the option wasn't added by a chat
    /// after poll creation.
    pub added_by_chat: Option<Chat>,

    /// Point in time when the option was added; omitted if the option existed
    /// in the original poll.
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "Option<i64>"))]
    pub addition_date: Option<DateTime<Utc>>,
}

impl Poll {
    /// Returns all users that are "contained" in this `Poll`
    /// structure.
    ///
    /// This might be useful to track information about users.
    ///
    /// Note that this function can return duplicate users.
    pub fn mentioned_users(&self) -> impl Iterator<Item = &User> {
        use crate::util::{flatten, mentioned_users_from_entities};

        flatten(self.explanation_entities.as_deref().map(mentioned_users_from_entities))
            .chain(flatten(self.description_entities.as_deref().map(mentioned_users_from_entities)))
            .chain(self.options.iter().filter_map(|opt| opt.added_by_user.as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
        {
            "allows_multiple_answers": false,
            "id": "5377643193141559299",
            "is_anonymous": true,
            "is_closed": false,
            "options": [
                {
                    "text": "1",
                    "voter_count": 1
                },
                {
                    "text": "2",
                    "voter_count": 0
                },
                {
                    "text": "3",
                    "voter_count": 0
                },
                {
                    "text": "4",
                    "voter_count": 0
                },
                {
                    "text": "5",
                    "voter_count": 0
                }
            ],
            "question": "Rate me from 1 to 5.",
            "total_voter_count": 1,
            "type": "regular"
        }
        "#;
        serde_json::from_str::<Poll>(data).unwrap();
    }

    #[test]
    fn bot_api_9_6_poll_roundtrip() {
        use crate::types::UserId;

        let data = r#"{
            "id": "poll-9-6",
            "question": "Pick all valid choices",
            "options": [
                {
                    "persistent_id": "opt-a",
                    "text": "Option A",
                    "voter_count": 3
                },
                {
                    "persistent_id": "opt-b",
                    "text": "Option B",
                    "voter_count": 1,
                    "added_by_user": {
                        "id": 701,
                        "is_bot": false,
                        "first_name": "Alice"
                    },
                    "addition_date": 1775174400
                },
                {
                    "persistent_id": "opt-c",
                    "text": "Option C",
                    "voter_count": 2,
                    "added_by_chat": {
                        "id": -100123456,
                        "title": "Contributors",
                        "type": "supergroup"
                    },
                    "addition_date": 1775174405
                }
            ],
            "total_voter_count": 4,
            "is_closed": false,
            "is_anonymous": false,
            "type": "quiz",
            "allows_multiple_answers": true,
            "allows_revoting": true,
            "correct_option_ids": [0, 2],
            "description": "Vote with @alice",
            "description_entities": [
                {
                    "type": "text_mention",
                    "offset": 10,
                    "length": 6,
                    "user": {
                        "id": 702,
                        "is_bot": false,
                        "first_name": "Bob"
                    }
                }
            ]
        }"#;

        let poll: Poll = serde_json::from_str(data).unwrap();
        assert!(poll.allows_revoting);
        assert_eq!(poll.correct_option_ids, Some(vec![0, 2]));
        assert_eq!(poll.description.as_deref(), Some("Vote with @alice"));
        assert_eq!(poll.options[0].persistent_id, "opt-a");
        assert_eq!(poll.options[1].added_by_user.as_ref().map(|u| u.id), Some(UserId(701)));
        assert_eq!(poll.options[1].addition_date, DateTime::from_timestamp(1_775_174_400, 0));
        assert!(poll.options[2].added_by_chat.is_some());

        let mentioned: Vec<UserId> = poll.mentioned_users().map(|u| u.id).collect();
        assert!(mentioned.contains(&UserId(701)));
        assert!(mentioned.contains(&UserId(702)));

        let roundtrip: Poll = serde_json::from_str(&serde_json::to_string(&poll).unwrap()).unwrap();
        assert_eq!(poll, roundtrip);
    }

    #[test]
    fn legacy_quiz_and_revoting_defaults() {
        let legacy_quiz_json = r#"{
            "id": "legacy-quiz",
            "question": "2+2?",
            "options": [
                { "text": "3", "voter_count": 0 },
                { "text": "4", "voter_count": 2 }
            ],
            "total_voter_count": 2,
            "is_closed": false,
            "is_anonymous": true,
            "type": "quiz",
            "allows_multiple_answers": false,
            "correct_option_id": 1
        }"#;
        let legacy_quiz: Poll = serde_json::from_str(legacy_quiz_json).unwrap();
        assert_eq!(legacy_quiz.correct_option_ids, Some(vec![1]));
        assert!(!legacy_quiz.allows_revoting);

        let both_fields_json = r#"{
            "id": "both-fields",
            "question": "Pick",
            "options": [
                { "text": "A", "voter_count": 0 },
                { "text": "B", "voter_count": 0 }
            ],
            "total_voter_count": 0,
            "is_closed": false,
            "is_anonymous": true,
            "type": "quiz",
            "allows_multiple_answers": true,
            "correct_option_id": 0,
            "correct_option_ids": [0, 1]
        }"#;
        let both_fields: Poll = serde_json::from_str(both_fields_json).unwrap();
        assert_eq!(both_fields.correct_option_ids, Some(vec![0, 1]));

        let legacy_regular_json = r#"{
            "id": "legacy-regular",
            "question": "Color?",
            "options": [{ "text": "Blue", "voter_count": 1 }],
            "total_voter_count": 1,
            "is_closed": false,
            "is_anonymous": true,
            "type": "regular",
            "allows_multiple_answers": false
        }"#;
        let legacy_regular: Poll = serde_json::from_str(legacy_regular_json).unwrap();
        assert!(legacy_regular.allows_revoting);

        let no_revote_regular_json = r#"{
            "id": "no-revote-regular",
            "question": "Color?",
            "options": [{ "text": "Blue", "voter_count": 1 }],
            "total_voter_count": 1,
            "is_closed": false,
            "is_anonymous": true,
            "type": "regular",
            "allows_multiple_answers": false,
            "allows_revoting": false
        }"#;
        let no_revote_regular: Poll = serde_json::from_str(no_revote_regular_json).unwrap();
        assert!(!no_revote_regular.allows_revoting);
    }
}
