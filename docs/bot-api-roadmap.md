# Telegram Bot API Upgrade Roadmap (9.2 → 10.5)

Tracking plan for bringing **Teloxidium** (`crates/teloxide-core`, `crates/teloxide`, `crates/teloxide-macros`) from its initial **Bot API 9.2** starting baseline up to **Bot API 10.5**.

- **Current Baseline:** Bot API **9.4** (February 9, 2026) declared in [`crates/teloxide-core/schema.ron`](../crates/teloxide-core/schema.ron) and [`crates/teloxide-core/custom_v2.json`](../crates/teloxide-core/custom_v2.json) (upgraded from the M0 Bot API **9.2** baseline), plus selective Bot API 10.x support from commit `870ba033`:
  - Receive-only `RichMessage` (`Vec<serde_json::Value>`), `RichMessage::project()`, `MediaKind::RichMessage`, `TextQuote::rich_texts`, and optional `ExternalReplyInfo::kind`.
  - Ephemeral text menus (`SendMessage::ephemeral_message_parameters`, `EphemeralMessageId`, `EphemeralMessageParameters`, `Message::receiver_user`, `edit_ephemeral_message_reply_markup`, `delete_ephemeral_message`).
- **Target Baseline:** Bot API **10.5** (November 22, 2026) — 10 Bot API releases across 11 incremental milestones (**M0–M10**).

## Milestone Dependency Flow

```mermaid
flowchart LR
    M0["M0 (#2): Deser Safety\n(date_time entity + baseline)"] --> M1["M1 (#3): Bot API 9.3\n(Drafts, Gifts, Stories, Ratings)"]
    M1 --> M2["M2 (#4): Bot API 9.4\n(Button Styles, Profile Photo/Audio,\nVideoQuality, ChatOwner)"]
    M2 --> M3["M3 (#5): Bot API 9.5\n(Member Tags, DateTime, Open Drafts)"]
    M3 --> M4["M4 (#6): Bot API 9.6\n(Managed Bots, Polls v2,\nPrepared Buttons)"]
    M4 --> M5["M5 (#7): Bot API 10.0\n(Guest Mode)"]
    M5 --> M6["M6 (#8): Bot API 10.1\n(Rich Messages Send/Draft/Edit)"]
    M6 --> M7["M7 (#9): Bot API 10.2\n(Rich Builders, Communities,\nSubscriptions, Ephemeral Edits)"]
    M7 --> M8["M8 (#10): Bot API 10.3\n(Rich Buttons/Blocks,\nAll-Send Ephemeral Params)"]
    M8 --> M9["M9 (#11): Bot API 10.4\n(HTML Widgets, Live Photos,\nEphemeral Replies/Callbacks)"]
    M9 --> M10["M10 (#12): Bot API 10.5\n(BotFullInfo, setMySettings,\nAccess Settings, Inviter)"]
```

## Master Tracking Table

