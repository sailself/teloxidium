use crate::types::{
    ButtonStyle, CallbackGame, CopyTextButton, CustomEmojiId, LoginUrl,
    SwitchInlineQueryChosenChat, True, WebAppInfo,
};
use serde::{Deserialize, Serialize};

/// This object represents one button of an inline keyboard.
///
/// Exactly one of the fields other than `text`, `icon_custom_emoji_id`, and
/// `style` must be used to specify the type of the button.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinekeyboardbutton).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InlineKeyboardButton {
    /// Label text on the button.
    pub text: String,

    /// Unique identifier of the custom emoji shown before the text of the
    /// button. Can only be used by bots that purchased additional usernames
    /// on Fragment or in the messages directly sent by the bot to private,
    /// group and supergroup chats if the owner of the bot has a Telegram
    /// Premium subscription.
    pub icon_custom_emoji_id: Option<CustomEmojiId>,

    /// Style of the button. If omitted, then an app-specific style is used.
    #[serde(skip_serializing_if = "ButtonStyle::is_none_or_unknown")]
    pub style: Option<ButtonStyle>,

    #[serde(flatten)]
    pub kind: InlineKeyboardButtonKind,
}

/// Marker payload for a disabled inline keyboard button.
///
/// [The official docs](https://core.telegram.org/bots/api#disabledbutton).
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct DisabledButton {}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[cfg_attr(test, schemars(rename_all = "snake_case"))]
pub enum InlineKeyboardButtonKind {
    /// HTTP or `tg://` url to be opened when button is pressed.
    ///
    /// Links in the form of `tg://user?id=<user_id>` can be used to mention a
    /// user by their ID without using a username, if this is allowed by
    /// their privacy settings.
    Url(reqwest::Url),

    /// An HTTPS URL used to automatically authorize the user. Can be used as a
    /// replacement for the [Telegram Login Widget].
    ///
    /// [Telegram Login Widget]: https://core.telegram.org/widgets/login
    LoginUrl(LoginUrl),

    /// Data to be sent in a [`CallbackQuery`] to the bot when button is
    /// pressed, 1-64 bytes.
    ///
    /// [`CallbackQuery`]: crate::types::CallbackQuery
    CallbackData(String),

    /// Description of the [Web App] that will be launched when the user presses
    /// the button. The Web App will be able to send an arbitrary message on
    /// behalf of the user using the method [`AnswerWebAppQuery`]. Available
    /// only in private chats between a user and the bot. Not supported for
    /// messages sent on behalf of a Telegram Business account.
    ///
    /// [Web App]: https://core.telegram.org/bots/webapps
    /// [`AnswerWebAppQuery`]: crate::payloads::AnswerWebAppQuery
    WebApp(WebAppInfo),

    /// If set, pressing the button will prompt the user to select one of their
    /// chats, open that chat and insert the bot‘s username and the specified
    /// inline query in the input field. Can be empty, in which case just the
    /// bot’s username will be inserted. Not supported for messages sent on
    /// behalf of a Telegram Business account.
    ///
    /// Note: This offers an easy way for users to start using your bot in
    /// [inline mode] when they are currently in a private chat with it.
    /// Especially useful when combined with [switch_pm…] actions – in this
    /// case the user will be automatically returned to the chat they
    /// switched from, skipping the chat selection screen.
    ///
    /// [inline mode]: https://core.telegram.org/bots/inline
    /// [switch_pm…]: crate::payloads::AnswerInlineQuery
    SwitchInlineQuery(String),

    /// If set, pressing the button will insert the bot‘s username and the
    /// specified inline query in the current chat's input field.
    /// Can be empty, in which case only the bot’s username will be
    /// inserted.
    ///
    /// This offers a quick way for the user to open your bot in inline mode in
    /// the same chat – good for selecting something from multiple options. Not
    /// supported in channels and for messages sent on behalf of a Telegram
    /// Business account.
    SwitchInlineQueryCurrentChat(String),

    /// If set, pressing the button will prompt the user to select one of their
    /// chats of the specified type, open that chat and insert the bot's
    /// username and the specified inline query in the input field. Not
    /// supported for messages sent on behalf of a Telegram Business account.
    SwitchInlineQueryChosenChat(SwitchInlineQueryChosenChat),

