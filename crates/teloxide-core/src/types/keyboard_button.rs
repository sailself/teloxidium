use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

use crate::types::{
    ButtonStyle, CustomEmojiId, KeyboardButtonPollType, KeyboardButtonRequestChat,
    KeyboardButtonRequestManagedBot, KeyboardButtonRequestUsers, True, WebAppInfo,
};

/// This object represents one button of the reply keyboard.
///
/// At most one of the fields other than `text`, `icon_custom_emoji_id`, and
/// `style` must be used to specify the type of the button.
///
/// For simple text buttons `String` can be used instead of this object to
/// specify text of the button.
///
/// [The official docs](https://core.telegram.org/bots/api#keyboardbutton).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct KeyboardButton {
    /// Text of the button. If none of the fields other than `text`,
    /// `icon_custom_emoji_id`, and `style` are used, it will be sent as a
    /// message when the button is pressed.
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

    /// Request something from user. This is available in private chats only.
    ///
    /// See [`ButtonRequest`] documentation for options on what can be
    /// requested.
    #[serde(flatten)]
    pub request: Option<ButtonRequest>,
}

impl KeyboardButton {
    pub fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self { text: text.into(), icon_custom_emoji_id: None, style: None, request: None }
    }

    pub fn icon_custom_emoji_id<T>(mut self, val: T) -> Self
    where
        T: Into<CustomEmojiId>,
    {
        self.icon_custom_emoji_id = Some(val.into());
        self
    }

    pub fn style(mut self, val: ButtonStyle) -> Self {
        self.style = Some(val);
        self
    }

    pub fn request<T>(mut self, val: T) -> Self
    where
        T: Into<ButtonRequest>,
    {
        self.request = Some(val.into());
        self
    }
}

/// Request something from user, when a button is pressed.
///
/// See individual variants documentation for more info.
#[derive(Clone, Debug, Eq, Hash, PartialEq /*, Serialize, Deserialize */)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub enum ButtonRequest {
    /// If this variant is used, the user's current location will be sent when
    /// the button is pressed.
    ///
    /// **Note:** this option will only work in Telegram versions released after
    /// 9 April, 2016. Older clients will display unsupported message.
    #[cfg_attr(test, schemars(rename = "request_location", with = "bool"))]
    Location,

    /// If this variant is used, the user's phone number will be sent as a
    /// contact when the button is pressed.
    ///
    /// **Note:** this option will only work in Telegram versions released after
    /// 9 April, 2016. Older clients will display unsupported message.
    #[cfg_attr(test, schemars(rename = "request_contact", with = "bool"))]
    Contact,

    /// If this variant is used, pressing the button will open a list of
    /// suitable chats. Tapping on a chat will send its identifier to the bot in
    /// a [`chat_shared`] service message.
    ///
    /// [`chat_shared`]: crate::types::MessageKind::ChatShared
    #[cfg_attr(test, schemars(rename = "request_chat"))]
    RequestChat(KeyboardButtonRequestChat),

    /// If specified, pressing the button will open a list of suitable users.
    /// Identifiers of selected users will be sent to the bot in a
    /// “users_shared” service message. Available in private chats only.
    #[cfg_attr(test, schemars(rename = "request_users"))]
    RequestUsers(KeyboardButtonRequestUsers),

    /// If specified, pressing the button will ask the user to create and share
    /// a bot that will be managed by the current bot. Available for bots that
    /// enabled management of other bots in the `@BotFather` Mini App.
    /// Available in private chats only.
    #[cfg_attr(test, schemars(rename = "request_managed_bot"))]
    RequestManagedBot(KeyboardButtonRequestManagedBot),

    /// If this variant is used, the user will be asked to create a poll and
    /// send it to the bot when the button is pressed.
    ///
    /// **Note:** this option will only work in Telegram versions released after
    /// 23 January, 2020. Older clients will display unsupported message.
    #[cfg_attr(test, schemars(rename = "request_poll"))]
    Poll(KeyboardButtonPollType),

    /// If this variant is used, the described Web App will be launched when the
    /// button is pressed. The Web App will be able to send a “web_app_data”
    /// service message.
    ///
    /// **Note:** this option will only work in Telegram versions released after
    /// 16 April, 2022. Older clients will display unsupported message.
    #[cfg_attr(test, schemars(rename = "web_app"))]
    WebApp(WebAppInfo),
}

