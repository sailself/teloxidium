use serde::{Deserialize, Serialize};

/// This object describes the rating of a user based on their Telegram Star
/// spending.
///
/// [The official docs](https://core.telegram.org/bots/api#userrating).
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UserRating {
    /// Current level of the user, indicating their reliability when purchasing
    /// digital goods and services. A higher level suggests a more trustworthy
    /// customer; a negative level is likely a reason for concern.
    pub level: i32,

    /// Numerical value of the user's rating; the higher the rating, the better
    pub rating: i64,

    /// The rating value required to get the current level
    pub current_level_rating: i64,

    /// The rating value required to get to the next level; omitted if the
    /// maximum level was reached
    pub next_level_rating: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let rating = UserRating {
            level: 3,
            rating: 1200,
            current_level_rating: 1000,
            next_level_rating: Some(2000),
        };
        let json = serde_json::to_string(&rating).unwrap();
        assert_eq!(
            json,
            r#"{"level":3,"rating":1200,"current_level_rating":1000,"next_level_rating":2000}"#
        );
        assert_eq!(serde_json::from_str::<UserRating>(&json).unwrap(), rating);

        let max_level = UserRating {
            level: 10,
            rating: 50000,
            current_level_rating: 50000,
            next_level_rating: None,
        };
        let max_json = serde_json::to_string(&max_level).unwrap();
        assert_eq!(max_json, r#"{"level":10,"rating":50000,"current_level_rating":50000}"#);
        assert_eq!(serde_json::from_str::<UserRating>(&max_json).unwrap(), max_level);

        let negative = UserRating {
            level: -2,
            rating: -150,
            current_level_rating: -200,
            next_level_rating: Some(0),
        };
        let neg_json = serde_json::to_string(&negative).unwrap();
        assert_eq!(serde_json::from_str::<UserRating>(&neg_json).unwrap(), negative);
    }
}
