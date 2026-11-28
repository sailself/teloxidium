#![allow(clippy::redundant_closure_call)]
// Required for the `filter_from` currently
#![allow(deprecated)]

use dptree::Handler;

use crate::{
    dispatching::DpHandlerDescription,
    types::{AllowedUpdate, Message, Update, UpdateKind},
};

macro_rules! define_ext {
    ($ext_name:ident, $for_ty:ty => $( ($func:ident, $proj_fn:expr, $fn_doc:expr $(, $Allowed:ident)? ) ,)*) => {
        #[doc = concat!("Filter methods for [`", stringify!($for_ty), "`].")]
        pub trait $ext_name<Out>: private::Sealed {
            $( define_ext!(@sig $func, $fn_doc); )*
        }

        impl<Out> $ext_name<Out> for $for_ty
        where
            Out: Send + Sync + 'static,
        {
            $( define_ext!(@impl $for_ty, $func, $proj_fn $(, $Allowed )? ); )*
        }
    };

    (@sig $func:ident, $fn_doc:expr) => {
        #[doc = $fn_doc]
        fn $func() -> Handler<'static, Out, DpHandlerDescription>;
    };

    (@impl $for_ty:ty, $func:ident, $proj_fn:expr, $Allowed:ident) => {
        fn $func() -> Handler<'static, Out, DpHandlerDescription> {
            dptree::filter_map_with_description(DpHandlerDescription::of(AllowedUpdate::$Allowed), move |input: $for_ty| {
                $proj_fn(input)
            })
        }
    };

    (@impl $for_ty:ty, $func:ident, $proj_fn:expr) => {
        fn $func() -> Handler<'static, Out, DpHandlerDescription> {
            dptree::filter_map(move |input: $for_ty| {
                $proj_fn(input)
            })
        }
    };
}

mod private {
    use teloxide_core::types::{Message, Update};

    pub trait Sealed {}

    impl Sealed for Update {}
    impl Sealed for Message {}
}

// FIXME: rewrite this macro to allow the usage of functions returning small
// values without borrowing
macro_rules! define_message_ext {
    ($( ($func:ident, $fn_name:path) ,)*) => {
        define_ext! {
            MessageFilterExt, Message =>
            $((
                $func,
                (|x| $fn_name(&x).map(ToOwned::to_owned)),
                concat!("Applies the [`", stringify!($fn_name), "`] filter.")
            ),)*
        }
    }
}

