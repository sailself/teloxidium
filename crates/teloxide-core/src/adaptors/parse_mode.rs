use std::future::IntoFuture;

use url::Url;

use crate::{
    payloads::{
        AnswerGuestQuery, AnswerInlineQuery, AnswerWebAppQuery, CopyMessage, EditMessageCaption,
        EditMessageCaptionInline, EditMessageChecklist, EditMessageMedia, EditMessageMediaInline,
        EditMessageText, EditMessageTextInline, EditStory, GiftPremiumSubscription, PostStory,
        SavePreparedInlineMessage, SendAnimation, SendAudio, SendChecklist, SendDocument, SendGift,
        SendGiftChat, SendLivePhoto, SendMediaGroup, SendMessage, SendMessageDraft, SendPaidMedia,
        SendPhoto, SendPoll, SendVideo, SendVoice,
    },
    prelude::Requester,
    requests::{HasPayload, Output, Request},
    types::*,
};

/// Default parse mode adaptor, see
/// [`RequesterExt::parse_mode`](crate::requests::RequesterExt::parse_mode).
#[derive(Clone, Debug)]
pub struct DefaultParseMode<B> {
    bot: B,
    mode: ParseMode,
}

/// Request returned by [`DefaultParseMode`] methods.
#[derive(Clone)]
pub struct DefaultParseModeRequest<R> {
    req: R,
    mode: ParseMode,
}

impl<B> DefaultParseMode<B> {
    /// Creates new [`DefaultParseMode`].
    ///
    /// Note: it's recommended to use [`RequesterExt::parse_mode`] instead.
    ///
    /// [`RequesterExt::parse_mode`]: crate::requests::RequesterExt::parse_mode
    pub fn new(bot: B, parse_mode: ParseMode) -> Self {
        Self { bot, mode: parse_mode }
    }

    /// Allows to access the inner bot.
    pub fn inner(&self) -> &B {
        &self.bot
    }

    /// Unwraps the inner bot.
    pub fn into_inner(self) -> B {
        self.bot
    }

    /// Returns currently used [`ParseMode`].
    pub fn parse_mode(&self) -> ParseMode {
        self.mode
    }
}

impl<R> Request for DefaultParseModeRequest<R>
where
    R: Request + Clone,
    R::Payload: VisitParseModes,
{
    type Err = R::Err;
    type Send = R::Send;
    type SendRef = R::Send;

    // Required methods
    fn send(mut self) -> Self::Send {
        self.req.payload_mut().visit_parse_modes(|mode| _ = mode.get_or_insert(self.mode));
        self.req.send()
    }

    fn send_ref(&self) -> Self::SendRef {
        // There is no other way to change the payload, given a `&self` :(
        self.clone().send()
    }
}

impl<R> IntoFuture for DefaultParseModeRequest<R>
where
    Self: Request,
{
    type Output = Result<Output<Self>, <Self as Request>::Err>;
    type IntoFuture = <Self as Request>::Send;

    fn into_future(self) -> Self::IntoFuture {
        self.send()
    }
}

impl<R> HasPayload for DefaultParseModeRequest<R>
where
    R: Request,
{
    type Payload = R::Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self.req.payload_mut()
    }

    fn payload_ref(&self) -> &Self::Payload {
        self.req.payload_ref()
    }
}

macro_rules! f {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        {
            let req = $this.inner().$m($($arg),*);
            DefaultParseModeRequest { req, mode: $this.mode }
        }
    };
}

macro_rules! fty {
    ($T:ident) => {
        DefaultParseModeRequest<B::$T>
    };
}

macro_rules! ftyid {
    ($T:ident) => {
        B::$T
    };
}

macro_rules! fid {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        $this.inner().$m($($arg),*)
    };
}