    /// Description of the button that copies the specified text to the
    /// clipboard.
    CopyText(CopyTextButton),

    /// Description of the game that will be launched when the user presses the
    /// button.
    ///
    /// ## Note
    ///
    /// This type of button **must** always be the first button in the first
    /// row.
    CallbackGame(CallbackGame),

    /// Specify True, to send a [Pay button].
    ///
    /// ## Note
    ///
    /// This type of button **must** always be the first button in the first
    /// row.
    ///
    /// [Pay button]: https://core.telegram.org/bots/api#payments
    Pay(True),

    /// A disabled button that does nothing when pressed.
    #[cfg_attr(test, schemars(skip))]
    Disabled(DisabledButton),

    /// A button kind unknown to this version of the library.
    #[cfg_attr(test, schemars(skip))]
    Unknown,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum KnownInlineKeyboardButtonKindRef<'a> {
    Url(&'a reqwest::Url),
    LoginUrl(&'a LoginUrl),
    CallbackData(&'a str),
    WebApp(&'a WebAppInfo),
    SwitchInlineQuery(&'a str),
    SwitchInlineQueryCurrentChat(&'a str),
    SwitchInlineQueryChosenChat(&'a SwitchInlineQueryChosenChat),
    CopyText(&'a CopyTextButton),
    CallbackGame(&'a CallbackGame),
    Pay(&'a True),
    Disabled(&'a DisabledButton),
}

impl Serialize for InlineKeyboardButtonKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let known = match self {
            Self::Url(v) => KnownInlineKeyboardButtonKindRef::Url(v),
            Self::LoginUrl(v) => KnownInlineKeyboardButtonKindRef::LoginUrl(v),
            Self::CallbackData(v) => KnownInlineKeyboardButtonKindRef::CallbackData(v),
            Self::WebApp(v) => KnownInlineKeyboardButtonKindRef::WebApp(v),
            Self::SwitchInlineQuery(v) => KnownInlineKeyboardButtonKindRef::SwitchInlineQuery(v),
            Self::SwitchInlineQueryCurrentChat(v) => {
                KnownInlineKeyboardButtonKindRef::SwitchInlineQueryCurrentChat(v)
            }
            Self::SwitchInlineQueryChosenChat(v) => {
                KnownInlineKeyboardButtonKindRef::SwitchInlineQueryChosenChat(v)
            }
            Self::CopyText(v) => KnownInlineKeyboardButtonKindRef::CopyText(v),
            Self::CallbackGame(v) => KnownInlineKeyboardButtonKindRef::CallbackGame(v),
            Self::Pay(v) => KnownInlineKeyboardButtonKindRef::Pay(v),
            Self::Disabled(v) => KnownInlineKeyboardButtonKindRef::Disabled(v),
            Self::Unknown => return serializer.serialize_map(Some(0))?.end(),
        };

        known.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for InlineKeyboardButtonKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct InlineKeyboardButtonKindVisitor;

        impl<'de> serde::de::Visitor<'de> for InlineKeyboardButtonKindVisitor {
            type Value = InlineKeyboardButtonKind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an inline keyboard button action")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut disabled: Option<DisabledButton> = None;
                let mut url: Option<reqwest::Url> = None;
                let mut login_url: Option<LoginUrl> = None;
                let mut callback_data: Option<String> = None;
                let mut web_app: Option<WebAppInfo> = None;
                let mut switch_inline_query: Option<String> = None;
                let mut switch_inline_query_current_chat: Option<String> = None;
                let mut switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat> = None;
                let mut copy_text: Option<CopyTextButton> = None;
                let mut callback_game: Option<CallbackGame> = None;
                let mut pay: Option<True> = None;
                let mut saw_unknown_key = false;

                while let Some(key) = map.next_key::<std::borrow::Cow<'de, str>>()? {
                    match key.as_ref() {
                        "disabled" => disabled = map.next_value()?,
                        "url" => url = map.next_value()?,
                        "login_url" => login_url = map.next_value()?,
                        "callback_data" => callback_data = map.next_value()?,
                        "web_app" => web_app = map.next_value()?,
                        "switch_inline_query" => switch_inline_query = map.next_value()?,
                        "switch_inline_query_current_chat" => {
                            switch_inline_query_current_chat = map.next_value()?
                        }
                        "switch_inline_query_chosen_chat" => {
                            switch_inline_query_chosen_chat = map.next_value()?
                        }
                        "copy_text" => copy_text = map.next_value()?,
                        "callback_game" => callback_game = map.next_value()?,
                        "pay" => pay = map.next_value()?,
                        _ => {
                            saw_unknown_key = true;
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                if let Some(v) = disabled {
                    Ok(InlineKeyboardButtonKind::Disabled(v))
                } else if let Some(v) = url {
                    Ok(InlineKeyboardButtonKind::Url(v))
                } else if let Some(v) = login_url {
                    Ok(InlineKeyboardButtonKind::LoginUrl(v))
                } else if let Some(v) = callback_data {
                    Ok(InlineKeyboardButtonKind::CallbackData(v))
                } else if let Some(v) = web_app {
                    Ok(InlineKeyboardButtonKind::WebApp(v))
                } else if let Some(v) = switch_inline_query {
                    Ok(InlineKeyboardButtonKind::SwitchInlineQuery(v))
                } else if let Some(v) = switch_inline_query_current_chat {
                    Ok(InlineKeyboardButtonKind::SwitchInlineQueryCurrentChat(v))
                } else if let Some(v) = switch_inline_query_chosen_chat {
                    Ok(InlineKeyboardButtonKind::SwitchInlineQueryChosenChat(v))
                } else if let Some(v) = copy_text {
                    Ok(InlineKeyboardButtonKind::CopyText(v))
                } else if let Some(v) = callback_game {
                    Ok(InlineKeyboardButtonKind::CallbackGame(v))
                } else if let Some(v) = pay {
                    Ok(InlineKeyboardButtonKind::Pay(v))
                } else if saw_unknown_key {
                    Ok(InlineKeyboardButtonKind::Unknown)
                } else {
                    Err(serde::de::Error::custom("missing inline keyboard button action field"))
                }
            }
        }

        deserializer.deserialize_map(InlineKeyboardButtonKindVisitor)
    }
}

impl InlineKeyboardButton {
    /// Creates a new `InlineKeyboardButton`.
    pub fn new<S>(text: S, kind: InlineKeyboardButtonKind) -> Self
    where
        S: Into<String>,
    {
        Self { text: text.into(), icon_custom_emoji_id: None, style: None, kind }
    }

    /// Sets the `icon_custom_emoji_id` on this button.
    pub fn icon_custom_emoji_id<T>(mut self, val: T) -> Self
    where
        T: Into<CustomEmojiId>,
    {
        self.icon_custom_emoji_id = Some(val.into());
        self
    }

    /// Sets the `style` on this button.
    pub fn style(mut self, val: ButtonStyle) -> Self {
        self.style = Some(val);
        self
    }

    /// Constructor for `InlineKeyboardButton` with [`Url`] kind.
    ///
    /// [`Url`]: InlineKeyboardButtonKind::Url
    pub fn url<T>(text: T, url: reqwest::Url) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonKind::Url(url))
    }

    /// Constructor for `InlineKeyboardButton` with [`LoginUrl`] kind.
    ///
    /// [`LoginUrl`]: InlineKeyboardButtonKind::LoginUrl
    pub fn login<T>(text: T, url: LoginUrl) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonKind::LoginUrl(url))
    }

    /// Constructor for `InlineKeyboardButton` with [`CallbackData`] kind.
    ///
    /// [`CallbackData`]: InlineKeyboardButtonKind::CallbackData
    pub fn callback<T, C>(text: T, callback_data: C) -> Self
    where
        T: Into<String>,
        C: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonKind::CallbackData(callback_data.into()))
    }

    /// Constructor for `InlineKeyboardButton` with [`WebApp`] kind.
    ///
    /// [`WebApp`]: InlineKeyboardButtonKind::WebApp
    pub fn web_app<T>(text: T, info: WebAppInfo) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonKind::WebApp(info))
    }

    /// Constructor for `InlineKeyboardButton` with [`SwitchInlineQuery`] kind.
    ///
    /// [`SwitchInlineQuery`]: InlineKeyboardButtonKind::SwitchInlineQuery
    pub fn switch_inline_query<T, Q>(text: T, switch_inline_query: Q) -> Self
    where
        T: Into<String>,
        Q: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonKind::SwitchInlineQuery(switch_inline_query.into()))
    }