// FIXME: change macro so that we can filter things without getters
// May be expanded in the future.
define_message_ext! {
    // MessageCommon
    (filter_from, Message::from),
    // MediaKind variants of the MessageKind::Common
    (filter_animation, Message::animation),
    (filter_audio, Message::audio),
    (filter_contact, Message::contact),
    (filter_document, Message::document),
    (filter_paid_media, Message::paid_media),
    (filter_game, Message::game),
    (filter_venue, Message::venue),
    (filter_location, Message::location),
    (filter_live_photo, Message::live_photo),
    (filter_photo, Message::photo),
    (filter_poll, Message::poll),
    (filter_checklist, Message::checklist),
    (filter_sticker, Message::sticker),
    (filter_story, Message::story),
    (filter_text, Message::text),
    (filter_rich_message, Message::rich_message),
    (filter_video, Message::video),
    (filter_video_note, Message::video_note),
    (filter_voice, Message::voice),
    (filter_migration, Message::chat_migration),
    (filter_migration_from, Message::migrate_from_chat_id),
    (filter_migration_to, Message::migrate_to_chat_id),
    (filter_reply_to_message, Message::reply_to_message),
    (filter_forward_origin, Message::forward_origin),
    (filter_reply_to_story, Message::reply_to_story),
    // Rest variants of a MessageKind
    (filter_new_chat_members, Message::new_chat_members),
    (filter_left_chat_member, Message::left_chat_member),
    (filter_new_chat_title, Message::new_chat_title),
    (filter_new_chat_photo, Message::new_chat_photo),
    (filter_delete_chat_photo, Message::delete_chat_photo),
    (filter_group_chat_created, Message::group_chat_created),
    (filter_supergroup_chat_created, Message::super_group_chat_created),
    (filter_channel_chat_created, Message::channel_chat_created),
    (filter_message_auto_delete_timer_changed, Message::message_auto_delete_timer_changed),
    (filter_pinned, Message::pinned_message),
    (filter_invoice, Message::invoice),
    (filter_successful_payment, Message::successful_payment),
    (filter_connected_website, Message::connected_website),
    (filter_write_access_allowed, Message::write_access_allowed),
    (filter_passport_data, Message::passport_data),
    (filter_dice, Message::dice),
    (filter_proximity_alert_triggered, Message::proximity_alert_triggered),
    (filter_boost_added, Message::boost_added),
    (filter_chat_background_set, Message::chat_background_set),
    (filter_checklist_tasks_done, Message::checklist_tasks_done),
    (filter_checklist_tasks_added, Message::checklist_tasks_added),
    (filter_direct_message_price_changed, Message::direct_message_price_changed),
    (filter_forum_topic_created, Message::forum_topic_created),
    (filter_forum_topic_edited, Message::forum_topic_edited),
    (filter_forum_topic_closed, Message::forum_topic_closed),
    (filter_forum_topic_reopened, Message::forum_topic_reopened),
    (filter_general_forum_topic_hidden, Message::general_forum_topic_hidden),
    (filter_general_forum_topic_unhidden, Message::general_forum_topic_unhidden),
    (filter_giveaway, Message::giveaway),
    (filter_giveaway_completed, Message::giveaway_completed),
    (filter_giveaway_created, Message::giveaway_created),
    (filter_giveaway_winners, Message::giveaway_winners),
    (filter_paid_message_price_changed, Message::paid_message_price_changed),
    (filter_suggested_post_approved, Message::suggested_post_approved),
    (filter_suggested_post_approval_failed, Message::suggested_post_approval_failed),
    (filter_suggested_post_declined, Message::suggested_post_declined),
    (filter_suggested_post_paid, Message::suggested_post_paid),
    (filter_suggested_post_refunded, Message::suggested_post_refunded),
    (filter_gift_info, Message::gift_info),
    (filter_unique_gift_info, Message::unique_gift_info),
    (filter_gift_upgrade_sent, Message::gift_upgrade_sent),
    (filter_chat_owner_left, Message::chat_owner_left),
    (filter_chat_owner_changed, Message::chat_owner_changed),
    (filter_managed_bot_created, Message::managed_bot_created),
    (filter_poll_option_added, Message::poll_option_added),
    (filter_poll_option_deleted, Message::poll_option_deleted),
    (filter_video_chat_scheduled, Message::video_chat_scheduled),
    (filter_video_chat_started, Message::video_chat_started),
    (filter_video_chat_ended, Message::video_chat_ended),
    (filter_video_chat_participants_invited, Message::video_chat_participants_invited),
    (filter_web_app_data, Message::web_app_data),
}

macro_rules! define_update_ext {
    ($( ($func:ident, $kind:path, $Allowed:ident) ,)*) => {
        define_ext! {
            UpdateFilterExt, Update =>
            $((
                $func,
                |update: Update| match update.kind {
                    $kind(x) => Some(x),
                    _ => None,
                },
                concat!("Filters out [`", stringify!($kind), "`] objects."),
                $Allowed
            ),)*
        }
    }
}