impl<B> Requester for DefaultParseMode<B>
where
    B: Requester,
    B::SendMessage: Clone,
    B::SendMessageDraft: Clone,
    B::SendPhoto: Clone,
    B::SendLivePhoto: Clone,
    B::SendVideo: Clone,
    B::SendAudio: Clone,
    B::SendDocument: Clone,
    B::SendAnimation: Clone,
    B::SendVoice: Clone,
    B::EditMessageText: Clone,
    B::EditMessageTextInline: Clone,
    B::EditMessageCaption: Clone,
    B::EditMessageCaptionInline: Clone,
    B::SendPoll: Clone,
    B::SendChecklist: Clone,
    B::CopyMessage: Clone,
    B::PostStory: Clone,
    B::EditStory: Clone,
    B::AnswerInlineQuery: Clone,
    B::AnswerWebAppQuery: Clone,
    B::AnswerGuestQuery: Clone,
    B::SavePreparedInlineMessage: Clone,
    B::EditMessageMedia: Clone,
    B::EditMessageMediaInline: Clone,
    B::EditMessageChecklist: Clone,
    B::SendPaidMedia: Clone,
    B::SendMediaGroup: Clone,
    B::GiftPremiumSubscription: Clone,
    B::SendGift: Clone,
    B::SendGiftChat: Clone,
{
    type Err = B::Err;

    requester_forward! {
        send_message,
        send_message_draft,
        send_photo,
        send_live_photo,
        send_video,
        send_audio,
        send_document,
        send_animation,
        send_voice,
        send_poll,
        send_checklist,
        edit_message_text,
        edit_message_text_inline,
        edit_message_caption,
        edit_message_caption_inline,
        edit_message_checklist,
        copy_message,
        post_story,
        edit_story,
        answer_inline_query,
        answer_web_app_query,
        answer_guest_query,
        save_prepared_inline_message,
        send_paid_media,
        send_media_group,
        edit_message_media,
        edit_message_media_inline,
        gift_premium_subscription,
        send_gift,
        send_gift_chat,
        => f, fty
    }

    requester_forward! {
        get_me,
        log_out,
        close,
        get_updates,
        set_webhook,
        delete_webhook,
        get_webhook_info,
        forward_message,
        forward_messages,
        copy_messages,
        send_video_note,
        send_location,
        edit_message_live_location,
        edit_message_live_location_inline,
        stop_message_live_location,
        stop_message_live_location_inline,
        send_venue,
        send_contact,
        send_dice,
        send_chat_action,
        set_message_reaction,
        delete_all_message_reactions,
        delete_message_reaction,
        get_user_profile_photos,
        get_user_profile_audios,
        set_user_emoji_status,
        get_file,
        kick_chat_member,
        ban_chat_member,
        unban_chat_member,
        restrict_chat_member,
        promote_chat_member,
        set_chat_administrator_custom_title,
        set_chat_member_tag,
        ban_chat_sender_chat,
        unban_chat_sender_chat,
        set_chat_permissions,
        export_chat_invite_link,
        create_chat_invite_link,
        edit_chat_invite_link,
        create_chat_subscription_invite_link,
        edit_chat_subscription_invite_link,
        revoke_chat_invite_link,
        set_chat_photo,
        delete_chat_photo,
        set_chat_title,
        set_chat_description,
        pin_chat_message,
        unpin_chat_message,
        unpin_all_chat_messages,
        leave_chat,
        get_chat,
        get_chat_administrators,
        get_chat_members_count,
        get_chat_member_count,
        get_chat_member,
        set_chat_sticker_set,
        delete_chat_sticker_set,
        get_forum_topic_icon_stickers,
        create_forum_topic,
        edit_forum_topic,
        close_forum_topic,
        reopen_forum_topic,
        delete_forum_topic,
        unpin_all_forum_topic_messages,
        edit_general_forum_topic,
        close_general_forum_topic,
        reopen_general_forum_topic,
        hide_general_forum_topic,
        unhide_general_forum_topic,
        unpin_all_general_forum_topic_messages,
        answer_callback_query,
        get_user_chat_boosts,
        set_my_commands,
        get_business_connection,
        get_my_commands,
        set_my_profile_photo,
        remove_my_profile_photo,
        set_my_name,
        get_my_name,
        set_my_description,
        get_my_description,
        set_my_short_description,
        get_my_short_description,
        set_chat_menu_button,
        get_chat_menu_button,
        set_my_default_administrator_rights,
        get_my_default_administrator_rights,
        delete_my_commands,
        save_prepared_keyboard_button,
        get_managed_bot_token,
        replace_managed_bot_token,
        get_managed_bot_access_settings,
        set_managed_bot_access_settings,
        get_user_personal_chat_messages,
        edit_message_reply_markup,
        edit_message_reply_markup_inline,
        stop_poll,
        approve_suggested_post,
        decline_suggested_post,
        edit_ephemeral_message_reply_markup,
        delete_ephemeral_message,
        delete_message,
        delete_messages,
        send_sticker,
        get_sticker_set,
        get_custom_emoji_stickers,
        upload_sticker_file,
        create_new_sticker_set,
        add_sticker_to_set,
        set_sticker_position_in_set,
        delete_sticker_from_set,
        replace_sticker_in_set,
        set_sticker_set_thumbnail,
        set_custom_emoji_sticker_set_thumbnail,
        set_sticker_set_title,
        delete_sticker_set,
        set_sticker_emoji_list,
        set_sticker_keywords,
        set_sticker_mask_position,
        get_available_gifts,
        get_user_gifts,
        get_chat_gifts,
        verify_user,
        verify_chat,
        remove_user_verification,
        remove_chat_verification,
        read_business_message,
        delete_business_messages,
        set_business_account_name,
        set_business_account_username,
        set_business_account_bio,
        set_business_account_profile_photo,
        remove_business_account_profile_photo,
        set_business_account_gift_settings,
        get_business_account_star_balance,
        transfer_business_account_stars,
        get_business_account_gifts,
        convert_gift_to_stars,
        upgrade_gift,
        transfer_gift,
        delete_story,
        repost_story,
        send_invoice,
        create_invoice_link,
        answer_shipping_query,
        answer_pre_checkout_query,
        get_my_star_balance,
        get_star_transactions,
        refund_star_payment,
        edit_user_star_subscription,
        set_passport_data_errors,
        send_game,
        set_game_score,
        set_game_score_inline,
        get_game_high_scores,
        approve_chat_join_request,
        decline_chat_join_request
        => fid, ftyid
    }
}

