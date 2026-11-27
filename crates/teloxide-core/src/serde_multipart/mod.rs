//! Module for serializing into `multipart/form-data`
//! ([`reqwest::multipart::Form`])
//!
//! [`reqwest::multipart::Form`]: reqwest::multipart::Form
//!
//! ## How it works
//!
//! You better not know...
//!
//! This whole module is an awful hack and we'll probably stop using it in next
//! versions (in favor of something less automatic, but more simple).

mod error;
mod serializers;

use std::future::Future;

use reqwest::multipart::Form;
use serde::Serialize;

use crate::requests::MultipartPayload;
use error::Error;
use serializers::MultipartSerializer;

/// Serializes given value into [`Form`] **taking all input files out**.
///
/// [`Form`]:  reqwest::multipart::Form
pub(crate) fn to_form<T>(val: &mut T) -> Result<impl Future<Output = Form>, Error>
where
    T: Serialize + MultipartPayload,
{
    let mut form = val.serialize(MultipartSerializer::new())?;

    let mut vec = Vec::with_capacity(1);
    val.move_files(&mut |f| vec.push(f));
    let iter = vec.into_iter();

    let fut = async move {
        for file in iter {
            if file.needs_attach() {
                let id = file.id().to_owned();
                if let Some(part) = file.into_part() {
                    form = form.part(id, part.await);
                }
            }
        }

        form
    };

    Ok(fut)
}

/// Serializes given value into [`Form`].
///
/// [`Form`]:  reqwest::multipart::Form
pub(crate) fn to_form_ref<T: ?Sized>(val: &T) -> Result<impl Future<Output = Form>, Error>
where
    T: Serialize + MultipartPayload,
{
    let mut form = val.serialize(MultipartSerializer::new())?;
    let mut vec = Vec::with_capacity(1);
    val.copy_files(&mut |f| vec.push(f));

    let iter = vec.into_iter();

    let fut = async move {
        for file in iter {
            if file.needs_attach() {
                let id = file.id().to_owned();
                if let Some(part) = file.into_part() {
                    form = form.part(id, part.await);
                }
            }
        }

        form
    };

    Ok(fut)
}

#[cfg(test)]
mod tests {
    use tokio::fs::File;

    use super::{to_form, to_form_ref};
    use crate::{
        payloads::{self, setters::*},
        types::{
            BusinessConnectionId, ChatId, InputFile, InputMedia, InputMediaAnimation,
            InputMediaAudio, InputMediaDocument, InputMediaPhoto, InputMediaVideo,
            InputProfilePhoto, InputProfilePhotoAnimated, InputProfilePhotoStatic, InputSticker,
            MessageEntity, MessageEntityKind, ParseMode, StickerFormat, UserId,
        },
    };

    // https://github.com/teloxide/teloxide/issues/473
    #[tokio::test]
    async fn issue_473() {
        to_form_ref(
            &payloads::SendPhoto::new(ChatId(0), InputFile::file_id("0".into())).caption_entities(
                [MessageEntity { kind: MessageEntityKind::Url, offset: 0, length: 0 }],
            ),
        )
        .unwrap()
        .await;
    }

    #[tokio::test]
    async fn test_send_photo_date_time_caption_entities() {
        use futures::TryStreamExt;

        let ts1 = chrono::DateTime::from_timestamp(1_700_000_000, 0).unwrap();
        let ts2 = chrono::DateTime::from_timestamp(1_700_003_600, 0).unwrap();

        let form = to_form_ref(
            &payloads::SendPhoto::new(ChatId(0), InputFile::file_id("0".into())).caption_entities(
                [
                    MessageEntity::date_time(ts1, Some("wDT".to_owned()), 0, 5),
                    MessageEntity::date_time(ts2, None, 6, 4),
                ],
            ),
        )
        .unwrap()
        .await;

        let chunks: Vec<bytes::Bytes> = form.into_stream().try_collect().await.unwrap();
        let body = String::from_utf8(chunks.concat()).unwrap();
        let expected_json = r#"[{"type":"date_time","unix_time":1700000000,"date_time_format":"wDT","offset":0,"length":5},{"type":"date_time","unix_time":1700003600,"offset":6,"length":4}]"#;
        assert!(
            body.contains(&format!("name=\"caption_entities\"\r\n\r\n{expected_json}\r\n")),
            "unexpected multipart body:\n{body}"
        );
    }