/// Helper struct for (de)serializing [`ButtonRequest`](ButtonRequest)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
struct RawRequest {
    /// If `true`, the user's phone number will be sent as a contact
    /// when the button is pressed. Available in private chats only.
    #[serde(rename = "request_contact")]
    contact: Option<True>,

    /// If `true`, the user's current location will be sent when the
    /// button is pressed. Available in private chats only.
    #[serde(rename = "request_location")]
    location: Option<True>,

    /// If specified, pressing the button will open a list of suitable chats.
    /// Tapping on a chat will send its identifier to the bot in a “chat_shared”
    /// service message. Available in private chats only.
    #[serde(rename = "request_chat")]
    chat: Option<KeyboardButtonRequestChat>,

    /// If specified, pressing the button will open a list of suitable users.
    /// Identifiers of selected users will be sent to the bot in a
    /// “users_shared” service message. Available in private chats only.
    #[serde(rename = "request_users")]
    users: Option<KeyboardButtonRequestUsers>,

    /// If specified, pressing the button will ask the user to create and share
    /// a bot that will be managed by the current bot. Available in private
    /// chats only.
    #[serde(rename = "request_managed_bot")]
    managed_bot: Option<KeyboardButtonRequestManagedBot>,

    /// If specified, the user will be asked to create a poll and
    /// send it to the bot when the button is pressed. Available in private
    /// chats only.
    #[serde(rename = "request_poll")]
    poll: Option<KeyboardButtonPollType>,

    /// If specified, the described Web App will be launched when the button is
    /// pressed. The Web App will be able to send a “web_app_data” service
    /// message. Available in private chats only.
    web_app: Option<WebAppInfo>,
}

#[derive(Deserialize)]
struct RawKeyboardButton {
    text: String,
    icon_custom_emoji_id: Option<CustomEmojiId>,
    style: Option<ButtonStyle>,
    #[serde(flatten)]
    raw_request: RawRequest,
}

impl<'de> Deserialize<'de> for KeyboardButton {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawKeyboardButton::deserialize(deserializer)?;
        let request = raw.raw_request.into_optional_button_request()?;
        Ok(Self {
            text: raw.text,
            icon_custom_emoji_id: raw.icon_custom_emoji_id,
            style: raw.style,
            request,
        })
    }
}

impl RawRequest {
    fn is_empty(&self) -> bool {
        matches!(
            self,
            RawRequest {
                contact: None,
                location: None,
                chat: None,
                users: None,
                managed_bot: None,
                poll: None,
                web_app: None,
            }
        )
    }

    fn into_optional_button_request<E: Error>(self) -> Result<Option<ButtonRequest>, E> {
        if self.is_empty() {
            Ok(None)
        } else {
            ButtonRequest::try_from_raw(self).map(Some)
        }
    }
}

impl ButtonRequest {
    fn try_from_raw<E: Error>(raw: RawRequest) -> Result<Self, E> {
        match raw {
            RawRequest { contact, location, chat, users, managed_bot, poll, web_app }
                if 1 < (contact.is_some() as u8
                    + location.is_some() as u8
                    + chat.is_some() as u8
                    + users.is_some() as u8
                    + managed_bot.is_some() as u8
                    + poll.is_some() as u8
                    + web_app.is_some() as u8) =>
            {
                Err(E::custom(
                    "`request_contact`, `request_location`, `request_chat`, `request_users`, \
                     `request_managed_bot`, `request_poll` and `web_app` fields are mutually \
                     exclusive",
                ))
            }
            RawRequest { contact: Some(True), .. } => Ok(Self::Contact),
            RawRequest { location: Some(True), .. } => Ok(Self::Location),
            RawRequest { chat: Some(request_chat), .. } => Ok(Self::RequestChat(request_chat)),
            RawRequest { users: Some(request_users), .. } => Ok(Self::RequestUsers(request_users)),
            RawRequest { managed_bot: Some(request_managed_bot), .. } => {
                Ok(Self::RequestManagedBot(request_managed_bot))
            }
            RawRequest { poll: Some(poll_type), .. } => Ok(Self::Poll(poll_type)),
            RawRequest { web_app: Some(web_app), .. } => Ok(Self::WebApp(web_app)),

            RawRequest {
                contact: None,
                location: None,
                chat: None,
                users: None,
                managed_bot: None,
                poll: None,
                web_app: None,
            } => Err(E::custom(
                "Either one of `request_contact`, `request_chat`, `request_users`, \
                 `request_managed_bot`, `request_location`, `request_poll` and `web_app` fields \
                 is required",
            )),
        }
    }
}