define_update_ext! {
    (filter_message, UpdateKind::Message, Message),
    (filter_edited_message, UpdateKind::EditedMessage, EditedMessage),
    (filter_channel_post, UpdateKind::ChannelPost, ChannelPost),
    (filter_edited_channel_post, UpdateKind::EditedChannelPost, EditedChannelPost),
    (filter_business_connection, UpdateKind::BusinessConnection, BusinessConnection),
    (filter_business_message, UpdateKind::BusinessMessage, BusinessMessage),
    (filter_edited_business_message, UpdateKind::EditedBusinessMessage, EditedBusinessMessage),
    (filter_deleted_business_messages, UpdateKind::DeletedBusinessMessages, DeletedBusinessMessages),
    (filter_message_reaction_updated, UpdateKind::MessageReaction, MessageReaction),
    (filter_message_reaction_count_updated, UpdateKind::MessageReactionCount, MessageReactionCount),
    (filter_inline_query, UpdateKind::InlineQuery, InlineQuery),
    (filter_chosen_inline_result, UpdateKind::ChosenInlineResult, ChosenInlineResult),
    (filter_callback_query, UpdateKind::CallbackQuery, CallbackQuery),
    (filter_shipping_query, UpdateKind::ShippingQuery, ShippingQuery),
    (filter_pre_checkout_query, UpdateKind::PreCheckoutQuery, PreCheckoutQuery),
    (filter_purchased_paid_media, UpdateKind::PurchasedPaidMedia, PurchasedPaidMedia),
    (filter_poll, UpdateKind::Poll, Poll),
    (filter_poll_answer, UpdateKind::PollAnswer, PollAnswer),
    (filter_my_chat_member, UpdateKind::MyChatMember, MyChatMember),
    (filter_chat_member, UpdateKind::ChatMember, ChatMember),
    (filter_chat_join_request, UpdateKind::ChatJoinRequest, ChatJoinRequest),
    (filter_chat_boost, UpdateKind::ChatBoost, ChatBoost),
    (filter_removed_chat_boost, UpdateKind::RemovedChatBoost, RemovedChatBoost),
    (filter_managed_bot, UpdateKind::ManagedBot, ManagedBot),
    (filter_guest_message, UpdateKind::GuestMessage, GuestMessage),
}

#[cfg(test)]
mod tests {
    use std::ops::ControlFlow;

    use dptree::deps;

    use super::*;

    #[tokio::test]
    async fn filter_chat_owner_left_and_changed_dispatch() {
        let left_with_new_owner: Update = serde_json::from_str(
            r#"{
                "update_id": 1,
                "message": {
                    "message_id": 10,
                    "chat": { "id": -1001, "title": "Group", "type": "supergroup" },
                    "date": 1770600000,
                    "chat_owner_left": {
                        "new_owner": { "id": 42, "is_bot": false, "first_name": "Alice" }
                    }
                }
            }"#,
        )
        .unwrap();