download_forward! {
    B
    DefaultParseMode<B>
    { this => this.inner() }
}

trait VisitParseModes {
    fn visit_parse_modes(&mut self, visitor: impl FnMut(&mut Option<ParseMode>));
}

macro_rules! impl_visit_parse_modes {
    (
        $(
            $T:ty => [
                $(
                    $field:ident
                ),*
            ]
            ,
        )*
    ) => {
        $(
            impl VisitParseModes for $T {
                fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
                    $(
                        visitor(&mut self.$field);
                    )*
                }
            }
        )*
    }
}

impl_visit_parse_modes! {
    SendMessage => [parse_mode],
    SendPhoto => [parse_mode],
    SendVideo => [parse_mode],
    SendAudio => [parse_mode],
    SendDocument => [parse_mode],
    SendAnimation => [parse_mode],
    SendVoice => [parse_mode],
    EditMessageText => [parse_mode],
    EditMessageTextInline => [parse_mode],
    EditMessageCaption => [parse_mode],
    EditMessageCaptionInline => [parse_mode],
    SendPaidMedia => [parse_mode],
    GiftPremiumSubscription => [text_parse_mode],
    SendGift => [text_parse_mode],
    SendGiftChat => [text_parse_mode],
    // FIXME: check if `parse_mode` changes anything if `.caption` is not set
    //        (and if it does, maybe not call visitor if `self.caption.is_none()`)
    CopyMessage => [parse_mode],
    PostStory => [parse_mode],
    EditStory => [parse_mode],
}

impl VisitParseModes for SendLivePhoto {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        if self.caption_entities.is_none() {
            visitor(&mut self.parse_mode);
        }
    }
}

impl VisitParseModes for SendPoll {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        if self.explanation_entities.is_none() {
            visitor(&mut self.explanation_parse_mode);
        }
        if self.description_entities.is_none() {
            visitor(&mut self.description_parse_mode);
        }
    }
}

impl VisitParseModes for AnswerGuestQuery {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_inline_query_result(&mut self.result, &mut visitor);
    }
}

impl VisitParseModes for SendMessageDraft {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        // Explicit `entities` take precedence over `DefaultParseMode` so Telegram
        // does not reject or re-parse entity-formatted draft text.
        if self.entities.is_none() {
            visitor(&mut self.parse_mode);
        }
    }
}

impl VisitParseModes for AnswerInlineQuery {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        self.results
            .iter_mut()
            .for_each(|result| visit_parse_modes_in_inline_query_result(result, &mut visitor))
    }
}

impl VisitParseModes for AnswerWebAppQuery {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_inline_query_result(&mut self.result, &mut visitor);
    }
}

impl VisitParseModes for SavePreparedInlineMessage {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_inline_query_result(&mut self.result, &mut visitor);
    }
}

impl VisitParseModes for SendMediaGroup {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        self.media
            .iter_mut()
            .for_each(|media| visit_parse_modes_in_input_media(media, &mut visitor))
    }
}

impl VisitParseModes for EditMessageMedia {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_input_media(&mut self.media, &mut visitor);
    }
}

impl VisitParseModes for EditMessageMediaInline {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_input_media(&mut self.media, &mut visitor);
    }
}

impl VisitParseModes for EditMessageChecklist {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visitor(&mut self.checklist.parse_mode);
    }
}

impl VisitParseModes for SendChecklist {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visitor(&mut self.checklist.parse_mode);
    }
}