    /// Constructor for `InlineKeyboardButton` with
    /// [`SwitchInlineQueryCurrentChat`] kind.
    ///
    /// [`SwitchInlineQueryCurrentChat`]: InlineKeyboardButtonKind::SwitchInlineQueryCurrentChat
    pub fn switch_inline_query_current_chat<T, Q>(
        text: T,
        switch_inline_query_current_chat: Q,
    ) -> Self
    where
        T: Into<String>,
        Q: Into<String>,
    {
        Self::new(
            text,
            InlineKeyboardButtonKind::SwitchInlineQueryCurrentChat(
                switch_inline_query_current_chat.into(),
            ),
        )
    }

    /// Constructor for `InlineKeyboardButton` with
    /// [`CopyText`] kind.
    ///
    /// [`CopyText`]: InlineKeyboardButtonKind::CopyText
    pub fn copy_text_button<T>(text: T, copy_text_button: CopyTextButton) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonKind::CopyText(copy_text_button))
    }

    /// Constructor for `InlineKeyboardButton` with [`CallbackGame`] kind.
    ///
    /// [`CallbackGame`]: InlineKeyboardButtonKind::CallbackGame
    pub fn callback_game<T>(text: T, game: CallbackGame) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonKind::CallbackGame(game))
    }

    /// Constructor for `InlineKeyboardButton` with [`Pay`] kind.
    ///
    /// [`Pay`]: InlineKeyboardButtonKind::Pay
    pub fn pay<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonKind::Pay(True))
    }

    /// Constructor for `InlineKeyboardButton` with [`Disabled`] kind.
    ///
    /// [`Disabled`]: InlineKeyboardButtonKind::Disabled
    pub fn disabled<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonKind::Disabled(DisabledButton {}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn styled_and_legacy_roundtrip() {
        let button = InlineKeyboardButton::callback("Delete", "delete_1")
            .icon_custom_emoji_id("5312536423851630009")
            .style(ButtonStyle::Danger);
        let json = serde_json::to_string(&button).unwrap();
        assert_eq!(
            json,
            r#"{"text":"Delete","icon_custom_emoji_id":"5312536423851630009","style":"danger","callback_data":"delete_1"}"#
        );
        let parsed: InlineKeyboardButton = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, button);

        let legacy_json = r#"{"text":"Go","callback_data":"go"}"#;
        let parsed_legacy: InlineKeyboardButton = serde_json::from_str(legacy_json).unwrap();
        assert_eq!(parsed_legacy, InlineKeyboardButton::callback("Go", "go"));
        assert_eq!(serde_json::to_string(&parsed_legacy).unwrap(), legacy_json);

        let disabled_json = r#"{"text":"Off","disabled":{}}"#;
        let parsed_disabled: InlineKeyboardButton = serde_json::from_str(disabled_json).unwrap();
        assert_eq!(parsed_disabled, InlineKeyboardButton::disabled("Off"));
        assert_eq!(serde_json::to_string(&parsed_disabled).unwrap(), disabled_json);

        let unknown_kind_json = r#"{"text":"Future","future_action":"x","style":"secondary"}"#;
        let parsed_unknown: InlineKeyboardButton = serde_json::from_str(unknown_kind_json).unwrap();
        assert_eq!(parsed_unknown.kind, InlineKeyboardButtonKind::Unknown);
        assert_eq!(parsed_unknown.style, Some(ButtonStyle::Unknown));

        let unknown_style_known_action: InlineKeyboardButton =
            serde_json::from_str(r#"{"text":"Go","callback_data":"go","style":"secondary"}"#)
                .unwrap();
        assert_eq!(unknown_style_known_action.style, Some(ButtonStyle::Unknown));
        assert_eq!(
            serde_json::to_string(&unknown_style_known_action).unwrap(),
            r#"{"text":"Go","callback_data":"go"}"#
        );

        assert!(serde_json::from_str::<InlineKeyboardButton>(r#"{"text":"Broken"}"#).is_err());
    }
}