        let left_without_new_owner: Update = serde_json::from_str(
            r#"{
                "update_id": 2,
                "message": {
                    "message_id": 11,
                    "chat": { "id": -1001, "title": "Group", "type": "supergroup" },
                    "date": 1770600001,
                    "chat_owner_left": {}
                }
            }"#,
        )
        .unwrap();

        let owner_changed: Update = serde_json::from_str(
            r#"{
                "update_id": 3,
                "message": {
                    "message_id": 12,
                    "chat": { "id": -1001, "title": "Group", "type": "supergroup" },
                    "date": 1770600002,
                    "chat_owner_changed": {
                        "new_owner": { "id": 99, "is_bot": false, "first_name": "Bob" }
                    }
                }
            }"#,
        )
        .unwrap();

        let ordinary_text: Update = serde_json::from_str(
            r#"{
                "update_id": 4,
                "message": {
                    "message_id": 13,
                    "chat": { "id": -1001, "title": "Group", "type": "supergroup" },
                    "date": 1770600003,
                    "text": "hello"
                }
            }"#,
        )
        .unwrap();

        let left_handler = dptree::entry().branch(Update::filter_message().chain(
            Message::filter_chat_owner_left().endpoint(
                |left: crate::types::ChatOwnerLeft| async move { left.new_owner.map(|u| u.id.0) },
            ),
        ));
        let changed_handler = dptree::entry().branch(Update::filter_message().chain(
            Message::filter_chat_owner_changed().endpoint(
                |changed: crate::types::ChatOwnerChanged| async move { changed.new_owner.id.0 },
            ),
        ));

        assert!(matches!(
            left_handler.dispatch(deps![left_with_new_owner.clone()]).await,
            ControlFlow::Break(Some(42))
        ));
        assert!(matches!(
            changed_handler.dispatch(deps![left_with_new_owner]).await,
            ControlFlow::Continue(_)
        ));

        assert!(matches!(
            left_handler.dispatch(deps![left_without_new_owner.clone()]).await,
            ControlFlow::Break(None)
        ));
        assert!(matches!(
            changed_handler.dispatch(deps![left_without_new_owner]).await,
            ControlFlow::Continue(_)
        ));

        assert!(matches!(
            changed_handler.dispatch(deps![owner_changed.clone()]).await,
            ControlFlow::Break(99)
        ));
        assert!(matches!(
            left_handler.dispatch(deps![owner_changed]).await,
            ControlFlow::Continue(_)
        ));

        assert!(matches!(
            left_handler.dispatch(deps![ordinary_text.clone()]).await,
            ControlFlow::Continue(_)
        ));
        assert!(matches!(
            changed_handler.dispatch(deps![ordinary_text]).await,
            ControlFlow::Continue(_)
        ));
    }

    #[tokio::test]
    async fn filter_managed_bot_and_poll_option_service_messages_dispatch() {
        let managed_bot_update: Update = serde_json::from_str(
            r#"{
                "update_id": 10,
                "managed_bot": {
                    "user": { "id": 100, "is_bot": false, "first_name": "Alice" },
                    "bot": { "id": 900, "is_bot": true, "first_name": "Helper", "username": "helper_bot" }
                }
            }"#,
        )
        .unwrap();

        let managed_bot_created_msg: Update = serde_json::from_str(
            r#"{
                "update_id": 11,
                "message": {
                    "message_id": 20,
                    "chat": { "id": 100, "first_name": "Alice", "type": "private" },
                    "date": 1775174400,
                    "managed_bot_created": {
                        "bot": { "id": 901, "is_bot": true, "first_name": "SubBot", "username": "sub_bot" }
                    }
                }
            }"#,
        )
        .unwrap();

        let poll_option_added_msg: Update = serde_json::from_str(
            r#"{
                "update_id": 12,
                "message": {
                    "message_id": 21,
                    "chat": { "id": -1001, "title": "Group", "type": "supergroup" },
                    "date": 1775174401,
                    "poll_option_added": {
                        "option_persistent_id": "opt-1",
                        "option_text": "Choice 1"
                    }
                }
            }"#,
        )
        .unwrap();

        let poll_option_deleted_msg: Update = serde_json::from_str(
            r#"{
                "update_id": 13,
                "message": {
                    "message_id": 22,
                    "chat": { "id": -1001, "title": "Group", "type": "supergroup" },
                    "date": 1775174402,
                    "poll_option_deleted": {
                        "option_persistent_id": "opt-1",
                        "option_text": "Choice 1"
                    }
                }
            }"#,
        )
        .unwrap();

        let managed_bot_handler = dptree::entry().branch(
            Update::filter_managed_bot()
                .endpoint(|u: crate::types::ManagedBotUpdated| async move { u.bot.id.0 }),
        );
        let created_handler = dptree::entry().branch(
            Update::filter_message().chain(
                Message::filter_managed_bot_created()
                    .endpoint(|c: crate::types::ManagedBotCreated| async move { c.bot.id.0 }),
            ),
        );
        let added_handler = dptree::entry().branch(
            Update::filter_message().chain(Message::filter_poll_option_added().endpoint(
                |a: crate::types::PollOptionAdded| async move { a.option_persistent_id },
            )),
        );
        let deleted_handler = dptree::entry().branch(Update::filter_message().chain(
            Message::filter_poll_option_deleted().endpoint(
                |d: crate::types::PollOptionDeleted| async move { d.option_persistent_id },
            ),
        ));

        assert!(matches!(
            managed_bot_handler.dispatch(deps![managed_bot_update.clone()]).await,
            ControlFlow::Break(900)
        ));
        assert!(matches!(
            created_handler.dispatch(deps![managed_bot_update]).await,
            ControlFlow::Continue(_)
        ));

        assert!(matches!(
            created_handler.dispatch(deps![managed_bot_created_msg.clone()]).await,
            ControlFlow::Break(901)
        ));
        assert!(matches!(
            managed_bot_handler.dispatch(deps![managed_bot_created_msg]).await,
            ControlFlow::Continue(_)
        ));

        assert!(matches!(
            added_handler.dispatch(deps![poll_option_added_msg.clone()]).await,
            ControlFlow::Break(ref id) if id == "opt-1"
        ));
        assert!(matches!(
            deleted_handler.dispatch(deps![poll_option_added_msg]).await,
            ControlFlow::Continue(_)
        ));

        assert!(matches!(
            deleted_handler.dispatch(deps![poll_option_deleted_msg.clone()]).await,
            ControlFlow::Break(ref id) if id == "opt-1"
        ));
        assert!(matches!(
            added_handler.dispatch(deps![poll_option_deleted_msg]).await,
            ControlFlow::Continue(_)
        ));
    }

    #[tokio::test]
    async fn filter_guest_message_and_live_photo_dispatch() {
        let guest_update: Update = serde_json::from_str(
            r#"{
                "update_id": 20,
                "guest_message": {
                    "message_id": 30,
                    "chat": { "id": -1001, "title": "Lounge", "type": "supergroup" },
                    "date": 1778200000,
                    "guest_query_id": "gq-20",
                    "text": "Hi"
                }
            }"#,
        )
        .unwrap();

        let live_photo_update: Update = serde_json::from_str(
            r#"{
                "update_id": 21,
                "message": {
                    "message_id": 31,
                    "chat": { "id": 42, "first_name": "Alice", "type": "private" },
                    "date": 1778200001,
                    "live_photo": {
                        "photo": [{ "file_id": "p-id", "file_unique_id": "p-uniq", "width": 640, "height": 480 }],
                        "file_id": "lp-id",
                        "file_unique_id": "lp-uniq",
                        "width": 640,
                        "height": 480,
                        "duration": 2
                    },
                    "photo": [{ "file_id": "p-id", "file_unique_id": "p-uniq", "width": 640, "height": 480 }]
                }
            }"#,
        )
        .unwrap();

        let rich_update: Update = serde_json::from_str(
            r#"{
                "update_id": 22,
                "message": {
                    "message_id": 32,
                    "chat": { "id": 42, "first_name": "Alice", "type": "private" },
                    "date": 1778200002,
                    "rich_message": { "blocks": [{ "type": "paragraph", "text": "Hello" }] }
                }
            }"#,
        )
        .unwrap();

        let guest_handler = dptree::entry()
            .branch(Update::filter_guest_message().endpoint(|m: Message| async move { m.id.0 }));
        let live_photo_handler = dptree::entry().branch(
            Update::filter_message().chain(
                Message::filter_live_photo()
                    .endpoint(|lp: crate::types::LivePhoto| async move { lp.file_id.0 }),
            ),
        );
        let photo_handler = dptree::entry().branch(Update::filter_message().chain(
            Message::filter_photo().endpoint(|sizes: Vec<crate::types::PhotoSize>| async move {
                sizes[0].file.id.0.clone()
            }),
        ));
        let rich_handler = dptree::entry().branch(
            Update::filter_message().chain(
                Message::filter_rich_message()
                    .endpoint(|rm: crate::types::RichMessage| async move { rm.project().text }),
            ),
        );

        assert!(matches!(
            guest_handler.dispatch(deps![guest_update.clone()]).await,
            ControlFlow::Break(30)
        ));
        assert!(matches!(
            live_photo_handler.dispatch(deps![guest_update]).await,
            ControlFlow::Continue(_)
        ));
        assert!(matches!(
            live_photo_handler.dispatch(deps![live_photo_update.clone()]).await,
            ControlFlow::Break(ref id) if id == "lp-id"
        ));
        assert!(matches!(
            photo_handler.dispatch(deps![live_photo_update.clone()]).await,
            ControlFlow::Break(ref id) if id == "p-id"
        ));
        assert!(matches!(
            guest_handler.dispatch(deps![live_photo_update]).await,
            ControlFlow::Continue(_)
        ));
        assert!(matches!(
            rich_handler.dispatch(deps![rich_update]).await,
            ControlFlow::Break(ref t) if t == "Hello"
        ));
    }
}