fn visit_parse_modes_in_inline_query_result(
    result: &mut InlineQueryResult,
    visitor: &mut impl FnMut(&mut Option<ParseMode>),
) {
    use InlineQueryResult::*;

    let (caption_parse_mode, caption_entities, input_message_content) = match result {
        // Can contain `parse_mode` and/or `input_message_content`
        CachedAudio(r) => {
            (Some(&mut r.parse_mode), r.caption_entities.is_some(), &mut r.input_message_content)
        }
        CachedDocument(r) => {
            (Some(&mut r.parse_mode), r.caption_entities.is_some(), &mut r.input_message_content)
        }
        CachedGif(r) => {
            (Some(&mut r.parse_mode), r.caption_entities.is_some(), &mut r.input_message_content)
        }
        CachedMpeg4Gif(r) => {
            (Some(&mut r.parse_mode), r.caption_entities.is_some(), &mut r.input_message_content)
        }
        CachedPhoto(r) => {
            (Some(&mut r.parse_mode), r.caption_entities.is_some(), &mut r.input_message_content)
        }
        CachedVideo(r) => {
            (Some(&mut r.parse_mode), r.caption_entities.is_some(), &mut r.input_message_content)
        }
        CachedVoice(r) => {
            (Some(&mut r.parse_mode), r.caption_entities.is_some(), &mut r.input_message_content)
        }
        Audio(r) => {
            (Some(&mut r.parse_mode), r.caption_entities.is_some(), &mut r.input_message_content)
        }
        Document(r) => {
            (Some(&mut r.parse_mode), r.caption_entities.is_some(), &mut r.input_message_content)
        }
        Gif(r) => {
            (Some(&mut r.parse_mode), r.caption_entities.is_some(), &mut r.input_message_content)
        }
        Mpeg4Gif(r) => {
            (Some(&mut r.parse_mode), r.caption_entities.is_some(), &mut r.input_message_content)
        }
        Photo(r) => {
            (Some(&mut r.parse_mode), r.caption_entities.is_some(), &mut r.input_message_content)
        }
        Video(r) => {
            (Some(&mut r.parse_mode), r.caption_entities.is_some(), &mut r.input_message_content)
        }
        Voice(r) => {
            (Some(&mut r.parse_mode), r.caption_entities.is_some(), &mut r.input_message_content)
        }

        // Can contain parse mode if `InputMessageContent::Text`
        CachedSticker(r) => (None, true, &mut r.input_message_content),
        Contact(r) => (None, true, &mut r.input_message_content),
        Location(r) => (None, true, &mut r.input_message_content),
        Venue(r) => (None, true, &mut r.input_message_content),
        Article(r) => {
            if let InputMessageContent::Text(t) = &mut r.input_message_content {
                if t.entities.is_none() {
                    visitor(&mut t.parse_mode);
                }
            }
            return;
        }

        // Can't contain `parse_mode` at all
        Game(_r) => return,
    };

    if let Some(parse_mode) = caption_parse_mode {
        if !caption_entities {
            visitor(parse_mode);
        }
    }
    if let Some(InputMessageContent::Text(t)) = input_message_content {
        if t.entities.is_none() {
            visitor(&mut t.parse_mode);
        }
    }
}

