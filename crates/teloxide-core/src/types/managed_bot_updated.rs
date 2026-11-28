use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object contains information about a bot that was created to be managed
/// by the current bot, or about the update of its token or owner.
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
