use serde::{Deserialize, Serialize};

/// Represents the rights of an administrator in a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatAdministratorRights {
    /// `true`, if the user's presence in the chat is hidden
    pub is_anonymous: bool,

    /// `true`, if the administrator can access the chat event log, chat
    /// statistics, message statistics in channels, see channel members, see
    /// anonymous administrators in supergroups and ignore slow mode. Implied by
    /// any other administrator privilege
    pub can_manage_chat: bool,

    /// `true`, if the administrator can delete messages of other users
    pub can_delete_messages: bool,

    /// `true`, if the administrator can manage video chats
    pub can_manage_video_chats: bool,

    /// `true`, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: bool,

    /// `true`, if the administrator can add new administrators with a subset of
    /// their own privileges or demote administrators that he has promoted,
    /// directly or indirectly (promoted by administrators that were appointed
    /// by the user)
    pub can_promote_members: bool,

    /// `true`, if the user is allowed to change the chat title, photo and other
    /// settings
    pub can_change_info: bool,

    /// `true`, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,

    /// `true`, if the administrator can post in the channel; channels
    /// only
    pub can_post_messages: Option<bool>,

    /// `true`, if the administrator can edit messages of other users
    /// and can pin messages; channels only
    pub can_edit_messages: Option<bool>,

    /// `true`, if the user is allowed to pin messages; groups and
    /// supergroups only
    pub can_pin_messages: Option<bool>,

    /// `true`, if the administrator can post stories to the chat
    pub can_post_stories: Option<bool>,

    /// `true`, if the administrator can edit stories posted by other users
    pub can_edit_stories: Option<bool>,

    /// `true`, if the administrator can delete stories posted by other users
    pub can_delete_stories: Option<bool>,

    /// `true`, if the user is allowed to create, rename, close, and reopen
    /// forum topics; supergroups only
    pub can_manage_topics: Option<bool>,

    /// `true`, if the administrator can manage direct messages of the channel
    /// and decline suggested posts; for channels only
    pub can_manage_direct_messages: Option<bool>,

    /// `true`, if the administrator can edit the tags of regular members; for
    /// groups and supergroups only
    pub can_manage_tags: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_and_group_channel_fallbacks() {
        let group_json = r#"{
            "is_anonymous": false,
            "can_manage_chat": true,
            "can_delete_messages": true,
            "can_manage_video_chats": false,
            "can_restrict_members": false,
            "can_promote_members": false,
            "can_change_info": false,
            "can_invite_users": true,
            "can_manage_tags": true
        }"#;
        let group_rights: ChatAdministratorRights = serde_json::from_str(group_json).unwrap();
        assert_eq!(group_rights.can_manage_direct_messages, None);
        assert_eq!(group_rights.can_manage_tags, Some(true));
        let serialized_group = serde_json::to_string(&group_rights).unwrap();
        assert!(!serialized_group.contains("can_manage_direct_messages"));
        let group_roundtrip: ChatAdministratorRights =
            serde_json::from_str(&serialized_group).unwrap();
        assert_eq!(group_rights, group_roundtrip);

        let channel_json = r#"{
            "is_anonymous": false,
            "can_manage_chat": true,
            "can_delete_messages": true,
            "can_manage_video_chats": false,
            "can_restrict_members": false,
            "can_promote_members": false,
            "can_change_info": false,
            "can_invite_users": true,
            "can_manage_direct_messages": true
        }"#;
        let channel_rights: ChatAdministratorRights = serde_json::from_str(channel_json).unwrap();
        assert_eq!(channel_rights.can_manage_direct_messages, Some(true));
        assert_eq!(channel_rights.can_manage_tags, None);
        let serialized_channel = serde_json::to_string(&channel_rights).unwrap();
        assert!(!serialized_channel.contains("can_manage_tags"));
        let channel_roundtrip: ChatAdministratorRights =
            serde_json::from_str(&serialized_channel).unwrap();
        assert_eq!(channel_rights, channel_roundtrip);
    }
}
