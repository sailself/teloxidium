use url::Url;

use crate::{
    payloads,
    prelude::Requester,
    requests::{JsonRequest, MultipartRequest},
    types::{
        AcceptedGiftTypes, BotCommand, BusinessConnectionId, CallbackQueryId, ChatId,
        ChatPermissions, CustomEmojiId, DraftId, EphemeralMessageId, FileId, GiftId, GuestQueryId,
        InlineQueryId, InlineQueryResult, InputChecklist, InputFile, InputMedia, InputPaidMedia,
        InputPollOption, InputProfilePhoto, InputSticker, InputStoryContent, KeyboardButton,
        LabeledPrice, MessageId, OwnedGiftId, PreCheckoutQueryId, Recipient, Seconds,
        ShippingQueryId, StickerFormat, StoryId, TelegramTransactionId, ThreadId, UserId,
    },
    Bot,
};

impl Requester for Bot {
    type Err = crate::errors::RequestError;

    type GetUpdates = JsonRequest<payloads::GetUpdates>;

    fn get_updates(&self) -> Self::GetUpdates {
        Self::GetUpdates::new(self.clone(), payloads::GetUpdates::new())
    }

    type SetWebhook = MultipartRequest<payloads::SetWebhook>;

    fn set_webhook(&self, url: Url) -> Self::SetWebhook {
        Self::SetWebhook::new(self.clone(), payloads::SetWebhook::new(url))
    }

    type DeleteWebhook = JsonRequest<payloads::DeleteWebhook>;

    fn delete_webhook(&self) -> Self::DeleteWebhook {
        Self::DeleteWebhook::new(self.clone(), payloads::DeleteWebhook::new())
    }

    type GetWebhookInfo = JsonRequest<payloads::GetWebhookInfo>;

    fn get_webhook_info(&self) -> Self::GetWebhookInfo {
        Self::GetWebhookInfo::new(self.clone(), payloads::GetWebhookInfo::new())
    }

    type GetMe = JsonRequest<payloads::GetMe>;

    fn get_me(&self) -> Self::GetMe {
        Self::GetMe::new(self.clone(), payloads::GetMe::new())
    }

    type SendMessage = JsonRequest<payloads::SendMessage>;

    fn send_message<C, T>(&self, chat_id: C, text: T) -> Self::SendMessage
    where
        C: Into<Recipient>,
        T: Into<String>,
    {
        Self::SendMessage::new(self.clone(), payloads::SendMessage::new(chat_id, text))
    }

    type SendMessageDraft = JsonRequest<payloads::SendMessageDraft>;