fn visit_parse_modes_in_input_media(
    media: &mut InputMedia,
    visitor: &mut impl FnMut(&mut Option<ParseMode>),
) {
    use InputMedia::*;

    let (parse_mode, caption_entities) = match media {
        Photo(m) => (&mut m.parse_mode, &m.caption_entities),
        LivePhoto(m) => (&mut m.parse_mode, &m.caption_entities),
        Video(m) => (&mut m.parse_mode, &m.caption_entities),
        Animation(m) => (&mut m.parse_mode, &m.caption_entities),
        Audio(m) => (&mut m.parse_mode, &m.caption_entities),
        Document(m) => (&mut m.parse_mode, &m.caption_entities),
    };

    if caption_entities.is_none() {
        visitor(parse_mode);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        payloads::{SendLivePhotoSetters, SendMessageDraftSetters, SendPollSetters},
        requests::HasPayload,
        types::{ChatId, DraftId, InputPollOption, MessageEntity, ParseMode},
        Bot,
    };

    #[test]
    fn send_message_draft_uses_default_and_preserves_explicit_parse_mode() {
        let bot = DefaultParseMode::new(Bot::new("test-token"), ParseMode::MarkdownV2);

        let mut req = bot.send_message_draft(ChatId(42), DraftId(1), "*hello*");
        req.payload_mut().visit_parse_modes(|mode| _ = mode.get_or_insert(bot.parse_mode()));
        assert_eq!(req.payload_ref().parse_mode, Some(ParseMode::MarkdownV2));

        let mut explicit =
            bot.send_message_draft(ChatId(42), DraftId(2), "<b>hi</b>").parse_mode(ParseMode::Html);
        explicit.payload_mut().visit_parse_modes(|mode| _ = mode.get_or_insert(bot.parse_mode()));
        assert_eq!(explicit.payload_ref().parse_mode, Some(ParseMode::Html));

        let mut with_entities = bot
            .send_message_draft(ChatId(42), DraftId(3), "2 < 3")
            .entities([MessageEntity::bold(0, 5)]);
        with_entities
            .payload_mut()
            .visit_parse_modes(|mode| _ = mode.get_or_insert(bot.parse_mode()));
        assert_eq!(with_entities.payload_ref().parse_mode, None);
        let serialized = serde_json::to_value(with_entities.payload_ref()).unwrap();
        assert!(serialized.get("parse_mode").is_none());
        assert!(serialized.get("entities").is_some());
    }

    #[test]
    fn send_poll_uses_default_and_respects_explicit_parse_mode_and_entities() {
        let bot = DefaultParseMode::new(Bot::new("test-token"), ParseMode::MarkdownV2);

        let mut req = bot
            .send_poll(
                ChatId(42),
                "What *is* this?",
                [InputPollOption::new("A"), InputPollOption::new("B")],
            )
            .explanation("Default _explanation_")
            .description("Default _description_");
        req.payload_mut().visit_parse_modes(|mode| _ = mode.get_or_insert(bot.parse_mode()));
        assert_eq!(req.payload_ref().question_parse_mode, None);
        assert_eq!(req.payload_ref().explanation_parse_mode, Some(ParseMode::MarkdownV2));
        assert_eq!(req.payload_ref().description_parse_mode, Some(ParseMode::MarkdownV2));

        let mut explicit = bot
            .send_poll(ChatId(42), "Pick", [InputPollOption::new("A"), InputPollOption::new("B")])
            .explanation("<b>exp</b>")
            .explanation_parse_mode(ParseMode::Html)
            .description("<b>desc</b>")
            .description_parse_mode(ParseMode::Html);
        explicit.payload_mut().visit_parse_modes(|mode| _ = mode.get_or_insert(bot.parse_mode()));
        assert_eq!(explicit.payload_ref().explanation_parse_mode, Some(ParseMode::Html));
        assert_eq!(explicit.payload_ref().description_parse_mode, Some(ParseMode::Html));

        let mut with_entities = bot
            .send_poll(ChatId(42), "Pick", [InputPollOption::new("A"), InputPollOption::new("B")])
            .explanation("2 < 3")
            .explanation_entities([MessageEntity::bold(0, 5)])
            .description("4 > 1")
            .description_entities([MessageEntity::italic(0, 5)]);
        with_entities
            .payload_mut()
            .visit_parse_modes(|mode| _ = mode.get_or_insert(bot.parse_mode()));
        assert_eq!(with_entities.payload_ref().explanation_parse_mode, None);
        assert_eq!(with_entities.payload_ref().description_parse_mode, None);
    }

    fn spawn_capture_server(
        responses: Vec<String>,
    ) -> (Url, std::sync::mpsc::Receiver<(String, String, Vec<u8>)>) {
        use std::{
            io::{Read, Write},
            net::TcpListener,
            sync::mpsc,
        };

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            for response_body in responses {
                let (mut stream, _) = listener.accept().unwrap();
                let mut data = Vec::new();
                let mut buf = [0u8; 4096];
                let header_end = loop {
                    let n = stream.read(&mut buf).unwrap();
                    assert!(n > 0, "unexpected EOF reading HTTP headers");
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
                let body = if is_chunked {
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
                        assert!(n > 0, "unexpected EOF reading HTTP body");
                        data.extend_from_slice(&buf[..n]);
                    }
                    data[header_end..header_end + content_len].to_vec()
                };
                tx.send((req_line, headers, body)).unwrap();

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

        (Url::parse(&format!("http://{addr}/")).unwrap(), rx)
    }

    fn parse_multipart_parts(
        headers: &str,
        body: &[u8],
    ) -> std::collections::HashMap<String, Vec<u8>> {
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
        let mut parts = std::collections::HashMap::new();
        let mut pos = 0;
        while let Some(rel_start) =
            body[pos..].windows(delim.len()).position(|w| w == delim.as_slice())
        {
            let part_start = pos + rel_start + delim.len();
            pos = part_start;
            if body[part_start..].starts_with(b"--") {
                break;
            }
            let content_start =
                if body[part_start..].starts_with(b"\r\n") { part_start + 2 } else { part_start };
            let Some(hdr_end_rel) = body[content_start..].windows(4).position(|w| w == b"\r\n\r\n")
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
                    let name = name_rest[..name_end].to_owned();
                    parts.insert(name, body[data_start..data_end].to_vec());
                }
            }
            pos = data_start + next_delim_rel;
        }
        parts
    }

    fn parse_multipart_fields(
        headers: &str,
        body: &[u8],
    ) -> std::collections::HashMap<String, String> {
        parse_multipart_parts(headers, body)
            .into_iter()
            .map(|(k, v)| (k, String::from_utf8_lossy(&v).into_owned()))
            .collect()
    }

    struct TempFileGuard(std::path::PathBuf);

    impl Drop for TempFileGuard {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    #[tokio::test]
    async fn send_poll_default_parse_mode_send_and_send_ref_over_transport() {
        let poll_msg_json = serde_json::json!({
            "ok": true,
            "result": {
                "message_id": 501,
                "chat": { "id": 42, "first_name": "Alice", "type": "private" },
                "date": 1775174400,
                "poll": {
                    "id": "poll-resp-1",
                    "question": "Pick",
                    "options": [
                        { "persistent_id": "p1", "text": "A", "voter_count": 0 },
                        { "persistent_id": "p2", "text": "B", "voter_count": 0 }
                    ],
                    "total_voter_count": 0,
                    "is_closed": false,
                    "is_anonymous": true,
                    "type": "regular",
                    "allows_multiple_answers": false,
                    "allows_revoting": true
                }
            }
        })
        .to_string();

        let (api_url, rx) = spawn_capture_server(vec![poll_msg_json.clone(), poll_msg_json]);
        let bot = DefaultParseMode::new(
            Bot::with_client("test-token", reqwest::Client::new()).set_api_url(api_url),
            ParseMode::MarkdownV2,
        );

        let mixed_req = bot
            .send_poll(ChatId(42), "Pick", [InputPollOption::new("A"), InputPollOption::new("B")])
            .explanation("Default exp")
            .description("Raw description")
            .description_entities([MessageEntity::bold(0, 3)]);
        let msg1 = mixed_req.send_ref().await.unwrap();
        assert_eq!(msg1.id, MessageId(501));
        mixed_req.send().await.unwrap();

        let (line1, headers1, body1) = rx.recv().unwrap();
        let (line2, headers2, body2) = rx.recv().unwrap();
        for (line, headers, body) in [(line1, headers1, body1), (line2, headers2, body2)] {
            assert!(line.starts_with("POST /bottest-token/SendPoll "), "{line}");
            let fields = parse_multipart_fields(&headers, &body);
            assert_eq!(fields.get("question_parse_mode"), None);
            assert_eq!(
                fields.get("explanation_parse_mode").map(String::as_str),
                Some("MarkdownV2")
            );
            assert_eq!(fields.get("description_parse_mode"), None);
            assert!(fields.contains_key("description_entities"));
        }
    }

    #[tokio::test]
    async fn send_poll_multipart_attachments_and_send_ref_over_transport() {
        use crate::types::{
            InputMediaLivePhoto, InputMediaPhoto, InputMediaSticker, InputPollMedia,
            InputPollOptionMedia,
        };

        let poll_msg_json = serde_json::json!({
            "ok": true,
            "result": {
                "message_id": 502,
                "chat": { "id": 42, "first_name": "Alice", "type": "private" },
                "date": 1778200000,
                "poll": {
                    "id": "poll-resp-2",
                    "question": "Pick",
                    "options": [
                        { "persistent_id": "p1", "text": "A", "voter_count": 0 }
                    ],
                    "total_voter_count": 0,
                    "is_closed": false,
                    "is_anonymous": true,
                    "type": "regular",
                    "allows_multiple_answers": false,
                    "allows_revoting": true,
                    "members_only": false
                }
            }
        })
        .to_string();

        let lp_bytes = b"\x00\x01LIVE_PHOTO_DISK_MP4\xff\xfe".to_vec();
        let stk_bytes = b"\x89WEBP_STICKER_DISK\x00\x02".to_vec();
        let poll_photo_bytes = b"\xff\xd8POLL_PHOTO_MEM\xd9".to_vec();
        let exp_photo_bytes = b"\xff\xd8EXP_PHOTO_MEM\xd9".to_vec();

        let unique =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let lp_path = std::env::temp_dir().join(format!("teloxidium-test-lp-{unique}.mp4"));
        let stk_path = std::env::temp_dir().join(format!("teloxidium-test-stk-{unique}.webp"));
        std::fs::write(&lp_path, &lp_bytes).unwrap();
        std::fs::write(&stk_path, &stk_bytes).unwrap();
        let _lp_guard = TempFileGuard(lp_path.clone());
        let _stk_guard = TempFileGuard(stk_path.clone());

        let (api_url, rx) = spawn_capture_server(vec![poll_msg_json.clone(), poll_msg_json]);
        let bot = DefaultParseMode::new(
            Bot::with_client("test-token", reqwest::Client::new()).set_api_url(api_url),
            ParseMode::Html,
        );

        let req = bot
            .send_poll(
                ChatId(42),
                "Pick",
                [InputPollOption::new("A").media(InputPollOptionMedia::Sticker(
                    InputMediaSticker::new(InputFile::file(&stk_path)),
                ))],
            )
            .media(InputPollMedia::LivePhoto(InputMediaLivePhoto::new(
                InputFile::file(&lp_path),
                InputFile::memory(poll_photo_bytes.clone()).file_name("poll.jpg"),
            )))
            .explanation_media(InputPollMedia::Photo(InputMediaPhoto::new(
                InputFile::memory(exp_photo_bytes.clone()).file_name("exp.jpg"),
            )));

        let msg_ref = req.send_ref().await.unwrap();
        assert_eq!(msg_ref.id, MessageId(502));
        let msg_owned = req.send().await.unwrap();
        assert_eq!(msg_owned.id, MessageId(502));

        for _ in 0..2 {
            let (line, headers, body) = rx.recv().unwrap();
            assert!(line.starts_with("POST /bottest-token/SendPoll "), "{line}");
            let parts = parse_multipart_parts(&headers, &body);

            let media_json: serde_json::Value =
                serde_json::from_slice(parts.get("media").expect("missing media field")).unwrap();
            let lp_attach = media_json["media"]
                .as_str()
                .and_then(|s| s.strip_prefix("attach://"))
                .expect("media.media must use attach://");
            assert_eq!(parts.get(lp_attach), Some(&lp_bytes));

            let poll_photo_attach = media_json["photo"]
                .as_str()
                .and_then(|s| s.strip_prefix("attach://"))
                .expect("media.photo must use attach://");
            assert_eq!(parts.get(poll_photo_attach), Some(&poll_photo_bytes));

            let exp_json: serde_json::Value = serde_json::from_slice(
                parts.get("explanation_media").expect("missing explanation_media field"),
            )
            .unwrap();
            let exp_attach = exp_json["media"]
                .as_str()
                .and_then(|s| s.strip_prefix("attach://"))
                .expect("explanation_media.media must use attach://");
            assert_eq!(parts.get(exp_attach), Some(&exp_photo_bytes));

            let options_json: serde_json::Value =
                serde_json::from_slice(parts.get("options").expect("missing options field"))
                    .unwrap();
            let stk_attach = options_json[0]["media"]["media"]
                .as_str()
                .and_then(|s| s.strip_prefix("attach://"))
                .expect("options[0].media.media must use attach://");
            assert_eq!(parts.get(stk_attach), Some(&stk_bytes));
        }
    }

    #[test]
    fn bot_api_10_0_parse_modes() {
        use crate::types::{
            GuestQueryId, InlineQueryResult, InlineQueryResultArticle, InlineQueryResultPhoto,
            InputMedia, InputMediaLivePhoto, InputMediaPhoto, InputMessageContent,
            InputMessageContentText, InputPollMedia, InputPollOptionMedia,
        };
        use reqwest::Url;

        let bot = DefaultParseMode::new(Bot::new("test-token"), ParseMode::Html);

        let mut lp_req = bot.send_live_photo(
            ChatId(42),
            InputFile::file_id("lp".into()),
            InputFile::file_id("p".into()),
        );
        lp_req.payload_mut().visit_parse_modes(|m| _ = m.get_or_insert(bot.parse_mode()));
        assert_eq!(lp_req.payload_ref().parse_mode, Some(ParseMode::Html));

        let mut lp_with_entities = bot
            .send_live_photo(
                ChatId(42),
                InputFile::file_id("lp".into()),
                InputFile::file_id("p".into()),
            )
            .caption("Raw <text>")
            .caption_entities([MessageEntity::bold(0, 3)]);
        lp_with_entities.payload_mut().visit_parse_modes(|m| _ = m.get_or_insert(bot.parse_mode()));
        assert_eq!(lp_with_entities.payload_ref().parse_mode, None);

        let mut edit_lp = bot.edit_message_media(
            ChatId(42),
            crate::types::MessageId(1),
            InputMedia::LivePhoto(
                InputMediaLivePhoto::new(
                    InputFile::file_id("lp".into()),
                    InputFile::file_id("p".into()),
                )
                .caption("Raw <text>")
                .caption_entities([MessageEntity::bold(0, 3)]),
            ),
        );
        edit_lp.payload_mut().visit_parse_modes(|m| _ = m.get_or_insert(bot.parse_mode()));
        match &edit_lp.payload_ref().media {
            InputMedia::LivePhoto(lp) => assert_eq!(lp.parse_mode, None),
            other => panic!("unexpected media: {other:?}"),
        }

        let mut guest_req = bot.answer_guest_query(
            GuestQueryId::from("gq-1"),
            InlineQueryResult::Article(InlineQueryResultArticle::new(
                "1",
                "Title",
                InputMessageContent::Text(InputMessageContentText::new("<b>Hello</b>")),
            )),
        );
        guest_req.payload_mut().visit_parse_modes(|m| _ = m.get_or_insert(bot.parse_mode()));
        match &guest_req.payload_ref().result {
            InlineQueryResult::Article(a) => match &a.input_message_content {
                InputMessageContent::Text(t) => assert_eq!(t.parse_mode, Some(ParseMode::Html)),
                other => panic!("unexpected content: {other:?}"),
            },
            other => panic!("unexpected result: {other:?}"),
        }

        let mut guest_article_entities = bot.answer_guest_query(
            GuestQueryId::from("gq-2"),
            InlineQueryResult::Article(InlineQueryResultArticle::new(
                "2",
                "Title",
                InputMessageContent::Text(
                    InputMessageContentText::new("2 < 3").entities([MessageEntity::bold(0, 5)]),
                ),
            )),
        );
        guest_article_entities
            .payload_mut()
            .visit_parse_modes(|m| _ = m.get_or_insert(bot.parse_mode()));
        match &guest_article_entities.payload_ref().result {
            InlineQueryResult::Article(a) => match &a.input_message_content {
                InputMessageContent::Text(t) => assert_eq!(t.parse_mode, None),
                other => panic!("unexpected content: {other:?}"),
            },
            other => panic!("unexpected result: {other:?}"),
        }

        let photo_url = Url::parse("https://example.com/photo.jpg").unwrap();
        let mut guest_photo_replacement = bot.answer_guest_query(
            GuestQueryId::from("gq-3"),
            InlineQueryResult::Photo(
                InlineQueryResultPhoto::new("3", photo_url.clone(), photo_url)
                    .caption("Explicit")
                    .caption_entities([MessageEntity::bold(0, 8)])
                    .input_message_content(InputMessageContent::Text(
                        InputMessageContentText::new("<b>Replacement</b>"),
                    )),
            ),
        );
        guest_photo_replacement
            .payload_mut()
            .visit_parse_modes(|m| _ = m.get_or_insert(bot.parse_mode()));
        match &guest_photo_replacement.payload_ref().result {
            InlineQueryResult::Photo(p) => {
                assert_eq!(p.parse_mode, None);
                match &p.input_message_content {
                    Some(InputMessageContent::Text(t)) => {
                        assert_eq!(t.parse_mode, Some(ParseMode::Html))
                    }
                    other => panic!("unexpected replacement content: {other:?}"),
                }
            }
            other => panic!("unexpected result: {other:?}"),
        }

        let mut poll_req = bot
            .send_poll(
                ChatId(42),
                "Pick",
                [InputPollOption::new("A").media(InputPollOptionMedia::Photo(
                    InputMediaPhoto::new(InputFile::file_id("opt-p".into())).caption("<b>Opt</b>"),
                ))],
            )
            .media(InputPollMedia::Photo(
                InputMediaPhoto::new(InputFile::file_id("desc-p".into())).caption("<b>Desc</b>"),
            ))
            .explanation_media(InputPollMedia::Photo(
                InputMediaPhoto::new(InputFile::file_id("exp-p".into())).caption("<b>Exp</b>"),
            ));
        poll_req.payload_mut().visit_parse_modes(|m| _ = m.get_or_insert(bot.parse_mode()));
        match poll_req.payload_ref().media.as_ref().unwrap() {
            InputPollMedia::Photo(p) => assert_eq!(p.parse_mode, None),
            other => panic!("unexpected poll media: {other:?}"),
        }
        match poll_req.payload_ref().explanation_media.as_ref().unwrap() {
            InputPollMedia::Photo(p) => assert_eq!(p.parse_mode, None),
            other => panic!("unexpected exp media: {other:?}"),
        }
        match poll_req.payload_ref().options[0].media.as_ref().unwrap() {
            InputPollOptionMedia::Photo(p) => assert_eq!(p.parse_mode, None),
            other => panic!("unexpected opt media: {other:?}"),
        }
    }
}