| Status | Milestone | Bot API Version & Date | GitHub Ticket | Breaking Changes | Summary |
| :---: | :--- | :--- | :--- | :---: | :--- |
| `[x]` | **M0** | Hotfix + Baseline (`9.2`) | [#2](https://github.com/sailself/teloxidium/issues/2) | `[**BC**]` | Add `MessageEntityKind::DateTime` deserialization safety and verify baseline CI |
| `[x]` | **M1** | **9.3** (2025-12-31) | [#3](https://github.com/sailself/teloxidium/issues/3) | `[**BC**]` | `sendMessageDraft`, `getUserGifts`, `getChatGifts`, `repostStory`, `GiftBackground`, `UniqueGiftColors`, `UserRating`, `has_topics_enabled`, `UniqueGiftInfo`, `getBusinessAccountGifts` |
| `[x]` | **M2** | **9.4** (2026-02-09) | [#4](https://github.com/sailself/teloxidium/issues/4) | `[**BC**]` | Button `style` + `icon_custom_emoji_id`, `setMyProfilePhoto`/`removeMyProfilePhoto`, `UserProfileAudios`, `VideoQuality`, `UniqueGift::is_burned`/`UniqueGiftModel::rarity`, `ChatOwnerLeft`/`Changed` |
| `[ ]` | **M3** | **9.5** (2026-03-01) | [#5](https://github.com/sailself/teloxidium/issues/5) | No | Chat member `tag` / `sender_tag` / `setChatMemberTag` / `EDIT_TAG`, full `9.5` schema refresh, unrestricted `sendMessageDraft` |
| `[ ]` | **M4** | **9.6** (2026-04-03) | [#6](https://github.com/sailself/teloxidium/issues/6) | `[**BC**]` | Managed Bots (`UpdateKind::ManagedBot`, tokens), `savePreparedKeyboardButton`, Polls v2 (`correct_option_ids`, revoting, persistent IDs) |
| `[ ]` | **M5** | **10.0** (2026-05-08) | [#7](https://github.com/sailself/teloxidium/issues/7) | `[**BC**]` | Guest Mode (`UpdateKind::GuestMessage`, `answerGuestQuery`, `SentGuestMessage`, `Message` guest caller fields) |
| `[ ]` | **M6** | **10.1** (2026-06-11) | [#8](https://github.com/sailself/teloxidium/issues/8) | No | Typed `RichText` & `RichBlock`, `InputRichMessage`, `sendRichMessage`, `sendRichMessageDraft`, `editMessageRichMessage` |
| `[ ]` | **M7** | **10.2** (2026-07-14) | [#9](https://github.com/sailself/teloxidium/issues/9) | `[**BC**]` | `InputRichMessageMedia` & `InputRichBlock*`, `InputMediaVoiceNote`, Communities, `UpdateKind::Subscription`, ephemeral edit methods |
| `[ ]` | **M8** | **10.3** (2026-08-24) | [#10](https://github.com/sailself/teloxidium/issues/10) | No | `RichMessageButton`, rich button/document/quote blocks, `ephemeral_message_parameters` across all `send*` methods + `Throttle` |
| `[ ]` | **M9** | **10.4** (2026-11-09) | [#11](https://github.com/sailself/teloxidium/issues/11) | `[**BC**]` | HTML Widgets (`sendWidget`, `sendSharedWidget`), Live Photos (`sendLivePhoto`), `ReplyParameters::ephemeral_message_id`, `TonAddress`/`BankCardNumber` entities |
| `[ ]` | **M10** | **10.5** (2026-11-22) | [#12](https://github.com/sailself/teloxidium/issues/12) | `[**BC**]` | `BotFullInfo` split from `User`, `setMySettings`, `BotAccessSettings`, `getUserPersonalChatMessages`, `WriteAccessAllowed::inviter` |

---

## Repeatable Per-Milestone Technical Checklist

Execute this checklist for every milestone **M1–M10**:

1. **Official Spec Verification**
   - Verify exact wire field names, requiredness, integer widths, defaults, and return types against the [official Bot API reference](https://core.telegram.org/bots/api) and [Bot API changelog](https://core.telegram.org/bots/api-changelog).
2. **Wire Types (`crates/teloxide-core/src/types/`)**
   - Derive `Clone, Debug, PartialEq, Serialize, Deserialize` (+ `Eq, Hash` where fields permit) and `#[cfg_attr(test, derive(schemars::JsonSchema))]`.
   - Use `#[serde_with::skip_serializing_none]`, `#[serde(default)]` on optional booleans, and `#[cfg_attr(test, schemars(with = "..."))]` for custom-serde fields (`DateTime<Utc>`, `MessageId`, `Mime`).
   - Wrap new IDs in strongly-typed newtypes (`DraftId`, `GuestQueryId`, `WidgetId`, etc.) and re-export all new types in [`crates/teloxide-core/src/types.rs`](../crates/teloxide-core/src/types.rs).
   - Preserve variant ordering in flattened or untagged enums (`MessageKind`, `MediaKind`, `ExternalReplyInfoKind`, `InputMessageContent`).
3. **Method Schema & Codegen**
   - Update [`crates/teloxide-core/schema.ron`](../crates/teloxide-core/schema.ron) and [`crates/teloxide-core/custom_v2.json`](../crates/teloxide-core/custom_v2.json) (plus [`crates/teloxide-core/src/codegen/patch.rs`](../crates/teloxide-core/src/codegen/patch.rs) if doc/type overrides are needed).
   - Regenerate payloads, setters, `Requester`, and schema checks:
     ```powershell
     cargo test --locked -p teloxide-core --features "full nightly" --lib codegen_
     ```
4. **Client & Adaptor Wiring (`crates/teloxide-core/src/`)**
   - [`bot/api.rs`](../crates/teloxide-core/src/bot/api.rs): choose `JsonRequest` vs `MultipartRequest` (any payload carrying `InputFile` directly or inside nested types must use `MultipartRequest` and expose `files()` / `files_mut()`).
   - [`requests/requester.rs`](../crates/teloxide-core/src/requests/requester.rs): `forward_all!`.
   - [`adaptors/cache_me.rs`](../crates/teloxide-core/src/adaptors/cache_me.rs) & [`adaptors/trace.rs`](../crates/teloxide-core/src/adaptors/trace.rs): `requester_forward!`.
   - [`adaptors/erased.rs`](../crates/teloxide-core/src/adaptors/erased.rs): `ErasedRequester` trait + impl + `requester_forward!`.
   - [`adaptors/parse_mode.rs`](../crates/teloxide-core/src/adaptors/parse_mode.rs): first `requester_forward!` + `VisitParseModes` for methods carrying `ParseMode`, `InputMedia`, `InlineQueryResult`, or `InputRichMessage`; second `requester_forward!` otherwise.
   - [`adaptors/throttle/requester_impl.rs`](../crates/teloxide-core/src/adaptors/throttle/requester_impl.rs): first `requester_forward!` (or `Option<ChatIdHash>` extractor) for rate-limited send methods; second `requester_forward!` otherwise.
5. **Dispatcher & Filters (`crates/teloxide/src/dispatching/`)**
   - For new `UpdateKind` variants: update [`update.rs`](../crates/teloxide-core/src/types/update.rs) (`UpdateKind`, `AllowedUpdate`, `Update::user`/`chat`/`from`, serde), [`handler_description.rs`](../crates/teloxide/src/dispatching/handler_description.rs) (`Kind::full_set()` + `allowed_updates_full_set`), and [`filter_ext.rs`](../crates/teloxide/src/dispatching/filter_ext.rs) (`UpdateFilterExt`).
   - For new `MessageKind` / `MediaKind` variants: add getters on [`message.rs`](../crates/teloxide-core/src/types/message.rs) and filters in [`filter_ext.rs`](../crates/teloxide/src/dispatching/filter_ext.rs) (`MessageFilterExt`).
6. **JSON Fixtures & Verification**
   - Add round-trip JSON tests for every new type/method and backwards-compatibility tests for omitted optional fields in legacy payloads.
   - Run the full verification suite:
     ```powershell
     cargo test --locked -p teloxide-core --features "full nightly" --lib codegen_
     cargo test --locked -p teloxide-core --features "full nightly" --lib schema_check
     cargo fmt --all -- --check
     cargo build --locked --workspace --features "full nightly"
     cargo test --locked --workspace --features "full nightly" --lib --tests
     cargo clippy --locked --workspace --all-targets --features "full nightly" -- -D warnings
     cargo check --locked -p teloxide-core --no-default-features --features "rustls"
     cargo check --locked -p teloxide --no-default-features --features "rustls macros"
     git diff --check
     ```
7. **Version Markers & Release Notes**
   - Update `api_version` in [`crates/teloxide-core/schema.ron`](../crates/teloxide-core/schema.ron), doc comments in [`crates/teloxide-core/src/lib.rs`](../crates/teloxide-core/src/lib.rs) and [`crates/teloxide/src/lib.rs`](../crates/teloxide/src/lib.rs), badges in [`README.md`](../README.md) and [`crates/teloxide-core/README.md`](../crates/teloxide-core/README.md), changelogs, and [`MIGRATION_GUIDE.md`](../MIGRATION_GUIDE.md) (`[**BC**]`).
   - Bump crate SemVer versions per [`RELEASE_PROCEDURE.md`](../RELEASE_PROCEDURE.md) (minor bump for `[**BC**]`, patch bump for additive releases) and tag `v<teloxide-version>` on `master` after each Bot API milestone merges (no `-m*` tag suffixes).

---

## Key Architectural Decisions to Confirm

1. **Schema Sync Cadence (`custom_v2.json`):**
   - **Recommended:** Update `custom_v2.json` incrementally per milestone (via `teloxide/tba-parser` against archived/pinned docs or scoped edits) so `schema_check` passes at every step.
   - **Alternative:** Regenerate `custom_v2.json` directly for `10.5` upfront and work through the diff sequentially.
2. **Incoming `RichMessage::blocks` Representation (M6–M8, M10):**
   - **Recommended:** Keep `RichMessage { blocks: Vec<serde_json::Value>, is_rtl: Option<bool> }` and `RichMessage::project()` intact for incoming deserialization (preserving unknown future block types), while adding strongly-typed `InputRichMessage` / `InputRichBlock` / `RichText` / `RichMessageButton` types for outgoing requests.
   - **Alternative:** Replace `RichMessage::blocks` with `Vec<RichBlock>` featuring an `Unknown(Value)` fallback (`[**BC**]` for existing fork callers).
3. **`Me` vs `BotFullInfo` Naming (M10):**
   - **Recommended:** Keep `Me` as the return type of `get_me`, extend it with all Bot API 10.5 `BotFullInfo` fields, export `pub type BotFullInfo = Me;`, and map `BotFullInfo` → `Me` in `rust_types_check_codegen.rs`.
   - **Alternative:** Rename `Me` → `BotFullInfo` (`[**BC**]`).
4. **Optional `chat_id` on Ephemeral Callback Replies (M9):**
   - **Recommended:** Keep `bot.send_message(chat_id, text)` and existing `send_*` signatures intact (`Throttle` already returns `Option<ChatIdHash>::None` for callback-only ephemeral calls, and Telegram ignores `chat_id` when `callback_query_id` is supplied without `receiver_user_id`); only add optional `chat_id` setters/siblings if downstream callers lack any `ChatId` to pass.
5. **Fork Metadata & Pinned Toolchain (M0 — Confirmed):**
   - Pointed `homepage`/`repository` in [`Cargo.toml`](../Cargo.toml) to `https://github.com/sailself/teloxidium`, and kept [`rust-toolchain.toml`](../rust-toolchain.toml) pinned to `nightly-2025-06-12` (coordinated with CI and `rustfmt.toml`).

---

## Milestone Breakdown

### M0 — Deserialization Safety (`date_time` Entity) & Baseline ([#2](https://github.com/sailself/teloxidium/issues/2))
- **Goal:** Prevent live Bot API 9.5+ `date_time` message entities from breaking `Update` deserialization while validating the baseline workspace.
- **Key changes (`[**BC**]`):** `MessageEntityKind::DateTime { unix_time: DateTime<Utc>, date_time_format: Option<String> }` + `MessageEntity::date_time(...)` + `Renderer` HTML/MarkdownV2 support + UTF-16 `MessageEntityRef::parse` / `Message::parse_caption_entities` / `serde_multipart` tests; `MessageEntity` synced in `custom_v2.json`; `Cargo.toml` fork metadata updated.

### M1 — Bot API 9.3 ([#3](https://github.com/sailself/teloxidium/issues/3))
- **Goal:** Support streaming drafts, user/chat gifts, cross-account business story reposts, gift backgrounds, unique gift color schemes, user ratings, and Bot API 9.3 schema updates.
- **Key changes (`[**BC**]`):** `DraftId`, `GiftBackground`, `UniqueGiftColors`, `UserRating`; updates to `Me` (`has_topics_enabled`), `Gift`, `UniqueGift` (`gift_id`, `is_premium`, `is_from_blockchain`, `colors`), `GiftInfo` & `OwnedGiftRegular`, `UniqueGiftInfo` (`last_resale_currency`/`last_resale_amount`, `UniqueGiftOrigin::{GiftedUpgrade, Offer}`), `AcceptedGiftTypes::gifts_from_channels`, `ForumTopic*`, `ChecklistTask::completed_by_chat`, `ChatFullInfo`, `MessageKind::GiftUpgradeSent` + `filter_gift_upgrade_sent`; methods `sendMessageDraft` (`VisitParseModes`, throttled), `getUserGifts`, `getChatGifts`, `repostStory`, updated `getBusinessAccountGifts`, `forwardMessage`/`copyMessage::message_effect_id`, and 25,000 Stars paid media docs.

### M2 — Bot API 9.4 ([#4](https://github.com/sailself/teloxidium/issues/4))
- **Goal:** Support styled/custom-emoji buttons, bot profile photos, user profile audios, video quality variants, crafted unique gift metadata (`is_burned`, `rarity`), and chat ownership service messages.
- **Key changes (`[**BC**]`):** `ButtonStyle` + `icon_custom_emoji_id` on `KeyboardButton`/`InlineKeyboardButton`; `UserProfileAudios`, `VideoQuality` (`Video::qualities`), `ChatFullInfoPrivate::first_profile_audio`, `Me::allows_users_to_create_topics`, `UniqueGift::is_burned`, `UniqueGiftModel::rarity` (`UniqueGiftModelRarity`); `MessageKind::{ChatOwnerLeft, ChatOwnerChanged}` + `filter_chat_owner_left`/`filter_chat_owner_changed`; methods `setMyProfilePhoto` (`MultipartRequest`), `removeMyProfilePhoto`, `getUserProfileAudios`, and updated `SetBusinessAccountProfilePhoto` (`MultipartRequest`).

### M3 — Bot API 9.5 ([#5](https://github.com/sailself/teloxidium/issues/5))
- **Goal:** Perform the full Bot API 9.5 schema/version refresh (building on the `MessageEntity` `date_time` entry synced in M0) and add chat member tags & tag permissions.
- **Key changes:** `ChatMember` (`Member::tag`, `Restricted::tag`, `Restricted::can_edit_tag`, `Administrator::can_manage_tags`), `ChatAdministratorRights::can_manage_tags`, `ChatPermissions::EDIT_TAG` (`1 << 14`), `Message::sender_tag`; methods `setChatMemberTag`, `promoteChatMember::can_manage_tags`; unrestricted `sendMessageDraft` docs.

### M4 — Bot API 9.6 ([#6](https://github.com/sailself/teloxidium/issues/6))
- **Goal:** Add Managed Bots, Prepared Keyboard Buttons, and the Polls v2 overhaul.
- **Key changes (`[**BC**]`):** `Poll::correct_option_ids` & `SendPoll::correct_option_ids` (replacing `correct_option_id`), revoting/shuffle/open options/descriptions, `PollOption::persistent_id` & adder metadata, `PollAnswer::option_persistent_ids`, `ReplyParameters::poll_option_id`, `PollOptionAdded`/`PollOptionDeleted`; `KeyboardButtonRequestManagedBot`, `ManagedBotCreated`, `ManagedBotUpdated` (`UpdateKind::ManagedBot`), `PreparedKeyboardButton`; methods `getManagedBotToken`, `replaceManagedBotToken`, `savePreparedKeyboardButton`, updated `sendPoll` (`VisitParseModes` including `description_parse_mode`).

### M5 — Bot API 10.0 ([#7](https://github.com/sailself/teloxidium/issues/7))
- **Goal:** Support Guest Mode for receiving and answering guest-mention queries in non-member chats.
- **Key changes (`[**BC**]`):** `GuestQueryId`, `SentGuestMessage`, `Me::supports_guest_queries`, `Message::{guest_bot_caller_user, guest_bot_caller_chat, guest_query_id}` (boxed `User`/`Chat`), `UpdateKind::GuestMessage` / `AllowedUpdate::GuestMessage` + `filter_guest_message`, method `answerGuestQuery` (`VisitParseModes`).

### M6 — Bot API 10.1 ([#8](https://github.com/sailself/teloxidium/issues/8))
- **Goal:** Add outgoing, streaming, and editable Rich Messages alongside the fork's existing projection API.
- **Key changes:** Split `rich_message.rs` into `rich_message/{mod.rs, text.rs, blocks.rs, input.rs}`; typed `RichText` & `RichBlock`, `InputRichMessage`, `InputRichMessageContent` (`InputMessageContent::RichMessage`); methods `sendRichMessage`, `sendRichMessageDraft`, `editMessageRichMessage` / `editMessageRichMessageInline`.

### M7 — Bot API 10.2 ([#9](https://github.com/sailself/teloxidium/issues/9))
- **Goal:** Add rich block input builders/media, voice-note album media, Communities, payment subscription updates, and full ephemeral edit methods.
- **Key changes (`[**BC**]`):** `InputRichMessageMedia` (`MultipartRequest`), `InputRichBlock*`, `InputMediaVoiceNote`, `Community` / `CommunityChatAdded` / `CommunityChatRemoved`, `BotSubscriptionUpdated` (`UpdateKind::Subscription` + filter), and `editEphemeralMessageText` / `editEphemeralMessageCaption` / `editEphemeralMessageMedia` / `editEphemeralMessageRichMessage`.

### M8 — Bot API 10.3 ([#10](https://github.com/sailself/teloxidium/issues/10))
- **Goal:** Add interactive Rich Message buttons/blocks and generalize `ephemeral_message_parameters` across all supported send methods.
- **Key changes:** `RichMessageButton`, `RichText::Button`, `RichBlockButtons`/`Document`/`ExpandableBlockQuotation`, `RichBlockTable::is_compact`; `ephemeral_message_parameters` added across all Bot API 10.3 `send*` methods; shared `Throttle` `Option<ChatIdHash>` helper across all ephemeral-capable throttled sends.

### M9 — Bot API 10.4 ([#11](https://github.com/sailself/teloxidium/issues/11))
- **Goal:** Add HTML Widgets, Live Photos, `ton_address`/`bank_card_number` entities, and ephemeral message reply/callback enhancements.
- **Key changes (`[**BC**]`):** `WidgetId`, `Widget`, `LivePhoto`, `MediaKind::{Widget, LivePhoto}`, `ExternalReplyInfoKind::{Widget, LivePhoto}`, `MessageEntityKind::{TonAddress, BankCardNumber}` (plus `custom_v2.json` & update fixtures), `InputMediaLivePhoto`, `PaidMediaLivePhoto`, `InputPaidMediaLivePhoto`, `ReplyParameters::{ephemeral_message_id, message_id: Option<MessageId>}`, filters `filter_widget`/`filter_live_photo`, methods `sendLivePhoto` (`MultipartRequest`), `sendWidget`, `sendSharedWidget`.

### M10 — Bot API 10.5 ([#12](https://github.com/sailself/teloxidium/issues/12))
- **Goal:** Complete Bot API 10.5 parity with `BotFullInfo`, `setMySettings`, managed bot access settings, personal chat messages, bulk invite tracking, and updated expandable block quotations.
- **Key changes (`[**BC**]`):** `BotAccessSettings`, `CallbackQueryMode`, `Me`/`BotFullInfo` new fields + `rust_types_check_codegen.rs` cleanup, `WriteAccessAllowed::inviter`, `CallbackQuery` doc updates, `RichBlockExpandableBlockQuotation` (`text` → `blocks`), methods `setMySettings`, `getManagedBotAccessSettings`, `setManagedBotAccessSettings`, `getUserPersonalChatMessages`, and final `10.5` badges/docs.