    fn send_message_draft<C, T>(
        &self,
        chat_id: C,
        draft_id: DraftId,
        text: T,
    ) -> Self::SendMessageDraft
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        Self::SendMessageDraft::new(
            self.clone(),
            payloads::SendMessageDraft::new(chat_id, draft_id, text),
        )
    }

    type ForwardMessage = JsonRequest<payloads::ForwardMessage>;

    fn forward_message<C, F>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_id: MessageId,
    ) -> Self::ForwardMessage
    where
        C: Into<Recipient>,
        F: Into<Recipient>,
    {
        Self::ForwardMessage::new(
            self.clone(),
            payloads::ForwardMessage::new(chat_id, from_chat_id, message_id),
        )
    }

    type ForwardMessages = JsonRequest<payloads::ForwardMessages>;
    fn forward_messages<C, F, M>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_ids: M,
    ) -> Self::ForwardMessages
    where
        C: Into<Recipient>,
        F: Into<Recipient>,
        M: IntoIterator<Item = MessageId>,
    {
        Self::ForwardMessages::new(
            self.clone(),
            payloads::ForwardMessages::new(chat_id, from_chat_id, message_ids),
        )
    }

    type SendPhoto = MultipartRequest<payloads::SendPhoto>;

    fn send_photo<C>(&self, chat_id: C, photo: InputFile) -> Self::SendPhoto
    where
        C: Into<Recipient>,
    {
        Self::SendPhoto::new(self.clone(), payloads::SendPhoto::new(chat_id, photo))
    }

    type SendLivePhoto = MultipartRequest<payloads::SendLivePhoto>;

    fn send_live_photo<C>(
        &self,
        chat_id: C,
        live_photo: InputFile,
        photo: InputFile,
    ) -> Self::SendLivePhoto
    where
        C: Into<Recipient>,
    {
        Self::SendLivePhoto::new(
            self.clone(),
            payloads::SendLivePhoto::new(chat_id, live_photo, photo),
        )
    }

    type SendAudio = MultipartRequest<payloads::SendAudio>;

    fn send_audio<C>(&self, chat_id: C, audio: InputFile) -> Self::SendAudio
    where
        C: Into<Recipient>,
    {
        Self::SendAudio::new(self.clone(), payloads::SendAudio::new(chat_id, audio))
    }

    type SendDocument = MultipartRequest<payloads::SendDocument>;

    fn send_document<C>(&self, chat_id: C, document: InputFile) -> Self::SendDocument
    where
        C: Into<Recipient>,
    {
        Self::SendDocument::new(self.clone(), payloads::SendDocument::new(chat_id, document))
    }

    type SendVideo = MultipartRequest<payloads::SendVideo>;

    fn send_video<C>(&self, chat_id: C, video: InputFile) -> Self::SendVideo
    where
        C: Into<Recipient>,
    {
        Self::SendVideo::new(self.clone(), payloads::SendVideo::new(chat_id, video))
    }

    type SendAnimation = MultipartRequest<payloads::SendAnimation>;

    fn send_animation<C>(&self, chat_id: C, animation: InputFile) -> Self::SendAnimation
    where
        C: Into<Recipient>,
    {
        Self::SendAnimation::new(self.clone(), payloads::SendAnimation::new(chat_id, animation))
    }

    type SendVoice = MultipartRequest<payloads::SendVoice>;

    fn send_voice<C>(&self, chat_id: C, voice: InputFile) -> Self::SendVoice
    where
        C: Into<Recipient>,
    {
        Self::SendVoice::new(self.clone(), payloads::SendVoice::new(chat_id, voice))
    }

    type SendVideoNote = MultipartRequest<payloads::SendVideoNote>;

    fn send_video_note<C>(&self, chat_id: C, video_note: InputFile) -> Self::SendVideoNote
    where
        C: Into<Recipient>,
    {
        Self::SendVideoNote::new(self.clone(), payloads::SendVideoNote::new(chat_id, video_note))
    }

    type SendPaidMedia = MultipartRequest<payloads::SendPaidMedia>;

    fn send_paid_media<C, M>(&self, chat_id: C, star_count: u32, media: M) -> Self::SendPaidMedia
    where
        C: Into<Recipient>,
        M: IntoIterator<Item = InputPaidMedia>,
    {
        Self::SendPaidMedia::new(
            self.clone(),
            payloads::SendPaidMedia::new(chat_id, star_count, media),
        )
    }

    type SendMediaGroup = MultipartRequest<payloads::SendMediaGroup>;

    fn send_media_group<C, M>(&self, chat_id: C, media: M) -> Self::SendMediaGroup
    where
        C: Into<Recipient>,
        M: IntoIterator<Item = InputMedia>,
    {
        Self::SendMediaGroup::new(self.clone(), payloads::SendMediaGroup::new(chat_id, media))
    }

    type SendLocation = JsonRequest<payloads::SendLocation>;

    fn send_location<C>(&self, chat_id: C, latitude: f64, longitude: f64) -> Self::SendLocation
    where
        C: Into<Recipient>,
    {
        Self::SendLocation::new(
            self.clone(),
            payloads::SendLocation::new(chat_id, latitude, longitude),
        )
    }

    type EditMessageLiveLocation = JsonRequest<payloads::EditMessageLiveLocation>;

    fn edit_message_live_location<C>(
        &self,
        chat_id: C,
        message_id: MessageId,
        latitude: f64,
        longitude: f64,
    ) -> Self::EditMessageLiveLocation
    where
        C: Into<Recipient>,
    {
        Self::EditMessageLiveLocation::new(
            self.clone(),
            payloads::EditMessageLiveLocation::new(chat_id, message_id, latitude, longitude),
        )
    }

    type EditMessageLiveLocationInline = JsonRequest<payloads::EditMessageLiveLocationInline>;

    fn edit_message_live_location_inline<I>(
        &self,
        inline_message_id: I,
        latitude: f64,
        longitude: f64,
    ) -> Self::EditMessageLiveLocationInline
    where
        I: Into<String>,
    {
        Self::EditMessageLiveLocationInline::new(
            self.clone(),
            payloads::EditMessageLiveLocationInline::new(inline_message_id, latitude, longitude),
        )
    }

    type StopMessageLiveLocation = JsonRequest<payloads::StopMessageLiveLocation>;

    fn stop_message_live_location<C>(
        &self,
        chat_id: C,
        message_id: MessageId,
    ) -> Self::StopMessageLiveLocation
    where
        C: Into<Recipient>,
    {
        Self::StopMessageLiveLocation::new(
            self.clone(),
            payloads::StopMessageLiveLocation::new(chat_id, message_id),
        )
    }

    type StopMessageLiveLocationInline = JsonRequest<payloads::StopMessageLiveLocationInline>;

    fn stop_message_live_location_inline<I>(
        &self,
        inline_message_id: I,
    ) -> Self::StopMessageLiveLocationInline
    where
        I: Into<String>,
    {
        Self::StopMessageLiveLocationInline::new(
            self.clone(),
            payloads::StopMessageLiveLocationInline::new(inline_message_id),
        )
    }

    type EditMessageChecklist = JsonRequest<payloads::EditMessageChecklist>;

    fn edit_message_checklist<C>(
        &self,
        business_connection_id: BusinessConnectionId,
        chat_id: C,
        message_id: MessageId,
        checklist: InputChecklist,
    ) -> Self::EditMessageChecklist
    where
        C: Into<ChatId>,
    {
        Self::EditMessageChecklist::new(
            self.clone(),
            payloads::EditMessageChecklist::new(
                business_connection_id,
                chat_id,
                message_id,
                checklist,
            ),
        )
    }

    type SendVenue = JsonRequest<payloads::SendVenue>;

    fn send_venue<C, T, A>(
        &self,
        chat_id: C,
        latitude: f64,
        longitude: f64,
        title: T,
        address: A,
    ) -> Self::SendVenue
    where
        C: Into<Recipient>,
        T: Into<String>,
        A: Into<String>,
    {
        Self::SendVenue::new(
            self.clone(),
            payloads::SendVenue::new(chat_id, latitude, longitude, title, address),
        )
    }

    type SendContact = JsonRequest<payloads::SendContact>;

    fn send_contact<C, P, F>(&self, chat_id: C, phone_number: P, first_name: F) -> Self::SendContact
    where
        C: Into<Recipient>,
        P: Into<String>,
        F: Into<String>,
    {
        Self::SendContact::new(
            self.clone(),
            payloads::SendContact::new(chat_id, phone_number, first_name),
        )
    }

    type SendPoll = MultipartRequest<payloads::SendPoll>;

    fn send_poll<C, Q, O>(&self, chat_id: C, question: Q, options: O) -> Self::SendPoll
    where
        C: Into<Recipient>,
        Q: Into<String>,
        O: IntoIterator<Item = InputPollOption>,
    {
        Self::SendPoll::new(self.clone(), payloads::SendPoll::new(chat_id, question, options))
    }

    type SendChecklist = JsonRequest<payloads::SendChecklist>;

    fn send_checklist<C>(
        &self,
        business_connection_id: BusinessConnectionId,
        chat_id: C,
        checklist: InputChecklist,
    ) -> Self::SendChecklist
    where
        C: Into<ChatId>,
    {
        Self::SendChecklist::new(
            self.clone(),
            payloads::SendChecklist::new(business_connection_id, chat_id, checklist),
        )
    }

    type SendDice = JsonRequest<payloads::SendDice>;

    fn send_dice<C>(&self, chat_id: C) -> Self::SendDice
    where
        C: Into<Recipient>,
    {
        Self::SendDice::new(self.clone(), payloads::SendDice::new(chat_id))
    }

    type SendChatAction = JsonRequest<payloads::SendChatAction>;

    fn send_chat_action<C>(
        &self,
        chat_id: C,
        action: crate::types::ChatAction,
    ) -> Self::SendChatAction
    where
        C: Into<Recipient>,
    {
        Self::SendChatAction::new(self.clone(), payloads::SendChatAction::new(chat_id, action))
    }

    type SetMessageReaction = JsonRequest<payloads::SetMessageReaction>;

    fn set_message_reaction<C>(&self, chat_id: C, message_id: MessageId) -> Self::SetMessageReaction
    where
        C: Into<Recipient>,
    {
        Self::SetMessageReaction::new(
            self.clone(),
            payloads::SetMessageReaction::new(chat_id, message_id),
        )
    }

    type DeleteAllMessageReactions = JsonRequest<payloads::DeleteAllMessageReactions>;

    fn delete_all_message_reactions<C>(&self, chat_id: C) -> Self::DeleteAllMessageReactions
    where
        C: Into<Recipient>,
    {
        Self::DeleteAllMessageReactions::new(
            self.clone(),
            payloads::DeleteAllMessageReactions::new(chat_id),
        )
    }

    type DeleteMessageReaction = JsonRequest<payloads::DeleteMessageReaction>;

    fn delete_message_reaction<C>(
        &self,
        chat_id: C,
        message_id: MessageId,
    ) -> Self::DeleteMessageReaction
    where
        C: Into<Recipient>,
    {
        Self::DeleteMessageReaction::new(
            self.clone(),
            payloads::DeleteMessageReaction::new(chat_id, message_id),
        )
    }

    type GetUserProfilePhotos = JsonRequest<payloads::GetUserProfilePhotos>;

    fn get_user_profile_photos(&self, user_id: UserId) -> Self::GetUserProfilePhotos {
        Self::GetUserProfilePhotos::new(self.clone(), payloads::GetUserProfilePhotos::new(user_id))
    }

    type GetUserProfileAudios = JsonRequest<payloads::GetUserProfileAudios>;

    fn get_user_profile_audios(&self, user_id: UserId) -> Self::GetUserProfileAudios {
        Self::GetUserProfileAudios::new(self.clone(), payloads::GetUserProfileAudios::new(user_id))
    }

    type SetUserEmojiStatus = JsonRequest<payloads::SetUserEmojiStatus>;

    fn set_user_emoji_status(&self, user_id: UserId) -> Self::SetUserEmojiStatus {
        Self::SetUserEmojiStatus::new(self.clone(), payloads::SetUserEmojiStatus::new(user_id))
    }

    type GetFile = JsonRequest<payloads::GetFile>;

    fn get_file(&self, file_id: FileId) -> Self::GetFile {
        Self::GetFile::new(self.clone(), payloads::GetFile::new(file_id))
    }

    type KickChatMember = JsonRequest<payloads::KickChatMember>;

    fn kick_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::KickChatMember
    where
        C: Into<Recipient>,
    {
        Self::KickChatMember::new(self.clone(), payloads::KickChatMember::new(chat_id, user_id))
    }

    type BanChatMember = JsonRequest<payloads::BanChatMember>;

    fn ban_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::BanChatMember
    where
        C: Into<Recipient>,
    {
        Self::BanChatMember::new(self.clone(), payloads::BanChatMember::new(chat_id, user_id))
    }

    type UnbanChatMember = JsonRequest<payloads::UnbanChatMember>;

    fn unban_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::UnbanChatMember
    where
        C: Into<Recipient>,
    {
        Self::UnbanChatMember::new(self.clone(), payloads::UnbanChatMember::new(chat_id, user_id))
    }

    type RestrictChatMember = JsonRequest<payloads::RestrictChatMember>;

    fn restrict_chat_member<C>(
        &self,
        chat_id: C,
        user_id: UserId,
        permissions: ChatPermissions,
    ) -> Self::RestrictChatMember
    where
        C: Into<Recipient>,
    {
        Self::RestrictChatMember::new(
            self.clone(),
            payloads::RestrictChatMember::new(chat_id, user_id, permissions),
        )
    }

    type PromoteChatMember = JsonRequest<payloads::PromoteChatMember>;

    fn promote_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::PromoteChatMember
    where
        C: Into<Recipient>,
    {
        Self::PromoteChatMember::new(
            self.clone(),
            payloads::PromoteChatMember::new(chat_id, user_id),
        )
    }

    type SetChatAdministratorCustomTitle = JsonRequest<payloads::SetChatAdministratorCustomTitle>;

    fn set_chat_administrator_custom_title<Ch, Cu>(
        &self,
        chat_id: Ch,
        user_id: UserId,
        custom_title: Cu,
    ) -> Self::SetChatAdministratorCustomTitle
    where
        Ch: Into<Recipient>,
        Cu: Into<String>,
    {
        Self::SetChatAdministratorCustomTitle::new(
            self.clone(),
            payloads::SetChatAdministratorCustomTitle::new(chat_id, user_id, custom_title),
        )
    }

    type SetChatMemberTag = JsonRequest<payloads::SetChatMemberTag>;

    fn set_chat_member_tag<C>(&self, chat_id: C, user_id: UserId) -> Self::SetChatMemberTag
    where
        C: Into<Recipient>,
    {
        Self::SetChatMemberTag::new(self.clone(), payloads::SetChatMemberTag::new(chat_id, user_id))
    }

    type BanChatSenderChat = JsonRequest<payloads::BanChatSenderChat>;

    fn ban_chat_sender_chat<C, S>(&self, chat_id: C, sender_chat_id: S) -> Self::BanChatSenderChat
    where
        C: Into<Recipient>,
        S: Into<ChatId>,
    {
        Self::BanChatSenderChat::new(
            self.clone(),
            payloads::BanChatSenderChat::new(chat_id, sender_chat_id),
        )
    }

    type UnbanChatSenderChat = JsonRequest<payloads::UnbanChatSenderChat>;

    fn unban_chat_sender_chat<C, S>(
        &self,
        chat_id: C,
        sender_chat_id: S,
    ) -> Self::UnbanChatSenderChat
    where
        C: Into<Recipient>,
        S: Into<ChatId>,
    {
        Self::UnbanChatSenderChat::new(
            self.clone(),
            payloads::UnbanChatSenderChat::new(chat_id, sender_chat_id),
        )
    }

    type SetChatPermissions = JsonRequest<payloads::SetChatPermissions>;

    fn set_chat_permissions<C>(
        &self,
        chat_id: C,
        permissions: ChatPermissions,
    ) -> Self::SetChatPermissions
    where
        C: Into<Recipient>,
    {
        Self::SetChatPermissions::new(
            self.clone(),
            payloads::SetChatPermissions::new(chat_id, permissions),
        )
    }

    type ExportChatInviteLink = JsonRequest<payloads::ExportChatInviteLink>;

    fn export_chat_invite_link<C>(&self, chat_id: C) -> Self::ExportChatInviteLink
    where
        C: Into<Recipient>,
    {
        Self::ExportChatInviteLink::new(self.clone(), payloads::ExportChatInviteLink::new(chat_id))
    }

    type CreateChatInviteLink = JsonRequest<payloads::CreateChatInviteLink>;

    fn create_chat_invite_link<C>(&self, chat_id: C) -> Self::CreateChatInviteLink
    where
        C: Into<Recipient>,
    {
        Self::CreateChatInviteLink::new(self.clone(), payloads::CreateChatInviteLink::new(chat_id))
    }

    type EditChatInviteLink = JsonRequest<payloads::EditChatInviteLink>;

    fn edit_chat_invite_link<C, I>(&self, chat_id: C, invite_link: I) -> Self::EditChatInviteLink
    where
        C: Into<Recipient>,
        I: Into<String>,
    {
        Self::EditChatInviteLink::new(
            self.clone(),
            payloads::EditChatInviteLink::new(chat_id, invite_link),
        )
    }

    type CreateChatSubscriptionInviteLink = JsonRequest<payloads::CreateChatSubscriptionInviteLink>;

    fn create_chat_subscription_invite_link<C>(
        &self,
        chat_id: C,
        subscription_period: Seconds,
        subscription_price: u32,
    ) -> Self::CreateChatSubscriptionInviteLink
    where
        C: Into<Recipient>,
    {
        Self::CreateChatSubscriptionInviteLink::new(
            self.clone(),
            payloads::CreateChatSubscriptionInviteLink::new(
                chat_id,
                subscription_period,
                subscription_price,
            ),
        )
    }

    type EditChatSubscriptionInviteLink = JsonRequest<payloads::EditChatSubscriptionInviteLink>;

    fn edit_chat_subscription_invite_link<C, I>(
        &self,
        chat_id: C,
        invite_link: I,
    ) -> Self::EditChatSubscriptionInviteLink
    where
        C: Into<Recipient>,
        I: Into<String>,
    {
        Self::EditChatSubscriptionInviteLink::new(
            self.clone(),
            payloads::EditChatSubscriptionInviteLink::new(chat_id, invite_link),
        )
    }

    type RevokeChatInviteLink = JsonRequest<payloads::RevokeChatInviteLink>;

    fn revoke_chat_invite_link<C, I>(
        &self,
        chat_id: C,
        invite_link: I,
    ) -> Self::RevokeChatInviteLink
    where
        C: Into<Recipient>,
        I: Into<String>,
    {
        Self::RevokeChatInviteLink::new(
            self.clone(),
            payloads::RevokeChatInviteLink::new(chat_id, invite_link),
        )
    }

    type ApproveChatJoinRequest = JsonRequest<payloads::ApproveChatJoinRequest>;

    fn approve_chat_join_request<C>(
        &self,
        chat_id: C,
        user_id: UserId,
    ) -> Self::ApproveChatJoinRequest
    where
        C: Into<Recipient>,
    {
        Self::ApproveChatJoinRequest::new(
            self.clone(),
            payloads::ApproveChatJoinRequest::new(chat_id, user_id),
        )
    }

    type DeclineChatJoinRequest = JsonRequest<payloads::DeclineChatJoinRequest>;

    fn decline_chat_join_request<C>(
        &self,
        chat_id: C,
        user_id: UserId,
    ) -> Self::DeclineChatJoinRequest
    where
        C: Into<Recipient>,
    {
        Self::DeclineChatJoinRequest::new(
            self.clone(),
            payloads::DeclineChatJoinRequest::new(chat_id, user_id),
        )
    }

    type SetChatPhoto = MultipartRequest<payloads::SetChatPhoto>;

    fn set_chat_photo<C>(&self, chat_id: C, photo: InputFile) -> Self::SetChatPhoto
    where
        C: Into<Recipient>,
    {
        Self::SetChatPhoto::new(self.clone(), payloads::SetChatPhoto::new(chat_id, photo))
    }

    type DeleteChatPhoto = JsonRequest<payloads::DeleteChatPhoto>;

    fn delete_chat_photo<C>(&self, chat_id: C) -> Self::DeleteChatPhoto
    where
        C: Into<Recipient>,
    {
        Self::DeleteChatPhoto::new(self.clone(), payloads::DeleteChatPhoto::new(chat_id))
    }

    type SetChatTitle = JsonRequest<payloads::SetChatTitle>;

    fn set_chat_title<C, T>(&self, chat_id: C, title: T) -> Self::SetChatTitle
    where
        C: Into<Recipient>,
        T: Into<String>,
    {
        Self::SetChatTitle::new(self.clone(), payloads::SetChatTitle::new(chat_id, title))
    }

    type SetChatDescription = JsonRequest<payloads::SetChatDescription>;

    fn set_chat_description<C>(&self, chat_id: C) -> Self::SetChatDescription
    where
        C: Into<Recipient>,
    {
        Self::SetChatDescription::new(self.clone(), payloads::SetChatDescription::new(chat_id))
    }

    type PinChatMessage = JsonRequest<payloads::PinChatMessage>;

    fn pin_chat_message<C>(&self, chat_id: C, message_id: MessageId) -> Self::PinChatMessage
    where
        C: Into<Recipient>,
    {
        Self::PinChatMessage::new(self.clone(), payloads::PinChatMessage::new(chat_id, message_id))
    }

    type UnpinChatMessage = JsonRequest<payloads::UnpinChatMessage>;

    fn unpin_chat_message<C>(&self, chat_id: C) -> Self::UnpinChatMessage
    where
        C: Into<Recipient>,
    {
        Self::UnpinChatMessage::new(self.clone(), payloads::UnpinChatMessage::new(chat_id))
    }

    type LeaveChat = JsonRequest<payloads::LeaveChat>;

    fn leave_chat<C>(&self, chat_id: C) -> Self::LeaveChat
    where
        C: Into<Recipient>,
    {
        Self::LeaveChat::new(self.clone(), payloads::LeaveChat::new(chat_id))
    }

    type GetChat = JsonRequest<payloads::GetChat>;

    fn get_chat<C>(&self, chat_id: C) -> Self::GetChat
    where
        C: Into<Recipient>,
    {
        Self::GetChat::new(self.clone(), payloads::GetChat::new(chat_id))
    }

    type GetChatAdministrators = JsonRequest<payloads::GetChatAdministrators>;

    fn get_chat_administrators<C>(&self, chat_id: C) -> Self::GetChatAdministrators
    where
        C: Into<Recipient>,
    {
        Self::GetChatAdministrators::new(
            self.clone(),
            payloads::GetChatAdministrators::new(chat_id),
        )
    }

    type GetChatMembersCount = JsonRequest<payloads::GetChatMembersCount>;

    fn get_chat_members_count<C>(&self, chat_id: C) -> Self::GetChatMembersCount
    where
        C: Into<Recipient>,
    {
        Self::GetChatMembersCount::new(self.clone(), payloads::GetChatMembersCount::new(chat_id))
    }

    type GetChatMemberCount = JsonRequest<payloads::GetChatMemberCount>;

    fn get_chat_member_count<C>(&self, chat_id: C) -> Self::GetChatMemberCount
    where
        C: Into<Recipient>,
    {
        Self::GetChatMemberCount::new(self.clone(), payloads::GetChatMemberCount::new(chat_id))
    }

    type GetChatMember = JsonRequest<payloads::GetChatMember>;

    fn get_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::GetChatMember
    where
        C: Into<Recipient>,
    {
        Self::GetChatMember::new(self.clone(), payloads::GetChatMember::new(chat_id, user_id))
    }

    type SetChatStickerSet = JsonRequest<payloads::SetChatStickerSet>;

    fn set_chat_sticker_set<C, S>(&self, chat_id: C, sticker_set_name: S) -> Self::SetChatStickerSet
    where
        C: Into<Recipient>,
        S: Into<String>,
    {
        Self::SetChatStickerSet::new(
            self.clone(),
            payloads::SetChatStickerSet::new(chat_id, sticker_set_name),
        )
    }

    type DeleteChatStickerSet = JsonRequest<payloads::DeleteChatStickerSet>;

    fn delete_chat_sticker_set<C>(&self, chat_id: C) -> Self::DeleteChatStickerSet
    where
        C: Into<Recipient>,
    {
        Self::DeleteChatStickerSet::new(self.clone(), payloads::DeleteChatStickerSet::new(chat_id))
    }

    type GetForumTopicIconStickers = JsonRequest<payloads::GetForumTopicIconStickers>;

    fn get_forum_topic_icon_stickers(&self) -> Self::GetForumTopicIconStickers {
        Self::GetForumTopicIconStickers::new(
            self.clone(),
            payloads::GetForumTopicIconStickers::new(),
        )
    }

    type CreateForumTopic = JsonRequest<payloads::CreateForumTopic>;

    fn create_forum_topic<C, N>(&self, chat_id: C, name: N) -> Self::CreateForumTopic
    where
        C: Into<Recipient>,
        N: Into<String>,
    {
        Self::CreateForumTopic::new(self.clone(), payloads::CreateForumTopic::new(chat_id, name))
    }

    type EditForumTopic = JsonRequest<payloads::EditForumTopic>;

    fn edit_forum_topic<C>(&self, chat_id: C, message_thread_id: ThreadId) -> Self::EditForumTopic
    where
        C: Into<Recipient>,
    {
        Self::EditForumTopic::new(
            self.clone(),
            payloads::EditForumTopic::new(chat_id, message_thread_id),
        )
    }

    type CloseForumTopic = JsonRequest<payloads::CloseForumTopic>;

    fn close_forum_topic<C>(&self, chat_id: C, message_thread_id: ThreadId) -> Self::CloseForumTopic
    where
        C: Into<Recipient>,
    {
        Self::CloseForumTopic::new(
            self.clone(),
            payloads::CloseForumTopic::new(chat_id, message_thread_id),
        )
    }

    type ReopenForumTopic = JsonRequest<payloads::ReopenForumTopic>;

    fn reopen_forum_topic<C>(
        &self,
        chat_id: C,
        message_thread_id: ThreadId,
    ) -> Self::ReopenForumTopic
    where
        C: Into<Recipient>,
    {
        Self::ReopenForumTopic::new(
            self.clone(),
            payloads::ReopenForumTopic::new(chat_id, message_thread_id),
        )
    }

    type DeleteForumTopic = JsonRequest<payloads::DeleteForumTopic>;

    fn delete_forum_topic<C>(
        &self,
        chat_id: C,
        message_thread_id: ThreadId,
    ) -> Self::DeleteForumTopic
    where
        C: Into<Recipient>,
    {
        Self::DeleteForumTopic::new(
            self.clone(),
            payloads::DeleteForumTopic::new(chat_id, message_thread_id),
        )
    }

    type UnpinAllForumTopicMessages = JsonRequest<payloads::UnpinAllForumTopicMessages>;

    fn unpin_all_forum_topic_messages<C>(
        &self,
        chat_id: C,
        message_thread_id: ThreadId,
    ) -> Self::UnpinAllForumTopicMessages
    where
        C: Into<Recipient>,
    {
        Self::UnpinAllForumTopicMessages::new(
            self.clone(),
            payloads::UnpinAllForumTopicMessages::new(chat_id, message_thread_id),
        )
    }

    type EditGeneralForumTopic = JsonRequest<payloads::EditGeneralForumTopic>;

    fn edit_general_forum_topic<C, N>(&self, chat_id: C, name: N) -> Self::EditGeneralForumTopic
    where
        C: Into<Recipient>,
        N: Into<String>,
    {
        Self::EditGeneralForumTopic::new(
            self.clone(),
            payloads::EditGeneralForumTopic::new(chat_id, name),
        )
    }

    type CloseGeneralForumTopic = JsonRequest<payloads::CloseGeneralForumTopic>;

    fn close_general_forum_topic<C>(&self, chat_id: C) -> Self::CloseGeneralForumTopic
    where
        C: Into<Recipient>,
    {
        Self::CloseGeneralForumTopic::new(
            self.clone(),
            payloads::CloseGeneralForumTopic::new(chat_id),
        )
    }

    type ReopenGeneralForumTopic = JsonRequest<payloads::ReopenGeneralForumTopic>;

    fn reopen_general_forum_topic<C>(&self, chat_id: C) -> Self::ReopenGeneralForumTopic
    where
        C: Into<Recipient>,
    {
        Self::ReopenGeneralForumTopic::new(
            self.clone(),
            payloads::ReopenGeneralForumTopic::new(chat_id),
        )
    }

    type HideGeneralForumTopic = JsonRequest<payloads::HideGeneralForumTopic>;

    fn hide_general_forum_topic<C>(&self, chat_id: C) -> Self::HideGeneralForumTopic
    where
        C: Into<Recipient>,
    {
        Self::HideGeneralForumTopic::new(
            self.clone(),
            payloads::HideGeneralForumTopic::new(chat_id),
        )
    }

    type UnhideGeneralForumTopic = JsonRequest<payloads::UnhideGeneralForumTopic>;

    fn unhide_general_forum_topic<C>(&self, chat_id: C) -> Self::UnhideGeneralForumTopic
    where
        C: Into<Recipient>,
    {
        Self::UnhideGeneralForumTopic::new(
            self.clone(),
            payloads::UnhideGeneralForumTopic::new(chat_id),
        )
    }

    type UnpinAllGeneralForumTopicMessages =
        JsonRequest<payloads::UnpinAllGeneralForumTopicMessages>;

    fn unpin_all_general_forum_topic_messages<C>(
        &self,
        chat_id: C,
    ) -> Self::UnpinAllGeneralForumTopicMessages
    where
        C: Into<Recipient>,
    {
        Self::UnpinAllGeneralForumTopicMessages::new(
            self.clone(),
            payloads::UnpinAllGeneralForumTopicMessages::new(chat_id),
        )
    }

    type AnswerCallbackQuery = JsonRequest<payloads::AnswerCallbackQuery>;

    fn answer_callback_query(
        &self,
        callback_query_id: CallbackQueryId,
    ) -> Self::AnswerCallbackQuery {
        Self::AnswerCallbackQuery::new(
            self.clone(),
            payloads::AnswerCallbackQuery::new(callback_query_id),
        )
    }

    type GetUserChatBoosts = JsonRequest<payloads::GetUserChatBoosts>;

    fn get_user_chat_boosts<C>(&self, chat_id: C, user_id: UserId) -> Self::GetUserChatBoosts
    where
        C: Into<Recipient>,
    {
        Self::GetUserChatBoosts::new(
            self.clone(),
            payloads::GetUserChatBoosts::new(chat_id, user_id),
        )
    }

    type SetMyCommands = JsonRequest<payloads::SetMyCommands>;

    fn set_my_commands<C>(&self, commands: C) -> Self::SetMyCommands
    where
        C: IntoIterator<Item = BotCommand>,
    {
        Self::SetMyCommands::new(self.clone(), payloads::SetMyCommands::new(commands))
    }

    type GetBusinessConnection = JsonRequest<payloads::GetBusinessConnection>;

    fn get_business_connection(
        &self,
        business_connection_id: BusinessConnectionId,
    ) -> Self::GetBusinessConnection {
        Self::GetBusinessConnection::new(
            self.clone(),
            payloads::GetBusinessConnection::new(business_connection_id),
        )
    }

    type GetMyCommands = JsonRequest<payloads::GetMyCommands>;

    fn get_my_commands(&self) -> Self::GetMyCommands {
        Self::GetMyCommands::new(self.clone(), payloads::GetMyCommands::new())
    }

    type SetMyProfilePhoto = MultipartRequest<payloads::SetMyProfilePhoto>;

    fn set_my_profile_photo(&self, photo: InputProfilePhoto) -> Self::SetMyProfilePhoto {
        Self::SetMyProfilePhoto::new(self.clone(), payloads::SetMyProfilePhoto::new(photo))
    }

    type RemoveMyProfilePhoto = JsonRequest<payloads::RemoveMyProfilePhoto>;

    fn remove_my_profile_photo(&self) -> Self::RemoveMyProfilePhoto {
        Self::RemoveMyProfilePhoto::new(self.clone(), payloads::RemoveMyProfilePhoto::new())
    }

    type SetMyName = JsonRequest<payloads::SetMyName>;

    fn set_my_name(&self) -> Self::SetMyName {
        Self::SetMyName::new(self.clone(), payloads::SetMyName::new())
    }

    type GetMyName = JsonRequest<payloads::GetMyName>;

    fn get_my_name(&self) -> Self::GetMyName {
        Self::GetMyName::new(self.clone(), payloads::GetMyName::new())
    }

    type SetMyDescription = JsonRequest<payloads::SetMyDescription>;

    fn set_my_description(&self) -> Self::SetMyDescription {
        Self::SetMyDescription::new(self.clone(), payloads::SetMyDescription::new())
    }

    type GetMyDescription = JsonRequest<payloads::GetMyDescription>;

    fn get_my_description(&self) -> Self::GetMyDescription {
        Self::GetMyDescription::new(self.clone(), payloads::GetMyDescription::new())
    }

    type SetMyShortDescription = JsonRequest<payloads::SetMyShortDescription>;

    fn set_my_short_description(&self) -> Self::SetMyShortDescription {
        Self::SetMyShortDescription::new(self.clone(), payloads::SetMyShortDescription::new())
    }

    type GetMyShortDescription = JsonRequest<payloads::GetMyShortDescription>;
    fn get_my_short_description(&self) -> Self::GetMyShortDescription {
        Self::GetMyShortDescription::new(self.clone(), payloads::GetMyShortDescription::new())
    }

    type SetChatMenuButton = JsonRequest<payloads::SetChatMenuButton>;

    fn set_chat_menu_button(&self) -> Self::SetChatMenuButton {
        Self::SetChatMenuButton::new(self.clone(), payloads::SetChatMenuButton::new())
    }

    type GetChatMenuButton = JsonRequest<payloads::GetChatMenuButton>;

    fn get_chat_menu_button(&self) -> Self::GetChatMenuButton {
        Self::GetChatMenuButton::new(self.clone(), payloads::GetChatMenuButton::new())
    }

    type SetMyDefaultAdministratorRights = JsonRequest<payloads::SetMyDefaultAdministratorRights>;

    fn set_my_default_administrator_rights(&self) -> Self::SetMyDefaultAdministratorRights {
        Self::SetMyDefaultAdministratorRights::new(
            self.clone(),
            payloads::SetMyDefaultAdministratorRights::new(),
        )
    }

    type GetMyDefaultAdministratorRights = JsonRequest<payloads::GetMyDefaultAdministratorRights>;

    fn get_my_default_administrator_rights(&self) -> Self::GetMyDefaultAdministratorRights {
        Self::GetMyDefaultAdministratorRights::new(
            self.clone(),
            payloads::GetMyDefaultAdministratorRights::new(),
        )
    }

    type DeleteMyCommands = JsonRequest<payloads::DeleteMyCommands>;

    fn delete_my_commands(&self) -> Self::DeleteMyCommands {
        Self::DeleteMyCommands::new(self.clone(), payloads::DeleteMyCommands::new())
    }

    type AnswerInlineQuery = JsonRequest<payloads::AnswerInlineQuery>;

    fn answer_inline_query<R>(
        &self,
        inline_query_id: InlineQueryId,
        results: R,
    ) -> Self::AnswerInlineQuery
    where
        R: IntoIterator<Item = InlineQueryResult>,
    {
        Self::AnswerInlineQuery::new(
            self.clone(),
            payloads::AnswerInlineQuery::new(inline_query_id, results),
        )
    }

    type AnswerWebAppQuery = JsonRequest<payloads::AnswerWebAppQuery>;

    fn answer_web_app_query<W>(
        &self,
        web_app_query_id: W,
        result: InlineQueryResult,
    ) -> Self::AnswerWebAppQuery
    where
        W: Into<String>,
    {
        Self::AnswerWebAppQuery::new(
            self.clone(),
            payloads::AnswerWebAppQuery::new(web_app_query_id, result),
        )
    }

    type AnswerGuestQuery = JsonRequest<payloads::AnswerGuestQuery>;

    fn answer_guest_query(
        &self,
        guest_query_id: GuestQueryId,
        result: InlineQueryResult,
    ) -> Self::AnswerGuestQuery {
        Self::AnswerGuestQuery::new(
            self.clone(),
            payloads::AnswerGuestQuery::new(guest_query_id, result),
        )
    }

    type SavePreparedInlineMessage = JsonRequest<payloads::SavePreparedInlineMessage>;

    fn save_prepared_inline_message(
        &self,
        user_id: UserId,
        result: InlineQueryResult,
    ) -> Self::SavePreparedInlineMessage {
        Self::SavePreparedInlineMessage::new(
            self.clone(),
            payloads::SavePreparedInlineMessage::new(user_id, result),
        )
    }

    type SavePreparedKeyboardButton = JsonRequest<payloads::SavePreparedKeyboardButton>;

    fn save_prepared_keyboard_button(
        &self,
        user_id: UserId,
        button: KeyboardButton,
    ) -> Self::SavePreparedKeyboardButton {
        Self::SavePreparedKeyboardButton::new(
            self.clone(),
            payloads::SavePreparedKeyboardButton::new(user_id, button),
        )
    }

    type GetManagedBotToken = JsonRequest<payloads::GetManagedBotToken>;

    fn get_managed_bot_token(&self, user_id: UserId) -> Self::GetManagedBotToken {
        Self::GetManagedBotToken::new(self.clone(), payloads::GetManagedBotToken::new(user_id))
    }

    type ReplaceManagedBotToken = JsonRequest<payloads::ReplaceManagedBotToken>;

    fn replace_managed_bot_token(&self, user_id: UserId) -> Self::ReplaceManagedBotToken {
        Self::ReplaceManagedBotToken::new(
            self.clone(),
            payloads::ReplaceManagedBotToken::new(user_id),
        )
    }

    type GetManagedBotAccessSettings = JsonRequest<payloads::GetManagedBotAccessSettings>;

    fn get_managed_bot_access_settings(
        &self,
        user_id: UserId,
    ) -> Self::GetManagedBotAccessSettings {
        Self::GetManagedBotAccessSettings::new(
            self.clone(),
            payloads::GetManagedBotAccessSettings::new(user_id),
        )
    }

    type SetManagedBotAccessSettings = JsonRequest<payloads::SetManagedBotAccessSettings>;

    fn set_managed_bot_access_settings(
        &self,
        user_id: UserId,
        is_access_restricted: bool,
    ) -> Self::SetManagedBotAccessSettings {
        Self::SetManagedBotAccessSettings::new(
            self.clone(),
            payloads::SetManagedBotAccessSettings::new(user_id, is_access_restricted),
        )
    }

    type GetUserPersonalChatMessages = JsonRequest<payloads::GetUserPersonalChatMessages>;

    fn get_user_personal_chat_messages(
        &self,
        user_id: UserId,
        limit: u8,
    ) -> Self::GetUserPersonalChatMessages {
        Self::GetUserPersonalChatMessages::new(
            self.clone(),
            payloads::GetUserPersonalChatMessages::new(user_id, limit),
        )
    }

    type EditMessageText = JsonRequest<payloads::EditMessageText>;

    fn edit_message_text<C, T>(
        &self,
        chat_id: C,
        message_id: MessageId,
        text: T,
    ) -> Self::EditMessageText
    where
        C: Into<Recipient>,
        T: Into<String>,
    {
        Self::EditMessageText::new(
            self.clone(),
            payloads::EditMessageText::new(chat_id, message_id, text),
        )
    }

    type EditMessageTextInline = JsonRequest<payloads::EditMessageTextInline>;

    fn edit_message_text_inline<I, T>(
        &self,
        inline_message_id: I,
        text: T,
    ) -> Self::EditMessageTextInline
    where
        I: Into<String>,
        T: Into<String>,
    {
        Self::EditMessageTextInline::new(
            self.clone(),
            payloads::EditMessageTextInline::new(inline_message_id, text),
        )
    }

    type EditMessageCaption = JsonRequest<payloads::EditMessageCaption>;

    fn edit_message_caption<C>(&self, chat_id: C, message_id: MessageId) -> Self::EditMessageCaption
    where
        C: Into<Recipient>,
    {
        Self::EditMessageCaption::new(
            self.clone(),
            payloads::EditMessageCaption::new(chat_id, message_id),
        )
    }

    type EditMessageCaptionInline = JsonRequest<payloads::EditMessageCaptionInline>;

    fn edit_message_caption_inline<I>(&self, inline_message_id: I) -> Self::EditMessageCaptionInline
    where
        I: Into<String>,
    {
        Self::EditMessageCaptionInline::new(
            self.clone(),
            payloads::EditMessageCaptionInline::new(inline_message_id),
        )
    }

    type EditMessageMedia = MultipartRequest<payloads::EditMessageMedia>;

    fn edit_message_media<C>(
        &self,
        chat_id: C,
        message_id: MessageId,
        media: InputMedia,
    ) -> Self::EditMessageMedia
    where
        C: Into<Recipient>,
    {
        Self::EditMessageMedia::new(
            self.clone(),
            payloads::EditMessageMedia::new(chat_id, message_id, media),
        )
    }

    type EditMessageMediaInline = MultipartRequest<payloads::EditMessageMediaInline>;

    fn edit_message_media_inline<I>(
        &self,
        inline_message_id: I,
        media: InputMedia,
    ) -> Self::EditMessageMediaInline
    where
        I: Into<String>,
    {
        Self::EditMessageMediaInline::new(
            self.clone(),
            payloads::EditMessageMediaInline::new(inline_message_id, media),
        )
    }

    type EditMessageReplyMarkup = JsonRequest<payloads::EditMessageReplyMarkup>;

    fn edit_message_reply_markup<C>(
        &self,
        chat_id: C,
        message_id: MessageId,
    ) -> Self::EditMessageReplyMarkup
    where
        C: Into<Recipient>,
    {
        Self::EditMessageReplyMarkup::new(
            self.clone(),
            payloads::EditMessageReplyMarkup::new(chat_id, message_id),
        )
    }

    type EditMessageReplyMarkupInline = JsonRequest<payloads::EditMessageReplyMarkupInline>;

    fn edit_message_reply_markup_inline<I>(
        &self,
        inline_message_id: I,
    ) -> Self::EditMessageReplyMarkupInline
    where
        I: Into<String>,
    {
        Self::EditMessageReplyMarkupInline::new(
            self.clone(),
            payloads::EditMessageReplyMarkupInline::new(inline_message_id),
        )
    }

    type StopPoll = JsonRequest<payloads::StopPoll>;

    fn stop_poll<C>(&self, chat_id: C, message_id: MessageId) -> Self::StopPoll
    where
        C: Into<Recipient>,
    {
        Self::StopPoll::new(self.clone(), payloads::StopPoll::new(chat_id, message_id))
    }

    type ApproveSuggestedPost = JsonRequest<payloads::ApproveSuggestedPost>;

    fn approve_suggested_post<C>(
        &self,
        chat_id: C,
        message_id: MessageId,
    ) -> Self::ApproveSuggestedPost
    where
        C: Into<ChatId>,
    {
        Self::ApproveSuggestedPost::new(
            self.clone(),
            payloads::ApproveSuggestedPost::new(chat_id, message_id),
        )
    }

    type DeclineSuggestedPost = JsonRequest<payloads::DeclineSuggestedPost>;

    fn decline_suggested_post<C>(
        &self,
        chat_id: C,
        message_id: MessageId,
    ) -> Self::DeclineSuggestedPost
    where
        C: Into<ChatId>,
    {
        Self::DeclineSuggestedPost::new(
            self.clone(),
            payloads::DeclineSuggestedPost::new(chat_id, message_id),
        )
    }

    type EditEphemeralMessageReplyMarkup = JsonRequest<payloads::EditEphemeralMessageReplyMarkup>;

    fn edit_ephemeral_message_reply_markup<C>(
        &self,
        chat_id: C,
        receiver_user_id: UserId,
        ephemeral_message_id: EphemeralMessageId,
    ) -> Self::EditEphemeralMessageReplyMarkup
    where
        C: Into<Recipient>,
    {
        Self::EditEphemeralMessageReplyMarkup::new(
            self.clone(),
            payloads::EditEphemeralMessageReplyMarkup::new(
                chat_id,
                receiver_user_id,
                ephemeral_message_id,
            ),
        )
    }

    type DeleteEphemeralMessage = JsonRequest<payloads::DeleteEphemeralMessage>;

    fn delete_ephemeral_message<C>(
        &self,
        chat_id: C,
        receiver_user_id: UserId,
        ephemeral_message_id: EphemeralMessageId,
    ) -> Self::DeleteEphemeralMessage
    where
        C: Into<Recipient>,
    {
        Self::DeleteEphemeralMessage::new(
            self.clone(),
            payloads::DeleteEphemeralMessage::new(chat_id, receiver_user_id, ephemeral_message_id),
        )
    }

    type DeleteMessage = JsonRequest<payloads::DeleteMessage>;

    fn delete_message<C>(&self, chat_id: C, message_id: MessageId) -> Self::DeleteMessage
    where
        C: Into<Recipient>,
    {
        Self::DeleteMessage::new(self.clone(), payloads::DeleteMessage::new(chat_id, message_id))
    }

    type DeleteMessages = JsonRequest<payloads::DeleteMessages>;
    fn delete_messages<C, M>(&self, chat_id: C, message_ids: M) -> Self::DeleteMessages
    where
        C: Into<Recipient>,
        M: IntoIterator<Item = MessageId>,
    {
        Self::DeleteMessages::new(self.clone(), payloads::DeleteMessages::new(chat_id, message_ids))
    }

    type SendSticker = MultipartRequest<payloads::SendSticker>;

    fn send_sticker<C>(&self, chat_id: C, sticker: InputFile) -> Self::SendSticker
    where
        C: Into<Recipient>,
    {
        Self::SendSticker::new(self.clone(), payloads::SendSticker::new(chat_id, sticker))
    }

    type GetStickerSet = JsonRequest<payloads::GetStickerSet>;

    fn get_sticker_set<N>(&self, name: N) -> Self::GetStickerSet
    where
        N: Into<String>,
    {
        Self::GetStickerSet::new(self.clone(), payloads::GetStickerSet::new(name))
    }

    type GetCustomEmojiStickers = JsonRequest<payloads::GetCustomEmojiStickers>;

    fn get_custom_emoji_stickers<C>(&self, custom_emoji_ids: C) -> Self::GetCustomEmojiStickers
    where
        C: IntoIterator<Item = CustomEmojiId>,
    {
        Self::GetCustomEmojiStickers::new(
            self.clone(),
            payloads::GetCustomEmojiStickers::new(custom_emoji_ids),
        )
    }

    type UploadStickerFile = MultipartRequest<payloads::UploadStickerFile>;

    fn upload_sticker_file(
        &self,
        user_id: UserId,
        sticker: InputFile,
        sticker_format: StickerFormat,
    ) -> Self::UploadStickerFile {
        Self::UploadStickerFile::new(
            self.clone(),
            payloads::UploadStickerFile::new(user_id, sticker, sticker_format),
        )
    }

    type CreateNewStickerSet = MultipartRequest<payloads::CreateNewStickerSet>;

    fn create_new_sticker_set<N, T, S>(
        &self,
        user_id: UserId,
        name: N,
        title: T,
        stickers: S,
    ) -> Self::CreateNewStickerSet
    where
        N: Into<String>,
        T: Into<String>,
        S: IntoIterator<Item = InputSticker>,
    {
        Self::CreateNewStickerSet::new(
            self.clone(),
            payloads::CreateNewStickerSet::new(user_id, name, title, stickers),
        )
    }

    type AddStickerToSet = MultipartRequest<payloads::AddStickerToSet>;

    fn add_sticker_to_set<N>(
        &self,
        user_id: UserId,
        name: N,
        sticker: InputSticker,
    ) -> Self::AddStickerToSet
    where
        N: Into<String>,
    {
        Self::AddStickerToSet::new(
            self.clone(),
            payloads::AddStickerToSet::new(user_id, name, sticker),
        )
    }

    type SetStickerPositionInSet = JsonRequest<payloads::SetStickerPositionInSet>;

    fn set_sticker_position_in_set<S>(
        &self,
        sticker: S,
        position: u32,
    ) -> Self::SetStickerPositionInSet
    where
        S: Into<String>,
    {
        Self::SetStickerPositionInSet::new(
            self.clone(),
            payloads::SetStickerPositionInSet::new(sticker, position),
        )
    }

    type DeleteStickerFromSet = JsonRequest<payloads::DeleteStickerFromSet>;

    fn delete_sticker_from_set<S>(&self, sticker: S) -> Self::DeleteStickerFromSet
    where
        S: Into<String>,
    {
        Self::DeleteStickerFromSet::new(self.clone(), payloads::DeleteStickerFromSet::new(sticker))
    }

    type ReplaceStickerInSet = JsonRequest<payloads::ReplaceStickerInSet>;

    fn replace_sticker_in_set<N, O>(
        &self,
        user_id: UserId,
        name: N,
        old_sticker: O,
        sticker: InputSticker,
    ) -> Self::ReplaceStickerInSet
    where
        N: Into<String>,
        O: Into<String>,
    {
        Self::ReplaceStickerInSet::new(
            self.clone(),
            payloads::ReplaceStickerInSet {
                user_id,
                name: name.into(),
                old_sticker: old_sticker.into(),
                sticker,
            },
        )
    }

    type SetStickerSetThumbnail = MultipartRequest<payloads::SetStickerSetThumbnail>;

    fn set_sticker_set_thumbnail<N>(
        &self,
        name: N,
        user_id: UserId,
        format: StickerFormat,
    ) -> Self::SetStickerSetThumbnail
    where
        N: Into<String>,
    {
        Self::SetStickerSetThumbnail::new(
            self.clone(),
            payloads::SetStickerSetThumbnail::new(name, user_id, format),
        )
    }

    type SetCustomEmojiStickerSetThumbnail =
        JsonRequest<payloads::SetCustomEmojiStickerSetThumbnail>;

    fn set_custom_emoji_sticker_set_thumbnail<N>(
        &self,
        name: N,
    ) -> Self::SetCustomEmojiStickerSetThumbnail
    where
        N: Into<String>,
    {
        Self::SetCustomEmojiStickerSetThumbnail::new(
            self.clone(),
            payloads::SetCustomEmojiStickerSetThumbnail::new(name),
        )
    }

    type SetStickerSetTitle = JsonRequest<payloads::SetStickerSetTitle>;

    fn set_sticker_set_title<N, T>(&self, name: N, title: T) -> Self::SetStickerSetTitle
    where
        N: Into<String>,
        T: Into<String>,
    {
        Self::SetStickerSetTitle::new(self.clone(), payloads::SetStickerSetTitle::new(name, title))
    }

    type DeleteStickerSet = JsonRequest<payloads::DeleteStickerSet>;

    fn delete_sticker_set<N>(&self, name: N) -> Self::DeleteStickerSet
    where
        N: Into<String>,
    {
        Self::DeleteStickerSet::new(self.clone(), payloads::DeleteStickerSet::new(name))
    }

    type SetStickerEmojiList = JsonRequest<payloads::SetStickerEmojiList>;

    fn set_sticker_emoji_list<S, E>(&self, sticker: S, emoji_list: E) -> Self::SetStickerEmojiList
    where
        S: Into<String>,
        E: IntoIterator<Item = String>,
    {
        Self::SetStickerEmojiList::new(
            self.clone(),
            payloads::SetStickerEmojiList::new(sticker, emoji_list),
        )
    }

    type SetStickerKeywords = JsonRequest<payloads::SetStickerKeywords>;

    fn set_sticker_keywords<S>(&self, sticker: S) -> Self::SetStickerKeywords
    where
        S: Into<String>,
    {
        Self::SetStickerKeywords::new(self.clone(), payloads::SetStickerKeywords::new(sticker))
    }

    type SetStickerMaskPosition = JsonRequest<payloads::SetStickerMaskPosition>;

    fn set_sticker_mask_position<S>(&self, sticker: S) -> Self::SetStickerMaskPosition
    where
        S: Into<String>,
    {
        Self::SetStickerMaskPosition::new(
            self.clone(),
            payloads::SetStickerMaskPosition::new(sticker),
        )
    }

    type GetAvailableGifts = JsonRequest<payloads::GetAvailableGifts>;

    fn get_available_gifts(&self) -> Self::GetAvailableGifts {
        Self::GetAvailableGifts::new(self.clone(), payloads::GetAvailableGifts::new())
    }

    type SendGift = JsonRequest<payloads::SendGift>;

    fn send_gift(&self, user_id: UserId, gift_id: GiftId) -> Self::SendGift {
        Self::SendGift::new(self.clone(), payloads::SendGift::new(user_id, gift_id))
    }

    type SendGiftChat = JsonRequest<payloads::SendGiftChat>;

    fn send_gift_chat<C>(&self, chat_id: C, gift_id: GiftId) -> Self::SendGiftChat
    where
        C: Into<Recipient>,
    {
        Self::SendGiftChat::new(self.clone(), payloads::SendGiftChat::new(chat_id, gift_id))
    }

    type GetUserGifts = JsonRequest<payloads::GetUserGifts>;

    fn get_user_gifts(&self, user_id: UserId) -> Self::GetUserGifts {
        Self::GetUserGifts::new(self.clone(), payloads::GetUserGifts::new(user_id))
    }

    type GetChatGifts = JsonRequest<payloads::GetChatGifts>;

    fn get_chat_gifts<C>(&self, chat_id: C) -> Self::GetChatGifts
    where
        C: Into<Recipient>,
    {
        Self::GetChatGifts::new(self.clone(), payloads::GetChatGifts::new(chat_id))
    }

    type GiftPremiumSubscription = JsonRequest<payloads::GiftPremiumSubscription>;

    fn gift_premium_subscription(
        &self,
        user_id: UserId,
        month_count: u8,
        star_count: u32,
    ) -> Self::GiftPremiumSubscription {
        Self::GiftPremiumSubscription::new(
            self.clone(),
            payloads::GiftPremiumSubscription::new(user_id, month_count, star_count),
        )
    }

    type VerifyUser = JsonRequest<payloads::VerifyUser>;

    fn verify_user(&self, user_id: UserId) -> Self::VerifyUser {
        Self::VerifyUser::new(self.clone(), payloads::VerifyUser::new(user_id))
    }

    type VerifyChat = JsonRequest<payloads::VerifyChat>;

    fn verify_chat<C>(&self, chat_id: C) -> Self::VerifyChat
    where
        C: Into<Recipient>,
    {
        Self::VerifyChat::new(self.clone(), payloads::VerifyChat::new(chat_id))
    }

    type RemoveUserVerification = JsonRequest<payloads::RemoveUserVerification>;

    fn remove_user_verification(&self, user_id: UserId) -> Self::RemoveUserVerification {
        Self::RemoveUserVerification::new(
            self.clone(),
            payloads::RemoveUserVerification::new(user_id),
        )
    }

    type RemoveChatVerification = JsonRequest<payloads::RemoveChatVerification>;

    fn remove_chat_verification<C>(&self, chat_id: C) -> Self::RemoveChatVerification
    where
        C: Into<Recipient>,
    {
        Self::RemoveChatVerification::new(
            self.clone(),
            payloads::RemoveChatVerification::new(chat_id),
        )
    }

    type ReadBusinessMessage = JsonRequest<payloads::ReadBusinessMessage>;

    fn read_business_message<C>(
        &self,
        business_connection_id: BusinessConnectionId,
        chat_id: C,
        message_id: MessageId,
    ) -> Self::ReadBusinessMessage
    where
        C: Into<ChatId>,
    {
        Self::ReadBusinessMessage::new(
            self.clone(),
            payloads::ReadBusinessMessage::new(business_connection_id, chat_id, message_id),
        )
    }

    type DeleteBusinessMessages = JsonRequest<payloads::DeleteBusinessMessages>;

    fn delete_business_messages<M>(
        &self,
        business_connection_id: BusinessConnectionId,
        message_ids: M,
    ) -> Self::DeleteBusinessMessages
    where
        M: IntoIterator<Item = MessageId>,
    {
        Self::DeleteBusinessMessages::new(
            self.clone(),
            payloads::DeleteBusinessMessages::new(business_connection_id, message_ids),
        )
    }

    type SetBusinessAccountName = JsonRequest<payloads::SetBusinessAccountName>;

    fn set_business_account_name<F>(
        &self,
        business_connection_id: BusinessConnectionId,
        first_name: F,
    ) -> Self::SetBusinessAccountName
    where
        F: Into<String>,
    {
        Self::SetBusinessAccountName::new(
            self.clone(),
            payloads::SetBusinessAccountName::new(business_connection_id, first_name),
        )
    }

    type SetBusinessAccountUsername = JsonRequest<payloads::SetBusinessAccountUsername>;

    fn set_business_account_username(
        &self,
        business_connection_id: BusinessConnectionId,
    ) -> Self::SetBusinessAccountUsername {
        Self::SetBusinessAccountUsername::new(
            self.clone(),
            payloads::SetBusinessAccountUsername::new(business_connection_id),
        )
    }

    type SetBusinessAccountBio = JsonRequest<payloads::SetBusinessAccountBio>;

    fn set_business_account_bio(
        &self,
        business_connection_id: BusinessConnectionId,
    ) -> Self::SetBusinessAccountBio {
        Self::SetBusinessAccountBio::new(
            self.clone(),
            payloads::SetBusinessAccountBio::new(business_connection_id),
        )
    }

    type SetBusinessAccountProfilePhoto =
        MultipartRequest<payloads::SetBusinessAccountProfilePhoto>;

    fn set_business_account_profile_photo(
        &self,
        business_connection_id: BusinessConnectionId,
        photo: InputProfilePhoto,
    ) -> Self::SetBusinessAccountProfilePhoto {
        Self::SetBusinessAccountProfilePhoto::new(
            self.clone(),
            payloads::SetBusinessAccountProfilePhoto::new(business_connection_id, photo),
        )
    }

    type RemoveBusinessAccountProfilePhoto =
        JsonRequest<payloads::RemoveBusinessAccountProfilePhoto>;

    fn remove_business_account_profile_photo(
        &self,
        business_connection_id: BusinessConnectionId,
    ) -> Self::RemoveBusinessAccountProfilePhoto {
        Self::RemoveBusinessAccountProfilePhoto::new(
            self.clone(),
            payloads::RemoveBusinessAccountProfilePhoto::new(business_connection_id),
        )
    }

    type SetBusinessAccountGiftSettings = JsonRequest<payloads::SetBusinessAccountGiftSettings>;

    fn set_business_account_gift_settings(
        &self,
        business_connection_id: BusinessConnectionId,
        show_gift_button: bool,
        accepted_gift_types: AcceptedGiftTypes,
    ) -> Self::SetBusinessAccountGiftSettings {
        Self::SetBusinessAccountGiftSettings::new(
            self.clone(),
            payloads::SetBusinessAccountGiftSettings::new(
                business_connection_id,
                show_gift_button,
                accepted_gift_types,
            ),
        )
    }

    type GetBusinessAccountStarBalance = JsonRequest<payloads::GetBusinessAccountStarBalance>;

    fn get_business_account_star_balance(
        &self,
        business_connection_id: BusinessConnectionId,
    ) -> Self::GetBusinessAccountStarBalance {
        Self::GetBusinessAccountStarBalance::new(
            self.clone(),
            payloads::GetBusinessAccountStarBalance::new(business_connection_id),
        )
    }

    type TransferBusinessAccountStars = JsonRequest<payloads::TransferBusinessAccountStars>;

    fn transfer_business_account_stars(
        &self,
        business_connection_id: BusinessConnectionId,
        star_count: u32,
    ) -> Self::TransferBusinessAccountStars {
        Self::TransferBusinessAccountStars::new(
            self.clone(),
            payloads::TransferBusinessAccountStars::new(business_connection_id, star_count),
        )
    }

    type GetBusinessAccountGifts = JsonRequest<payloads::GetBusinessAccountGifts>;

    fn get_business_account_gifts(
        &self,
        business_connection_id: BusinessConnectionId,
    ) -> Self::GetBusinessAccountGifts {
        Self::GetBusinessAccountGifts::new(
            self.clone(),
            payloads::GetBusinessAccountGifts::new(business_connection_id),
        )
    }

    type ConvertGiftToStars = JsonRequest<payloads::ConvertGiftToStars>;

    fn convert_gift_to_stars(
        &self,
        business_connection_id: BusinessConnectionId,
        owned_gift_id: OwnedGiftId,
    ) -> Self::ConvertGiftToStars {
        Self::ConvertGiftToStars::new(
            self.clone(),
            payloads::ConvertGiftToStars::new(business_connection_id, owned_gift_id),
        )
    }

    type UpgradeGift = JsonRequest<payloads::UpgradeGift>;

    fn upgrade_gift(
        &self,
        business_connection_id: BusinessConnectionId,
        owned_gift_id: OwnedGiftId,
    ) -> Self::UpgradeGift {
        Self::UpgradeGift::new(
            self.clone(),
            payloads::UpgradeGift::new(business_connection_id, owned_gift_id),
        )
    }

    type TransferGift = JsonRequest<payloads::TransferGift>;

    fn transfer_gift<C>(
        &self,
        business_connection_id: BusinessConnectionId,
        owned_gift_id: OwnedGiftId,
        new_owner_chat_id: C,
    ) -> Self::TransferGift
    where
        C: Into<ChatId>,
    {
        Self::TransferGift::new(
            self.clone(),
            payloads::TransferGift::new(business_connection_id, owned_gift_id, new_owner_chat_id),
        )
    }

    type PostStory = JsonRequest<payloads::PostStory>;

    fn post_story(
        &self,
        business_connection_id: BusinessConnectionId,
        content: InputStoryContent,
        active_period: Seconds,
    ) -> Self::PostStory {
        Self::PostStory::new(
            self.clone(),
            payloads::PostStory::new(business_connection_id, content, active_period),
        )
    }

    type EditStory = JsonRequest<payloads::EditStory>;

    fn edit_story(
        &self,
        business_connection_id: BusinessConnectionId,
        story_id: StoryId,
        content: InputStoryContent,
    ) -> Self::EditStory {
        Self::EditStory::new(
            self.clone(),
            payloads::EditStory::new(business_connection_id, story_id, content),
        )
    }

    type DeleteStory = JsonRequest<payloads::DeleteStory>;

    fn delete_story(
        &self,
        business_connection_id: BusinessConnectionId,
        story_id: StoryId,
    ) -> Self::DeleteStory {
        Self::DeleteStory::new(
            self.clone(),
            payloads::DeleteStory::new(business_connection_id, story_id),
        )
    }

    type RepostStory = JsonRequest<payloads::RepostStory>;

    fn repost_story<F>(
        &self,
        business_connection_id: BusinessConnectionId,
        from_chat_id: F,
        from_story_id: StoryId,
        active_period: Seconds,
    ) -> Self::RepostStory
    where
        F: Into<ChatId>,
    {
        Self::RepostStory::new(
            self.clone(),
            payloads::RepostStory::new(
                business_connection_id,
                from_chat_id,
                from_story_id,
                active_period,
            ),
        )
    }

    type SendInvoice = JsonRequest<payloads::SendInvoice>;

    fn send_invoice<Ch, T, D, Pa, C, Pri>(
        &self,
        chat_id: Ch,
        title: T,
        description: D,
        payload: Pa,
        currency: C,
        prices: Pri,
    ) -> Self::SendInvoice
    where
        Ch: Into<Recipient>,
        T: Into<String>,
        D: Into<String>,
        Pa: Into<String>,
        C: Into<String>,
        Pri: IntoIterator<Item = LabeledPrice>,
    {
        Self::SendInvoice::new(
            self.clone(),
            payloads::SendInvoice::new(chat_id, title, description, payload, currency, prices),
        )
    }

    type CreateInvoiceLink = JsonRequest<payloads::CreateInvoiceLink>;

    fn create_invoice_link<T, D, Pa, C, Pri>(
        &self,
        title: T,
        description: D,
        payload: Pa,
        currency: C,
        prices: Pri,
    ) -> Self::CreateInvoiceLink
    where
        T: Into<String>,
        D: Into<String>,
        Pa: Into<String>,
        C: Into<String>,
        Pri: IntoIterator<Item = LabeledPrice>,
    {
        Self::CreateInvoiceLink::new(
            self.clone(),
            payloads::CreateInvoiceLink::new(title, description, payload, currency, prices),
        )
    }

    type AnswerShippingQuery = JsonRequest<payloads::AnswerShippingQuery>;

    fn answer_shipping_query(
        &self,
        shipping_query_id: ShippingQueryId,
        ok: bool,
    ) -> Self::AnswerShippingQuery {
        Self::AnswerShippingQuery::new(
            self.clone(),
            payloads::AnswerShippingQuery::new(shipping_query_id, ok),
        )
    }

    type AnswerPreCheckoutQuery = JsonRequest<payloads::AnswerPreCheckoutQuery>;

    fn answer_pre_checkout_query(
        &self,
        pre_checkout_query_id: PreCheckoutQueryId,
        ok: bool,
    ) -> Self::AnswerPreCheckoutQuery {
        Self::AnswerPreCheckoutQuery::new(
            self.clone(),
            payloads::AnswerPreCheckoutQuery::new(pre_checkout_query_id, ok),
        )
    }

    type GetMyStarBalance = JsonRequest<payloads::GetMyStarBalance>;

    fn get_my_star_balance(&self) -> Self::GetMyStarBalance {
        Self::GetMyStarBalance::new(self.clone(), payloads::GetMyStarBalance::new())
    }

    type GetStarTransactions = JsonRequest<payloads::GetStarTransactions>;

    fn get_star_transactions(&self) -> Self::GetStarTransactions {
        Self::GetStarTransactions::new(self.clone(), payloads::GetStarTransactions::new())
    }

    type RefundStarPayment = JsonRequest<payloads::RefundStarPayment>;

    fn refund_star_payment(
        &self,
        user_id: UserId,
        telegram_payment_charge_id: TelegramTransactionId,
    ) -> Self::RefundStarPayment {
        Self::RefundStarPayment::new(
            self.clone(),
            payloads::RefundStarPayment::new(user_id, telegram_payment_charge_id),
        )
    }

    type EditUserStarSubscription = JsonRequest<payloads::EditUserStarSubscription>;

    fn edit_user_star_subscription(
        &self,
        user_id: UserId,
        telegram_payment_charge_id: TelegramTransactionId,
        is_canceled: bool,
    ) -> Self::EditUserStarSubscription {
        Self::EditUserStarSubscription::new(
            self.clone(),
            payloads::EditUserStarSubscription::new(
                user_id,
                telegram_payment_charge_id,
                is_canceled,
            ),
        )
    }

    type SetPassportDataErrors = JsonRequest<payloads::SetPassportDataErrors>;

    fn set_passport_data_errors<E>(&self, user_id: UserId, errors: E) -> Self::SetPassportDataErrors
    where
        E: IntoIterator<Item = crate::types::PassportElementError>,
    {
        Self::SetPassportDataErrors::new(
            self.clone(),
            payloads::SetPassportDataErrors::new(user_id, errors),
        )
    }

    type SendGame = JsonRequest<payloads::SendGame>;

    fn send_game<C, G>(&self, chat_id: C, game_short_name: G) -> Self::SendGame
    where
        C: Into<ChatId>,
        G: Into<String>,
    {
        Self::SendGame::new(self.clone(), payloads::SendGame::new(chat_id, game_short_name))
    }

    type SetGameScore = JsonRequest<payloads::SetGameScore>;

    fn set_game_score(
        &self,
        user_id: UserId,
        score: u64,
        chat_id: u32,
        message_id: MessageId,
    ) -> Self::SetGameScore {
        Self::SetGameScore::new(
            self.clone(),
            payloads::SetGameScore::new(user_id, score, chat_id, message_id),
        )
    }

    type SetGameScoreInline = JsonRequest<payloads::SetGameScoreInline>;

    fn set_game_score_inline<I>(
        &self,
        user_id: UserId,
        score: u64,
        inline_message_id: I,
    ) -> Self::SetGameScoreInline
    where
        I: Into<String>,
    {
        Self::SetGameScoreInline::new(
            self.clone(),
            payloads::SetGameScoreInline::new(user_id, score, inline_message_id),
        )
    }

    type GetGameHighScores = JsonRequest<payloads::GetGameHighScores>;

    fn get_game_high_scores<T>(&self, user_id: UserId, target: T) -> Self::GetGameHighScores
    where
        T: Into<crate::types::TargetMessage>,
    {
        Self::GetGameHighScores::new(
            self.clone(),
            payloads::GetGameHighScores::new(user_id, target),
        )
    }

    type LogOut = JsonRequest<payloads::LogOut>;

    fn log_out(&self) -> Self::LogOut {
        Self::LogOut::new(self.clone(), payloads::LogOut::new())
    }

    type Close = JsonRequest<payloads::Close>;

    fn close(&self) -> Self::Close {
        Self::Close::new(self.clone(), payloads::Close::new())
    }

    type CopyMessage = JsonRequest<payloads::CopyMessage>;

    fn copy_message<C, F>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_id: MessageId,
    ) -> Self::CopyMessage
    where
        C: Into<Recipient>,
        F: Into<Recipient>,
    {
        Self::CopyMessage::new(
            self.clone(),
            payloads::CopyMessage::new(chat_id, from_chat_id, message_id),
        )
    }

    type CopyMessages = JsonRequest<payloads::CopyMessages>;
    fn copy_messages<C, F, M>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_ids: M,
    ) -> Self::CopyMessages
    where
        C: Into<Recipient>,
        F: Into<Recipient>,
        M: IntoIterator<Item = MessageId>,
    {
        Self::CopyMessages::new(
            self.clone(),
            payloads::CopyMessages::new(chat_id, from_chat_id, message_ids),
        )
    }

    type UnpinAllChatMessages = JsonRequest<payloads::UnpinAllChatMessages>;

    fn unpin_all_chat_messages<C>(&self, chat_id: C) -> Self::UnpinAllChatMessages
    where
        C: Into<Recipient>,
    {
        Self::UnpinAllChatMessages::new(self.clone(), payloads::UnpinAllChatMessages::new(chat_id))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        payloads::{
            CopyMessageSetters, DeleteAllMessageReactionsSetters, DeleteMessageReactionSetters,
            ForwardMessageSetters, GetBusinessAccountGiftsSetters, GetChatAdministratorsSetters,
            GetChatGiftsSetters, GetUserGiftsSetters, GetUserProfileAudiosSetters,
            PromoteChatMemberSetters, RepostStorySetters, SendLivePhotoSetters,
            SendMessageDraftSetters, SendPollSetters, SetChatMemberTagSetters,
            SetManagedBotAccessSettingsSetters,
        },
        prelude::Requester,
        requests::HasPayload,
        types::{
            BusinessConnectionId, ButtonRequest, ChatId, ChatPermissions, DraftId, EffectId,
            GuestQueryId, InlineQueryResult, InlineQueryResultArticle, InputFile,
            InputMediaLivePhoto, InputMediaPhoto, InputMediaSticker, InputMessageContent,
            InputMessageContentText, InputPollMedia, InputPollOption, InputProfilePhoto,
            InputProfilePhotoAnimated, InputProfilePhotoStatic, KeyboardButton,
            KeyboardButtonRequestManagedBot, MessageEntity, MessageId, ParseMode, PollType,
            ReplyParameters, RequestId, Seconds, StoryId, ThreadId, UserId,
        },
        Bot,
    };

    #[test]
    fn bot_api_9_3_payload_serialization() {
        let bot = Bot::new("test-token");

        let draft = bot
            .send_message_draft(ChatId(100), DraftId(42), "Partial text")
            .message_thread_id(ThreadId(MessageId(7)))
            .parse_mode(ParseMode::Html);
        assert_eq!(
            serde_json::to_value(draft.payload_ref()).unwrap(),
            json!({
                "chat_id": 100,
                "message_thread_id": 7,
                "draft_id": 42,
                "text": "Partial text",
                "parse_mode": "HTML"
            })
        );

        let user_gifts = bot
            .get_user_gifts(UserId(99))
            .exclude_unlimited(true)
            .exclude_limited_upgradable(true)
            .exclude_limited_non_upgradable(false)
            .exclude_from_blockchain(true)
            .exclude_unique(false)
            .sort_by_price(true)
            .offset("cursor-1")
            .limit(50);
        assert_eq!(
            serde_json::to_value(user_gifts.payload_ref()).unwrap(),
            json!({
                "user_id": 99,
                "exclude_unlimited": true,
                "exclude_limited_upgradable": true,
                "exclude_limited_non_upgradable": false,
                "exclude_from_blockchain": true,
                "exclude_unique": false,
                "sort_by_price": true,
                "offset": "cursor-1",
                "limit": 50
            })
        );

        let chat_gifts = bot
            .get_chat_gifts("@channel_name".to_owned())
            .exclude_unsaved(true)
            .exclude_saved(false)
            .exclude_from_blockchain(true)
            .limit(25);
        assert_eq!(
            serde_json::to_value(chat_gifts.payload_ref()).unwrap(),
            json!({
                "chat_id": "@channel_name",
                "exclude_unsaved": true,
                "exclude_saved": false,
                "exclude_from_blockchain": true,
                "limit": 25
            })
        );

        let biz_gifts = bot
            .get_business_account_gifts(BusinessConnectionId("biz-1".to_owned()))
            .exclude_limited_upgradable(true)
            .exclude_limited_non_upgradable(true)
            .exclude_from_blockchain(true);
        assert_eq!(
            serde_json::to_value(biz_gifts.payload_ref()).unwrap(),
            json!({
                "business_connection_id": "biz-1",
                "exclude_limited_upgradable": true,
                "exclude_limited_non_upgradable": true,
                "exclude_from_blockchain": true
            })
        );

        let repost = bot
            .repost_story(
                BusinessConnectionId("biz-2".to_owned()),
                ChatId(555),
                StoryId(88),
                Seconds::from_seconds(86400),
            )
            .post_to_chat_page(true)
            .protect_content(true);
        assert_eq!(
            serde_json::to_value(repost.payload_ref()).unwrap(),
            json!({
                "business_connection_id": "biz-2",
                "from_chat_id": 555,
                "from_story_id": 88,
                "active_period": 86400,
                "post_to_chat_page": true,
                "protect_content": true
            })
        );

        let forward = bot
            .forward_message(ChatId(1), ChatId(2), MessageId(3))
            .message_effect_id(EffectId("effect-fwd".to_owned()));
        assert_eq!(
            serde_json::to_value(forward.payload_ref()).unwrap(),
            json!({
                "chat_id": 1,
                "from_chat_id": 2,
                "message_id": 3,
                "message_effect_id": "effect-fwd"
            })
        );

        let copy = bot
            .copy_message(ChatId(1), ChatId(2), MessageId(3))
            .message_effect_id(EffectId("effect-cpy".to_owned()));
        assert_eq!(
            serde_json::to_value(copy.payload_ref()).unwrap(),
            json!({
                "chat_id": 1,
                "from_chat_id": 2,
                "message_id": 3,
                "message_effect_id": "effect-cpy"
            })
        );
    }

    #[test]
    fn bot_api_9_4_payload_serialization() {
        let bot = Bot::new("test-token");

        let profile_audios = bot.get_user_profile_audios(UserId(42)).offset(5).limit(20);
        assert_eq!(
            serde_json::to_value(profile_audios.payload_ref()).unwrap(),
            json!({
                "user_id": 42,
                "offset": 5,
                "limit": 20
            })
        );

        let static_file = InputFile::memory(b"jpg-bytes".to_vec());
        let static_id = static_file.id().to_owned();
        let set_static =
            bot.set_my_profile_photo(InputProfilePhoto::Static(InputProfilePhotoStatic {
                photo: static_file,
            }));
        assert_eq!(
            serde_json::to_value(set_static.payload_ref()).unwrap(),
            json!({
                "photo": {
                    "type": "static",
                    "photo": format!("attach://{static_id}")
                }
            })
        );

        let anim_file = InputFile::memory(b"mp4-bytes".to_vec());
        let anim_id = anim_file.id().to_owned();
        let set_anim =
            bot.set_my_profile_photo(InputProfilePhoto::Animated(InputProfilePhotoAnimated {
                animation: anim_file,
                main_frame_timestamp: Some(0.5),
            }));
        assert_eq!(
            serde_json::to_value(set_anim.payload_ref()).unwrap(),
            json!({
                "photo": {
                    "type": "animated",
                    "animation": format!("attach://{anim_id}"),
                    "main_frame_timestamp": 0.5
                }
            })
        );

        let remove_photo = bot.remove_my_profile_photo();
        assert_eq!(serde_json::to_value(remove_photo.payload_ref()).unwrap(), json!({}));
    }

    #[test]
    fn bot_api_9_5_payload_serialization() {
        let bot = Bot::new("test-token");

        let set_tag = bot.set_chat_member_tag(ChatId(-1001), UserId(77)).tag("core-dev");
        assert_eq!(
            serde_json::to_value(set_tag.payload_ref()).unwrap(),
            json!({
                "chat_id": -1001,
                "user_id": 77,
                "tag": "core-dev"
            })
        );

        let clear_tag = bot.set_chat_member_tag("@mygroup".to_owned(), UserId(77));
        assert_eq!(
            serde_json::to_value(clear_tag.payload_ref()).unwrap(),
            json!({
                "chat_id": "@mygroup",
                "user_id": 77
            })
        );

        let promote = bot.promote_chat_member(ChatId(-1001), UserId(77)).can_manage_tags(true);
        assert_eq!(
            serde_json::to_value(promote.payload_ref()).unwrap(),
            json!({
                "chat_id": -1001,
                "user_id": 77,
                "can_manage_tags": true
            })
        );

        let perms_without_tag =
            bot.set_chat_permissions(ChatId(-1001), ChatPermissions::PIN_MESSAGES);
        assert_eq!(
            serde_json::to_value(perms_without_tag.payload_ref()).unwrap()["permissions"]
                ["can_edit_tag"],
            json!(false)
        );

        let perms_with_tag = bot.set_chat_permissions(
            ChatId(-1001),
            ChatPermissions::PIN_MESSAGES | ChatPermissions::EDIT_TAG,
        );
        assert_eq!(
            serde_json::to_value(perms_with_tag.payload_ref()).unwrap()["permissions"]
                ["can_edit_tag"],
            json!(true)
        );

        let restrict_without_tag =
            bot.restrict_chat_member(ChatId(-1001), UserId(77), ChatPermissions::PIN_MESSAGES);
        assert_eq!(
            serde_json::to_value(restrict_without_tag.payload_ref()).unwrap()["permissions"]
                ["can_edit_tag"],
            json!(false)
        );

        let restrict_with_tag = bot.restrict_chat_member(
            ChatId(-1001),
            UserId(77),
            ChatPermissions::PIN_MESSAGES | ChatPermissions::EDIT_TAG,
        );
        assert_eq!(
            serde_json::to_value(restrict_with_tag.payload_ref()).unwrap()["permissions"]
                ["can_edit_tag"],
            json!(true)
        );
    }

    #[test]
    fn bot_api_9_6_payload_serialization() {
        let bot = Bot::new("test-token");

        let get_token = bot.get_managed_bot_token(UserId(9901));
        assert_eq!(
            serde_json::to_value(get_token.payload_ref()).unwrap(),
            json!({ "user_id": 9901 })
        );

        let replace_token = bot.replace_managed_bot_token(UserId(9901));
        assert_eq!(
            serde_json::to_value(replace_token.payload_ref()).unwrap(),
            json!({ "user_id": 9901 })
        );

        let prepared_btn = bot.save_prepared_keyboard_button(
            UserId(42),
            KeyboardButton::new("New Sub-Bot").request(ButtonRequest::RequestManagedBot(
                KeyboardButtonRequestManagedBot::new(RequestId(7))
                    .suggested_name("Helper")
                    .suggested_username("my_helper_bot"),
            )),
        );
        assert_eq!(
            serde_json::to_value(prepared_btn.payload_ref()).unwrap(),
            json!({
                "user_id": 42,
                "button": {
                    "text": "New Sub-Bot",
                    "request_managed_bot": {
                        "request_id": 7,
                        "suggested_name": "Helper",
                        "suggested_username": "my_helper_bot"
                    }
                }
            })
        );

        let send_poll = bot
            .send_poll(
                ChatId(-1001),
                "Quiz question?",
                [
                    InputPollOption::new("Opt 0"),
                    InputPollOption::new("Opt 1"),
                    InputPollOption::new("Opt 2"),
                ],
            )
            .type_(PollType::Quiz)
            .allows_multiple_answers(true)
            .allows_revoting(false)
            .shuffle_options(true)
            .allow_adding_options(false)
            .hide_results_until_closes(true)
            .correct_option_ids([0, 2])
            .open_period(2_628_000)
            .description("Multi-correct quiz")
            .description_parse_mode(ParseMode::Html)
            .description_entities([MessageEntity::bold(0, 5)])
            .reply_parameters(ReplyParameters::new(MessageId(10)).poll_option_id("opt-a"));
        assert_eq!(
            serde_json::to_value(send_poll.payload_ref()).unwrap(),
            json!({
                "chat_id": -1001,
                "question": "Quiz question?",
                "options": [
                    { "text": "Opt 0" },
                    { "text": "Opt 1" },
                    { "text": "Opt 2" }
                ],
                "type": "quiz",
                "allows_multiple_answers": true,
                "allows_revoting": false,
                "shuffle_options": true,
                "allow_adding_options": false,
                "hide_results_until_closes": true,
                "correct_option_ids": [0, 2],
                "open_period": 2628000,
                "description": "Multi-correct quiz",
                "description_parse_mode": "HTML",
                "description_entities": [
                    { "type": "bold", "offset": 0, "length": 5 }
                ],
                "reply_parameters": {
                    "message_id": 10,
                    "poll_option_id": "opt-a"
                }
            })
        );
    }

    #[tokio::test]
    async fn bot_api_9_6_managed_bot_endpoints_and_decoding() {
        use std::{
            io::{Read, Write},
            net::TcpListener,
            sync::mpsc,
        };

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = mpsc::channel();
        let responses = vec![
            r#"{"ok":true,"result":"9901:GET_TOKEN_OK"}"#,
            r#"{"ok":true,"result":"9901:REPLACE_TOKEN_OK"}"#,
            r#"{"ok":true,"result":{"id":"prep-kbd-1"}}"#,
        ];

        std::thread::spawn(move || {
            for response_body in responses {
                let (mut stream, _) = listener.accept().unwrap();
                let mut data = Vec::new();
                let mut buf = [0u8; 1024];
                let header_end = loop {
                    let n = stream.read(&mut buf).unwrap();
                    assert!(n > 0);
                    data.extend_from_slice(&buf[..n]);
                    if let Some(pos) = data.windows(4).position(|w| w == b"\r\n\r\n") {
                        break pos + 4;
                    }
                };
                let headers = String::from_utf8_lossy(&data[..header_end]).into_owned();
                let req_line = headers.lines().next().unwrap().to_owned();
                let content_len: usize = headers
                    .lines()
                    .find_map(|line| {
                        let (k, v) = line.split_once(':')?;
                        k.eq_ignore_ascii_case("content-length").then(|| v.trim().parse().ok())?
                    })
                    .unwrap_or(0);
                while data.len() < header_end + content_len {
                    let n = stream.read(&mut buf).unwrap();
                    assert!(n > 0);
                    data.extend_from_slice(&buf[..n]);
                }
                let body: serde_json::Value =
                    serde_json::from_slice(&data[header_end..header_end + content_len]).unwrap();
                tx.send((req_line, body)).unwrap();

                let reply = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: \
                     {}\r\nConnection: close\r\n\r\n{}",
                    response_body.len(),
                    response_body
                );
                stream.write_all(reply.as_bytes()).unwrap();
            }
        });

        let api_url = reqwest::Url::parse(&format!("http://{addr}/")).unwrap();
        let bot = Bot::with_client("test-token", reqwest::Client::new()).set_api_url(api_url);

        let token = bot.get_managed_bot_token(UserId(9901)).await.unwrap();
        assert_eq!(token, "9901:GET_TOKEN_OK");

        let new_token = bot.replace_managed_bot_token(UserId(9901)).await.unwrap();
        assert_eq!(new_token, "9901:REPLACE_TOKEN_OK");

        let prepared = bot
            .save_prepared_keyboard_button(
                UserId(42),
                KeyboardButton::new("Create Bot").request(ButtonRequest::RequestManagedBot(
                    KeyboardButtonRequestManagedBot::new(RequestId(7)),
                )),
            )
            .await
            .unwrap();
        assert_eq!(prepared, crate::types::PreparedKeyboardButton { id: "prep-kbd-1".to_owned() });

        let (line1, body1) = rx.recv().unwrap();
        assert!(line1.starts_with("POST /bottest-token/GetManagedBotToken "), "{line1}");
        assert_eq!(body1, json!({ "user_id": 9901 }));

        let (line2, body2) = rx.recv().unwrap();
        assert!(line2.starts_with("POST /bottest-token/ReplaceManagedBotToken "), "{line2}");
        assert_eq!(body2, json!({ "user_id": 9901 }));

        let (line3, body3) = rx.recv().unwrap();
        assert!(line3.starts_with("POST /bottest-token/SavePreparedKeyboardButton "), "{line3}");
        assert_eq!(
            body3,
            json!({
                "user_id": 42,
                "button": {
                    "text": "Create Bot",
                    "request_managed_bot": { "request_id": 7 }
                }
            })
        );
    }

    #[test]
    fn bot_api_10_0_payload_serialization_and_multipart() {
        use crate::requests::MultipartPayload;

        let bot = Bot::new("test-token");

        let answer_guest = bot.answer_guest_query(
            GuestQueryId::from("gq-1"),
            InlineQueryResult::Article(InlineQueryResultArticle::new(
                "1",
                "Hi",
                InputMessageContent::Text(InputMessageContentText::new("Body")),
            )),
        );
        let guest_val = serde_json::to_value(answer_guest.payload_ref()).unwrap();
        assert_eq!(guest_val["guest_query_id"], "gq-1");
        assert_eq!(guest_val["result"]["type"], "article");

        let admins = bot.get_chat_administrators(ChatId(-1001)).return_bots(true);
        assert_eq!(
            serde_json::to_value(admins.payload_ref()).unwrap(),
            json!({ "chat_id": -1001, "return_bots": true })
        );

        let del_all = bot
            .delete_all_message_reactions(ChatId(-1001))
            .user_id(UserId(11))
            .actor_chat_id(ChatId(-1002));
        assert_eq!(
            serde_json::to_value(del_all.payload_ref()).unwrap(),
            json!({ "chat_id": -1001, "user_id": 11, "actor_chat_id": -1002 })
        );

        let del_one = bot.delete_message_reaction(ChatId(-1001), MessageId(9)).user_id(UserId(11));
        assert_eq!(
            serde_json::to_value(del_one.payload_ref()).unwrap(),
            json!({ "chat_id": -1001, "message_id": 9, "user_id": 11 })
        );

        let empty_draft = bot.send_message_draft(ChatId(42), DraftId(1), "");
        assert_eq!(
            serde_json::to_value(empty_draft.payload_ref()).unwrap(),
            json!({ "chat_id": 42, "draft_id": 1, "text": "" })
        );

        let set_access = bot
            .set_managed_bot_access_settings(UserId(9901), true)
            .added_user_ids([UserId(10), UserId(20)]);
        assert_eq!(
            serde_json::to_value(set_access.payload_ref()).unwrap(),
            json!({
                "user_id": 9901,
                "is_access_restricted": true,
                "added_user_ids": [10, 20]
            })
        );

        let personal_msgs = bot.get_user_personal_chat_messages(UserId(42), 5);
        assert_eq!(
            serde_json::to_value(personal_msgs.payload_ref()).unwrap(),
            json!({ "user_id": 42, "limit": 5 })
        );

        let live_photo_req = bot
            .send_live_photo(
                ChatId(-1001),
                InputFile::memory("video-bytes").file_name("lp.mp4"),
                InputFile::memory("photo-bytes").file_name("lp.jpg"),
            )
            .has_spoiler(true);
        let mut lp_files = Vec::new();
        live_photo_req.payload_ref().copy_files(&mut |f| lp_files.push(f));
        assert_eq!(lp_files.len(), 2);

        let poll_req = bot
            .send_poll(
                ChatId(-1001),
                "Poll Q",
                [InputPollOption::new("Opt").media(InputMediaSticker::new(
                    InputFile::memory("stk-bytes").file_name("s.webp"),
                ))],
            )
            .members_only(true)
            .country_codes(["US".to_owned(), "FT".to_owned()])
            .media(InputPollMedia::LivePhoto(InputMediaLivePhoto::new(
                InputFile::memory("p-v").file_name("pv.mp4"),
                InputFile::memory("p-p").file_name("pp.jpg"),
            )))
            .explanation_media(InputPollMedia::Photo(InputMediaPhoto::new(
                InputFile::memory("exp-bytes").file_name("e.jpg"),
            )));
        let mut poll_files = Vec::new();
        poll_req.payload_ref().copy_files(&mut |f| poll_files.push(f));
        assert_eq!(poll_files.len(), 4);
    }

    #[tokio::test]
    async fn bot_api_10_0_endpoints_over_transport() {
        use crate::requests::Request;
        use std::{
            collections::HashMap,
            io::{Read, Write},
            net::TcpListener,
            sync::mpsc,
        };

        struct TempFileGuard(std::path::PathBuf);
        impl Drop for TempFileGuard {
            fn drop(&mut self) {
                let _ = std::fs::remove_file(&self.0);
            }
        }

        fn parse_multipart_parts(headers: &str, body: &[u8]) -> HashMap<String, Vec<u8>> {
            let boundary = headers
                .lines()
                .find_map(|line| {
                    let (k, v) = line.split_once(':')?;
                    if !k.eq_ignore_ascii_case("content-type") {
                        return None;
                    }
                    v.split(';')
                        .find_map(|part| part.trim().strip_prefix("boundary="))
                        .map(str::to_owned)
                })
                .expect("missing multipart boundary");
            let delim = format!("--{boundary}").into_bytes();
            let mut parts = HashMap::new();
            let mut pos = 0;
            while let Some(rel_start) =
                body[pos..].windows(delim.len()).position(|w| w == delim.as_slice())
            {
                let part_start = pos + rel_start + delim.len();
                pos = part_start;
                if body[part_start..].starts_with(b"--") {
                    break;
                }
                let content_start = if body[part_start..].starts_with(b"\r\n") {
                    part_start + 2
                } else {
                    part_start
                };
                let Some(hdr_end_rel) =
                    body[content_start..].windows(4).position(|w| w == b"\r\n\r\n")
                else {
                    continue;
                };
                let part_headers =
                    String::from_utf8_lossy(&body[content_start..content_start + hdr_end_rel]);
                let data_start = content_start + hdr_end_rel + 4;
                let Some(next_delim_rel) =
                    body[data_start..].windows(delim.len()).position(|w| w == delim.as_slice())
                else {
                    continue;
                };
                let mut data_end = data_start + next_delim_rel;
                if body[data_start..data_end].ends_with(b"\r\n") {
                    data_end -= 2;
                }
                if let Some(name_start) = part_headers.find("name=\"") {
                    let name_rest = &part_headers[name_start + 6..];
                    if let Some(name_end) = name_rest.find('"') {
                        parts.insert(
                            name_rest[..name_end].to_owned(),
                            body[data_start..data_end].to_vec(),
                        );
                    }
                }
                pos = data_start + next_delim_rel;
            }
            parts
        }

        fn resolve_multipart_file<'a>(
            parts: &'a HashMap<String, Vec<u8>>,
            field: &str,
        ) -> &'a [u8] {
            let val = parts.get(field).unwrap_or_else(|| panic!("missing part `{field}`"));
            if let Ok(s) = std::str::from_utf8(val) {
                if let Some(attach) = s.strip_prefix("attach://") {
                    return parts
                        .get(attach)
                        .unwrap_or_else(|| panic!("missing attach part `{attach}`"));
                }
            }
            val
        }

        let live_photo_msg_json = serde_json::json!({
            "ok": true,
            "result": {
                "message_id": 601,
                "chat": { "id": 42, "first_name": "Alice", "type": "private" },
                "date": 1778200000,
                "live_photo": {
                    "file_id": "lp-1",
                    "file_unique_id": "lp-u-1",
                    "width": 640,
                    "height": 480,
                    "duration": 2
                }
            }
        })
        .to_string();

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = mpsc::channel();
        let responses = vec![
            r#"{"ok":true,"result":{"is_access_restricted":true,"added_users":[{"id":12,"is_bot":false,"first_name":"Allowed"}]}}"#.to_owned(),
            r#"{"ok":true,"result":[]}"#.to_owned(),
            r#"{"ok":true,"result":{"inline_message_id":"guest-inline-7"}}"#.to_owned(),
            live_photo_msg_json.clone(),
            live_photo_msg_json,
        ];

        std::thread::spawn(move || {
            for response_body in responses {
                let (mut stream, _) = listener.accept().unwrap();
                let mut data = Vec::new();
                let mut buf = [0u8; 4096];
                let header_end = loop {
                    let n = stream.read(&mut buf).unwrap();
                    assert!(n > 0);
                    data.extend_from_slice(&buf[..n]);
                    if let Some(pos) = data.windows(4).position(|w| w == b"\r\n\r\n") {
                        break pos + 4;
                    }
                };
                let headers = String::from_utf8_lossy(&data[..header_end]).into_owned();
                let req_line = headers.lines().next().unwrap().to_owned();
                let is_chunked = headers.lines().any(|line| {
                    line.split_once(':').is_some_and(|(k, v)| {
                        k.eq_ignore_ascii_case("transfer-encoding")
                            && v.to_ascii_lowercase().contains("chunked")
                    })
                });
                let raw_body = if is_chunked {
                    let mut cursor = header_end;
                    let mut decoded = Vec::new();
                    loop {
                        let line_end = loop {
                            if let Some(pos) = data[cursor..].windows(2).position(|w| w == b"\r\n")
                            {
                                break cursor + pos;
                            }
                            let n = stream.read(&mut buf).unwrap();
                            assert!(n > 0, "unexpected EOF reading chunk size");
                            data.extend_from_slice(&buf[..n]);
                        };
                        let size_str = std::str::from_utf8(&data[cursor..line_end]).unwrap();
                        let size_hex = size_str.split(';').next().unwrap().trim();
                        let chunk_len = usize::from_str_radix(size_hex, 16).unwrap();
                        cursor = line_end + 2;
                        if chunk_len == 0 {
                            while data.len() < cursor + 2 {
                                let n = stream.read(&mut buf).unwrap();
                                if n == 0 {
                                    break;
                                }
                                data.extend_from_slice(&buf[..n]);
                            }
                            break;
                        }
                        while data.len() < cursor + chunk_len + 2 {
                            let n = stream.read(&mut buf).unwrap();
                            assert!(n > 0, "unexpected EOF reading chunk payload");
                            data.extend_from_slice(&buf[..n]);
                        }
                        decoded.extend_from_slice(&data[cursor..cursor + chunk_len]);
                        cursor += chunk_len + 2;
                    }
                    decoded
                } else {
                    let content_len: usize = headers
                        .lines()
                        .find_map(|line| {
                            let (k, v) = line.split_once(':')?;
                            k.eq_ignore_ascii_case("content-length")
                                .then(|| v.trim().parse().ok())?
                        })
                        .unwrap_or(0);
                    while data.len() < header_end + content_len {
                        let n = stream.read(&mut buf).unwrap();
                        assert!(n > 0);
                        data.extend_from_slice(&buf[..n]);
                    }
                    data[header_end..header_end + content_len].to_vec()
                };
                tx.send((req_line, headers, raw_body)).unwrap();

                let reply = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: \
                     {}\r\nConnection: close\r\n\r\n{}",
                    response_body.len(),
                    response_body
                );
                stream.write_all(reply.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        });

        let api_url = reqwest::Url::parse(&format!("http://{addr}/")).unwrap();
        let bot = Bot::with_client("test-token", reqwest::Client::new()).set_api_url(api_url);

        let access = bot.get_managed_bot_access_settings(UserId(9901)).await.unwrap();
        assert!(access.is_access_restricted);
        assert_eq!(access.added_users.as_ref().unwrap()[0].id, UserId(12));

        let personal = bot.get_user_personal_chat_messages(UserId(42), 10).await.unwrap();
        assert!(personal.is_empty());

        let sent_guest = bot
            .answer_guest_query(
                GuestQueryId::from("gq-7"),
                InlineQueryResult::Article(InlineQueryResultArticle::new(
                    "1",
                    "Hello",
                    InputMessageContent::Text(InputMessageContentText::new("Hi guest")),
                )),
            )
            .await
            .unwrap();
        assert_eq!(sent_guest.inline_message_id, "guest-inline-7");

        let disk_video = b"\x00\x01DISK_LIVE_PHOTO_MP4\xff\xfe".to_vec();
        let mem_photo = b"\xff\xd8MEM_LIVE_PHOTO_JPG\xd9".to_vec();
        let unique =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let lp_path = std::env::temp_dir().join(format!("teloxidium-api-lp-{unique}.mp4"));
        std::fs::write(&lp_path, &disk_video).unwrap();
        let _lp_guard = TempFileGuard(lp_path.clone());

        let lp_req = bot.send_live_photo(
            ChatId(42),
            InputFile::file(&lp_path),
            InputFile::memory(mem_photo.clone()).file_name("lp.jpg"),
        );
        let sent_lp_ref = lp_req.send_ref().await.unwrap();
        assert_eq!(sent_lp_ref.live_photo().map(|lp| lp.file_id.0.as_str()), Some("lp-1"));
        let sent_lp_owned = lp_req.send().await.unwrap();
        assert_eq!(sent_lp_owned.live_photo().map(|lp| lp.file_id.0.as_str()), Some("lp-1"));

        let (l1, _, _) = rx.recv().unwrap();
        assert!(l1.starts_with("POST /bottest-token/GetManagedBotAccessSettings "), "{l1}");
        let (l2, _, _) = rx.recv().unwrap();
        assert!(l2.starts_with("POST /bottest-token/GetUserPersonalChatMessages "), "{l2}");
        let (l3, _, _) = rx.recv().unwrap();
        assert!(l3.starts_with("POST /bottest-token/AnswerGuestQuery "), "{l3}");
        for _ in 0..2 {
            let (line, headers, body) = rx.recv().unwrap();
            assert!(line.starts_with("POST /bottest-token/SendLivePhoto "), "{line}");
            assert!(headers.to_ascii_lowercase().contains("multipart/form-data"));
            let parts = parse_multipart_parts(&headers, &body);
            assert_eq!(resolve_multipart_file(&parts, "live_photo"), disk_video.as_slice());
            assert_eq!(resolve_multipart_file(&parts, "photo"), mem_photo.as_slice());
        }
    }
}