    #[tokio::test]
    async fn test_send_media_group() {
        const CAPTION: &str = "caption";

        to_form_ref(&payloads::SendMediaGroup::new(
            ChatId(0),
            [
                InputMedia::Photo(
                    InputMediaPhoto::new(InputFile::file("../../media/teloxide-core-logo.png"))
                        .caption(CAPTION)
                        .parse_mode(ParseMode::MarkdownV2)
                        .caption_entities(entities()),
                ),
                InputMedia::Video(
                    InputMediaVideo::new(InputFile::file_id("17".into())).supports_streaming(true),
                ),
                InputMedia::Animation(
                    InputMediaAnimation::new(InputFile::read(
                        File::open("../../media/example.gif").await.unwrap(),
                    ))
                    .thumbnail(InputFile::read(
                        File::open("../../media/teloxide-core-logo.png").await.unwrap(),
                    ))
                    .duration(17),
                ),
                InputMedia::Audio(
                    InputMediaAudio::new(InputFile::url("https://example.com".parse().unwrap()))
                        .performer("a"),
                ),
                InputMedia::Document(InputMediaDocument::new(InputFile::memory(
                    &b"Hello world!"[..],
                ))),
            ],
        ))
        .unwrap()
        .await;
    }

    #[tokio::test]
    async fn test_add_sticker_to_set() {
        to_form_ref(&payloads::AddStickerToSet::new(
            UserId(0),
            "name",
            InputSticker {
                sticker: InputFile::file(
                    "../../media/
                teloxide-core-logo.png",
                ),
                emoji_list: vec!["✈️⚙️".to_owned()],
                keywords: vec![],
                mask_position: None,
                format: StickerFormat::Static,
            },
        ))
        .unwrap()
        .await;
    }

    #[tokio::test]
    async fn test_send_animation() {
        to_form_ref(
            &payloads::SendAnimation::new(
                ChatId(0),
                InputFile::file("../../media/teloxide-core-logo.png"),
            )
            .caption_entities(entities())
            .thumbnail(InputFile::read(
                File::open("../../media/teloxide-core-logo.png").await.unwrap(),
            )),
        )
        .unwrap()
        .await;
    }

