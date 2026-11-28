use std::ops::Range;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

use crate::types::{FileId, MessageEntity, MessageEntityKind, User};

/// An incoming [rich formatted message](https://core.telegram.org/bots/api#richmessage).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RichMessage {
    /// Bot API block objects in message order. The original JSON is retained,
    /// including fields and block types unknown to this version of the library.
    pub blocks: Vec<Value>,
    /// Whether the message should be shown right-to-left.
    pub is_rtl: Option<bool>,
}

impl RichMessage {
    /// Extracts readable text, link and user entities, mentions, and file
    /// references from the rich content. The original blocks remain available
    /// in [`Self::blocks`] for consumers that need the full formatting.
    #[must_use]
    pub fn project(&self) -> RichMessageContent {
        let mut projection = Projection::default();
        projection.blocks(&self.blocks);
        // Preserve a sender-supplied newline that may be part of an entity.
        if projection.trailing_separator {
            projection.content.text.pop();
        }
        projection.content
    }

    /// Extracts readable text, link and user entities, and mentions from an
    /// inline sequence of rich text fragments (such as
    /// [`TextQuote::rich_texts`]).
    ///
    /// [`TextQuote::rich_texts`]: crate::types::TextQuote::rich_texts
    #[must_use]
    pub fn project_inline(fragments: &[Value]) -> RichMessageContent {
        let mut projection = Projection::default();
        for fragment in fragments {
            projection.inline(fragment);
        }
        projection.content
    }
}

/// Text and file references extracted from a [`RichMessage`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RichMessageContent {
    /// Text in reading order, with line breaks between blocks.
    pub text: String,
    /// Links and user mentions with UTF-16 offsets into [`Self::text`].
    pub entities: Vec<MessageEntity>,
    /// Username mentions whose display text can differ from the actual target.
    pub mentions: Vec<RichMention>,
    /// File references in block order, including files inside nested blocks.
    pub media: Vec<RichMedia>,
}

impl RichMessageContent {
    /// Returns the users mentioned by ID in the projected rich content.
    pub fn mentioned_users(&self) -> impl Iterator<Item = &User> {
        crate::util::mentioned_users_from_entities(&self.entities)
    }
}

/// A rich-text mention that links to a username.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RichMention {
    /// Target username without a leading `@`.
    pub username: String,
    /// UTF-8 byte range of the displayed mention in the projected text.
    pub range: Range<usize>,
}

/// A file reference extracted from a rich block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RichMedia {
    pub file_id: FileId,
    /// Nonempty MIME type supplied by Telegram, when available.
    pub mime_type: Option<String>,
    pub file_name: Option<String>,
    pub kind: RichMediaKind,
}

/// The kind of rich block that supplied a file reference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RichMediaKind {
    Photo,
    Animation,
    Video,
    Audio,
    VoiceNote,
    Document,
}

#[derive(Default)]
struct Projection {
    content: RichMessageContent,
    utf16_len: usize,
    trailing_separator: bool,
}

fn items(value: &Value) -> &[Value] {
    value.as_array().map_or(&[], Vec::as_slice)
}

impl Projection {
    fn blocks(&mut self, blocks: &[Value]) {
        for block in blocks {
            match block["type"].as_str().unwrap_or_default() {
                "list" => {
                    for item in items(&block["items"]) {
                        self.line(&item["label"]);
                        self.blocks(items(&item["blocks"]));
                    }
                }
                "table" => {
                    self.line(&block["caption"]);
                    for row in items(&block["cells"]) {
                        for (index, cell) in items(row).iter().enumerate() {
                            if index > 0 {
                                self.push_text(" | ");
                            }
                            self.inline(&cell["text"]);
                        }
                        self.line_break();
                    }
                }
                "buttons" => {
                    for button in items(&block["buttons"]) {
                        self.link(&button["text"], &button["url"]);
                        self.line_break();
                    }
                }
                kind => {
                    self.line(&block["text"]);
                    self.line(&block["expression"]);
                    self.line(&block["summary"]);
                    self.collect_media(block, kind);
                    self.blocks(items(&block["blocks"]));
                    self.line(&block["caption"]["text"]);
                    self.line(&block["caption"]["credit"]);
                    self.line(&block["credit"]);
                }
            }
        }
    }