impl<'de> Deserialize<'de> for ButtonRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::try_from_raw(RawRequest::deserialize(deserializer)?)
    }
}

impl Serialize for ButtonRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut raw = RawRequest {
            contact: None,
            location: None,
            chat: None,
            users: None,
            managed_bot: None,
            poll: None,
            web_app: None,
        };

        match self {
            Self::Contact => raw.contact = Some(True),
            Self::Location => raw.location = Some(True),
            Self::RequestChat(request_chat) => raw.chat = Some(request_chat.clone()),
            Self::RequestUsers(request_users) => raw.users = Some(request_users.clone()),
            Self::RequestManagedBot(request_managed_bot) => {
                raw.managed_bot = Some(request_managed_bot.clone())
            }
            Self::Poll(poll_type) => raw.poll = Some(poll_type.clone()),
            Self::WebApp(web_app) => raw.web_app = Some(web_app.clone()),
        };

        raw.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::RequestId;

    use super::*;

    #[test]
    fn serialize_no_request() {
        let button = KeyboardButton::new("");
        let expected = r#"{"text":""}"#;
        let actual = serde_json::to_string(&button).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn serialize_request_contact() {
        let button = KeyboardButton::new("").request(ButtonRequest::Contact);
        let expected = r#"{"text":"","request_contact":true}"#;
        let actual = serde_json::to_string(&button).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn serialize_chat_request() {
        let button = KeyboardButton::new("").request(ButtonRequest::RequestChat(
            KeyboardButtonRequestChat::new(RequestId(0), false),
        ));
        let expected = r#"{"text":"","request_chat":{"request_id":0,"chat_is_channel":false}}"#;
        let actual = serde_json::to_string(&button).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_no_request() {
        let json = r#"{"text":""}"#;
        let expected = KeyboardButton::new("");
        let actual = serde_json::from_str(json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_request_contact() {
        let json = r#"{"text":"","request_contact":true}"#;
        let expected = KeyboardButton::new("").request(ButtonRequest::Contact);
        let actual = serde_json::from_str(json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn styled_and_icon_roundtrip() {
        let button = KeyboardButton::new("Confirm")
            .icon_custom_emoji_id("5312536423851630001")
            .style(ButtonStyle::Success)
            .request(ButtonRequest::Contact);
        let json = serde_json::to_string(&button).unwrap();
        assert_eq!(
            json,
            r#"{"text":"Confirm","icon_custom_emoji_id":"5312536423851630001","style":"success","request_contact":true}"#
        );
        let parsed: KeyboardButton = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, button);
    }

    #[test]
    fn managed_bot_request_roundtrip_and_mutual_exclusion() {
        let button = KeyboardButton::new("Create Bot").request(ButtonRequest::RequestManagedBot(
            KeyboardButtonRequestManagedBot::new(RequestId(99))
                .suggested_name("Agent")
                .suggested_username("my_agent_bot"),
        ));
        let json = serde_json::to_string(&button).unwrap();
        assert_eq!(
            json,
            r#"{"text":"Create Bot","request_managed_bot":{"request_id":99,"suggested_name":"Agent","suggested_username":"my_agent_bot"}}"#
        );
        let parsed: KeyboardButton = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, button);

        let conflicting = r#"{
            "text": "Conflict",
            "request_contact": true,
            "request_managed_bot": { "request_id": 1 }
        }"#;
        assert!(serde_json::from_str::<KeyboardButton>(conflicting).is_err());
        assert!(serde_json::from_str::<ButtonRequest>(conflicting).is_err());

        let plain = r#"{"text":"Plain button"}"#;
        assert_eq!(
            serde_json::from_str::<KeyboardButton>(plain).unwrap(),
            KeyboardButton::new("Plain button")
        );

        let unknown_style: KeyboardButton =
            serde_json::from_str(r#"{"text":"Styled","style":"secondary"}"#).unwrap();
        assert_eq!(unknown_style.style, Some(ButtonStyle::Unknown));
        assert_eq!(serde_json::to_string(&unknown_style).unwrap(), r#"{"text":"Styled"}"#);
    }
}