    #[tokio::test]
    async fn test_set_my_profile_photo_multipart() {
        use futures::TryStreamExt;

        let static_file = InputFile::memory(b"fake-jpg-bytes".to_vec());
        let attach_id = static_file.id().to_owned();
        let mut payload =
            payloads::SetMyProfilePhoto::new(InputProfilePhoto::Static(InputProfilePhotoStatic {
                photo: static_file,
            }));

        let form_ref = to_form_ref(&payload).unwrap().await;
        let body_ref = String::from_utf8(
            form_ref.into_stream().try_collect::<Vec<bytes::Bytes>>().await.unwrap().concat(),
        )
        .unwrap();
        let expected_static_json = format!(r#"{{"type":"static","photo":"attach://{attach_id}"}}"#);
        assert!(
            body_ref.contains(&format!("name=\"photo\"\r\n\r\n{expected_static_json}\r\n")),
            "unexpected multipart body:\n{body_ref}"
        );
        assert!(
            body_ref.contains(&format!("name=\"{attach_id}\""))
                && body_ref.contains("fake-jpg-bytes"),
            "missing attached file part:\n{body_ref}"
        );

        let form_owned = to_form(&mut payload).unwrap().await;
        let body_owned = String::from_utf8(
            form_owned.into_stream().try_collect::<Vec<bytes::Bytes>>().await.unwrap().concat(),
        )
        .unwrap();
        assert!(body_owned.contains(&expected_static_json));
        assert!(body_owned.contains("fake-jpg-bytes"));

        let anim_file = InputFile::memory(b"fake-mp4-bytes".to_vec());
        let anim_id = anim_file.id().to_owned();
        let anim_payload = payloads::SetMyProfilePhoto::new(InputProfilePhoto::Animated(
            InputProfilePhotoAnimated { animation: anim_file, main_frame_timestamp: Some(1.5) },
        ));
        let anim_form = to_form_ref(&anim_payload).unwrap().await;
        let anim_body = String::from_utf8(
            anim_form.into_stream().try_collect::<Vec<bytes::Bytes>>().await.unwrap().concat(),
        )
        .unwrap();
        let expected_anim_json = format!(
            r#"{{"type":"animated","animation":"attach://{anim_id}","main_frame_timestamp":1.5}}"#
        );
        assert!(
            anim_body.contains(&format!("name=\"photo\"\r\n\r\n{expected_anim_json}\r\n")),
            "unexpected animated multipart body:\n{anim_body}"
        );
        assert!(
            anim_body.contains(&format!("name=\"{anim_id}\""))
                && anim_body.contains("fake-mp4-bytes"),
            "missing animated file part:\n{anim_body}"
        );
    }

    #[tokio::test]
    async fn test_set_business_account_profile_photo_multipart() {
        use futures::TryStreamExt;

        let static_file = InputFile::memory(b"biz-jpg-bytes".to_vec());
        let attach_id = static_file.id().to_owned();
        let mut payload = payloads::SetBusinessAccountProfilePhoto::new(
            BusinessConnectionId("biz-conn-1".to_owned()),
            InputProfilePhoto::Static(InputProfilePhotoStatic { photo: static_file }),
        )
        .is_public(true);

        let form = to_form(&mut payload).unwrap().await;
        let body = String::from_utf8(
            form.into_stream().try_collect::<Vec<bytes::Bytes>>().await.unwrap().concat(),
        )
        .unwrap();
        let expected_json = format!(r#"{{"type":"static","photo":"attach://{attach_id}"}}"#);
        assert!(body.contains("name=\"business_connection_id\"\r\n\r\nbiz-conn-1\r\n"));
        assert!(body.contains(&format!("name=\"photo\"\r\n\r\n{expected_json}\r\n")));
        assert!(body.contains("name=\"is_public\"\r\n\r\ntrue\r\n"));
        assert!(body.contains(&format!("name=\"{attach_id}\"")) && body.contains("biz-jpg-bytes"));
    }

    #[tokio::test]
    async fn test_multipart_integer_keyed_map() {
        use std::collections::BTreeMap;

        use futures::TryStreamExt;
        use serde::Serialize;

        use crate::requests::{MultipartPayload, Payload};

        #[derive(Serialize)]
        struct MapPayload {
            map: BTreeMap<u32, String>,
        }

        impl Payload for MapPayload {
            type Output = ();
            const NAME: &'static str = "TestMapPayload";
        }

        impl MultipartPayload for MapPayload {
            fn copy_files(&self, _into: &mut dyn FnMut(InputFile)) {}

            fn move_files(&mut self, _into: &mut dyn FnMut(InputFile)) {}
        }

        let mut map = BTreeMap::new();
        map.insert(1, "one".to_owned());
        map.insert(2, "two".to_owned());

        let form = to_form_ref(&MapPayload { map }).unwrap().await;
        let body = String::from_utf8(
            form.into_stream().try_collect::<Vec<bytes::Bytes>>().await.unwrap().concat(),
        )
        .unwrap();
        assert!(
            body.contains("name=\"map\"\r\n\r\n{\"1\":\"one\",\"2\":\"two\"}\r\n"),
            "unexpected multipart map body:\n{body}"
        );
    }

    fn entities() -> impl Iterator<Item = MessageEntity> {
        <_>::into_iter([
            MessageEntity::new(MessageEntityKind::Url, 0, 0),
            MessageEntity::new(MessageEntityKind::Pre { language: None }, 0, 0),
            MessageEntity::new(MessageEntityKind::Pre { language: Some(String::new()) }, 0, 0),
            MessageEntity::new(MessageEntityKind::Url, 0, 0),
            MessageEntity::new(
                MessageEntityKind::TextLink { url: "https://example.com".parse().unwrap() },
                0,
                0,
            ),
        ])
    }
}