    fn collect_media(&mut self, block: &Value, block_kind: &str) {
        let (file, kind) = match block_kind {
            "photo" => {
                let Some(photo) = items(&block["photo"])
                    .iter()
                    .filter(|photo| file_id(photo).is_some())
                    .max_by_key(|photo| {
                        let width = photo["width"].as_u64().unwrap_or_default();
                        let height = photo["height"].as_u64().unwrap_or_default();
                        width.saturating_mul(height)
                    })
                else {
                    return;
                };
                (photo, RichMediaKind::Photo)
            }
            "animation" => (&block["animation"], RichMediaKind::Animation),
            "video" => (&block["video"], RichMediaKind::Video),
            "audio" => (&block["audio"], RichMediaKind::Audio),
            "voice_note" => (&block["voice_note"], RichMediaKind::VoiceNote),
            "document" => (&block["document"], RichMediaKind::Document),
            _ => return,
        };
        if let Some(id) = file_id(file) {
            self.content.media.push(RichMedia {
                file_id: FileId(id.to_owned()),
                mime_type: file["mime_type"]
                    .as_str()
                    .filter(|mime| !mime.trim().is_empty())
                    .map(str::to_owned),
                file_name: file["file_name"].as_str().map(str::to_owned),
                kind,
            });
        }
    }

    fn line(&mut self, value: &Value) {
        self.inline(value);
        self.line_break();
    }

    fn line_break(&mut self) {
        if !self.content.text.is_empty() && !self.content.text.ends_with('\n') {
            self.content.text.push('\n');
            self.utf16_len += 1;
            self.trailing_separator = true;
        }
    }

    fn push_text(&mut self, text: &str) {
        if !text.is_empty() {
            self.content.text.push_str(text);
            self.utf16_len += text.encode_utf16().count();
            self.trailing_separator = false;
        }
    }

    fn inline(&mut self, value: &Value) {
        match value {
            Value::String(text) => self.push_text(text),
            Value::Array(children) => {
                for child in children {
                    self.inline(child);
                }
            }
            Value::Object(_) => self.inline_object(value),
            _ => {}
        }
    }

    fn inline_object(&mut self, value: &Value) {
        match value["type"].as_str().unwrap_or_default() {
            "url" => self.link(&value["text"], &value["url"]),
            "button" => self.link(&value["button"]["text"], &value["button"]["url"]),
            "mention" => {
                let start = self.content.text.len();
                self.inline(&value["text"]);
                if let Some(username) = value["username"].as_str() {
                    let username = username.trim_start_matches('@');
                    if !username.is_empty() && self.content.text.len() > start {
                        self.content.mentions.push(RichMention {
                            username: username.to_owned(),
                            range: start..self.content.text.len(),
                        });
                    }
                }
            }
            "text_mention" => {
                let start = self.utf16_len;
                self.inline(&value["text"]);
                if let Ok(user) = User::deserialize(&value["user"]) {
                    self.entity(start, MessageEntityKind::TextMention { user });
                }
            }
            "custom_emoji" => self.inline(&value["alternative_text"]),
            "mathematical_expression" => self.inline(&value["expression"]),
            _ => self.inline(&value["text"]),
        }
    }

    fn link(&mut self, text: &Value, url: &Value) {
        let start = self.utf16_len;
        self.inline(text);
        if let Some(url) = url.as_str().and_then(|url| Url::parse(url).ok()) {
            self.entity(start, MessageEntityKind::TextLink { url });
        }
    }

    fn entity(&mut self, start: usize, kind: MessageEntityKind) {
        if self.utf16_len > start {
            self.content.entities.push(MessageEntity::new(kind, start, self.utf16_len - start));
        }
    }
}

