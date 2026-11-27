use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object contains information about the creation or token change of a bot
/// that is managed by the current bot.
///
/// [The official docs](https://core.telegram.org/bots/api#managedbotupdated).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ManagedBotUpdated {
    /// User that created the bot.
    pub user: User,

    /// Information about the bot. The token of the bot can be fetched using
    /// the method `getManagedBotToken`.
    pub bot: User,
}