fn file_id(file: &Value) -> Option<&str> {
    file["file_id"].as_str().filter(|id| !id.is_empty())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::types::{ExternalReplyInfoKind, MessageEntityRef, MessageId, Update, UpdateKind};

    #[test]
    fn rich_reply_is_decoded_by_the_standard_update_type() {
        let rich = json!({"is_rtl": true, "blocks": [
            {"type": "slideshow", "blocks": [
                {"type": "photo", "photo": [
                    {"file_id": "large", "width": 640, "height": 480},
                    {"file_id": "small", "width": 160, "height": 120}
                ]},
                {"type": "video", "video": {"file_id": "video", "mime_type": "video/webm"}}
            ], "caption": {"text": "Album"}},
            {"type": "future_block", "future_field": {"kept": true}},
            {"type": "paragraph", "text": {
                "type": "text_mention", "text": "Reader",
                "user": {"id": 8, "is_bot": false, "first_name": "Reader"}
            }}
        ]});
        let raw = json!({"update_id": 1, "message": {
            "message_id": 2, "date": 1700000000, "chat": {"id": -1, "type": "group", "title": "Test"},
            "text": "/q", "reply_to_message": {
                "message_id": 1, "date": 1700000000,
                "chat": {"id": -1, "type": "group", "title": "Test"},
                "from": {"id": 7, "is_bot": false, "first_name": "Sender"},
                "rich_message": rich
            }
        }});
        let update: Update = serde_json::from_str(&raw.to_string()).unwrap();
        let UpdateKind::Message(message) = update.kind else { panic!("expected a message") };
        assert_eq!(message.text(), Some("/q"));
        let reply = message.reply_to_message().unwrap();
        assert_eq!(reply.id, MessageId(1));
        assert_eq!(reply.from.as_ref().unwrap().id.0, 7);
        let content = reply.rich_message().unwrap();
        assert_eq!(serde_json::to_value(content).unwrap(), rich);
        let projected = content.project();
        assert_eq!(projected.text, "Album\nReader");
        let [photo, video] = projected.media.as_slice() else { panic!("expected two files") };
        assert_eq!(photo.file_id.0, "large");
        assert_eq!(photo.mime_type, None);
        assert_eq!(video.file_id.0, "video");
        assert_eq!(video.mime_type.as_deref(), Some("video/webm"));
        assert_eq!(projected.mentioned_users().map(|user| user.id.0).collect::<Vec<_>>(), [8]);
    }

    #[test]
    fn projection_preserves_mentions_and_utf16_entity_ranges() {
        let rich: RichMessage = serde_json::from_value(json!({"blocks": [
            {"type": "paragraph", "text": {"type": "mention", "text": "🧭", "username": "@map"}},
            {"type": "table", "cells": [[
                {"text": "a"},
                {"text": {"type": "url", "text": "界🚀\n", "url": "https://example.org/"}}
            ]]}
        ]}))
        .unwrap();
        let content = rich.project();
        assert_eq!(content.text, "🧭\na | 界🚀\n");
        assert_eq!(content.mentions, [RichMention { username: "map".into(), range: 0..4 }]);
        assert_eq!(content.entities[0].offset, 7);
        assert_eq!(content.entities[0].length, 4);
        assert_eq!(MessageEntityRef::parse(&content.text, &content.entities)[0].text(), "界🚀\n");
    }

    #[test]
    fn external_reply_without_media_keeps_the_message_and_rich_quote() {
        let rich_texts = json!(["Quoted ", {"type": "bold", "text": "rich text"}]);
        let mut raw = json!({"update_id": 1, "message": {
            "message_id": 2, "date": 1700000000, "chat": {"id": 42, "type": "private"},
            "text": "/q", "quote": {"text": "", "position": 0, "rich_texts": rich_texts},
            "external_reply": {
                "origin": {
                    "type": "user", "date": 1700000000,
                    "sender_user": {"id": 7, "is_bot": false, "first_name": "Sender"}
                }
            }
        }});
        let update: Update = serde_json::from_str(&raw.to_string()).unwrap();
        let UpdateKind::Message(message) = update.kind else { panic!("expected a message") };
        assert_eq!(message.text(), Some("/q"));
        let reply = message.external_reply().unwrap();
        assert_eq!(reply.message_id, None);
        assert_eq!(reply.kind, None);
        assert!(serde_json::to_value(reply).unwrap().get("kind").is_none());
        let quote = message.quote().unwrap();
        assert_eq!(serde_json::to_value(quote).unwrap()["rich_texts"], rich_texts);
        let projected_quote = quote.project_rich_texts().unwrap();
        assert_eq!(projected_quote.text, "Quoted rich text");

        let channel = json!({"id": -1000000000002_i64, "type": "channel", "title": "Channel"});
        raw["message"]["external_reply"] = json!({
            "origin": {"type": "channel", "date": 1700000000, "message_id": 1, "chat": channel},
            "message_id": 1, "chat": channel
        });
        let body = raw["message"].as_object_mut().unwrap();
        body.remove("text");
        body.insert(
            "rich_message".to_owned(),
            json!({"blocks": [{"type": "paragraph", "text": "Rich reply"}]}),
        );
        let update: Update = serde_json::from_str(&raw.to_string()).unwrap();
        let UpdateKind::Message(message) = update.kind else { panic!("expected a message") };
        assert_eq!(message.rich_message().unwrap().project().text, "Rich reply");
        assert_eq!(message.external_reply().unwrap().message_id, Some(MessageId(1)));

        raw["message"]["external_reply"]["photo"] = json!([
            {"file_id": "photo", "file_unique_id": "unique", "width": 320, "height": 240}
        ]);
        let update: Update = serde_json::from_str(&raw.to_string()).unwrap();
        let UpdateKind::Message(message) = update.kind else { panic!("expected a message") };
        let reply = message.external_reply().unwrap();
        assert!(matches!(
            &reply.kind, Some(ExternalReplyInfoKind::Photo(sizes)) if sizes[0].file.id.0 == "photo"
        ));
        assert_eq!(serde_json::to_value(reply).unwrap()["photo"][0]["file_id"], "photo");
    }
}
