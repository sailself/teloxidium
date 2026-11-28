use derive_more::derive::From;
use serde::{Deserialize, Serialize};

use crate::types::{
    Animation, Audio, Document, InputFile, InputMediaAnimation, InputMediaAudio,
    InputMediaDocument, InputMediaLivePhoto, InputMediaLocation, InputMediaPhoto,
    InputMediaSticker, InputMediaVenue, InputMediaVideo, LivePhoto, Location, PhotoSize, Sticker,
    Venue, Video,
};

/// Describes media attached to a poll, poll option, or quiz explanation.
///
/// [The official docs](https://core.telegram.org/bots/api#pollmedia).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollMedia {
    /// Media is an animation, information about the animation.
    pub animation: Option<Animation>,

    /// Media is an audio file, information about the file; currently, can't be
    /// received in a poll option.
    pub audio: Option<Audio>,

    /// Media is a general file, information about the file; currently, can't be
    /// received in a poll option.
    pub document: Option<Document>,

    /// Media is a live photo, information about the live photo.
    pub live_photo: Option<LivePhoto>,

    /// Media is a shared location, information about the location.
    pub location: Option<Location>,

    /// Media is a photo, available sizes of the photo.
    pub photo: Option<Vec<PhotoSize>>,

    /// Media is a sticker, information about the sticker; currently, for poll
    /// options only.
    pub sticker: Option<Sticker>,

    /// Media is a venue, information about the venue.
    pub venue: Option<Venue>,

    /// Media is a video, information about the video.
    pub video: Option<Video>,
}

/// This object represents the content of a poll description or a quiz
/// explanation to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpollmedia).
#[derive(Clone, Debug, From, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputPollMedia {
    Animation(InputMediaAnimation),
    Audio(InputMediaAudio),
    Document(InputMediaDocument),
    LivePhoto(InputMediaLivePhoto),
    Location(InputMediaLocation),
    Photo(InputMediaPhoto),
    Venue(InputMediaVenue),
    Video(InputMediaVideo),
}

impl InputPollMedia {
    /// Returns an iterator of all files in this input poll media.
    pub(crate) fn files(&self) -> impl Iterator<Item = &InputFile> {
        let slots: [Option<&InputFile>; 3] = match self {
            Self::Animation(m) => [Some(&m.media), m.thumbnail.as_ref(), None],
            Self::Audio(m) => [Some(&m.media), m.thumbnail.as_ref(), None],
            Self::Document(m) => [Some(&m.media), m.thumbnail.as_ref(), None],
            Self::LivePhoto(m) => [Some(&m.media), Some(&m.photo), None],
            Self::Photo(m) => [Some(&m.media), None, None],
            Self::Video(m) => [Some(&m.media), m.thumbnail.as_ref(), m.cover.as_ref()],
            Self::Location(_) | Self::Venue(_) => [None, None, None],
        };

        slots.into_iter().flatten()
    }

    /// Returns a mutable iterator of all files in this input poll media.
    pub(crate) fn files_mut(&mut self) -> impl Iterator<Item = &mut InputFile> {
        let slots: [Option<&mut InputFile>; 3] = match self {
            Self::Animation(m) => [Some(&mut m.media), m.thumbnail.as_mut(), None],
            Self::Audio(m) => [Some(&mut m.media), m.thumbnail.as_mut(), None],
            Self::Document(m) => [Some(&mut m.media), m.thumbnail.as_mut(), None],
            Self::LivePhoto(m) => [Some(&mut m.media), Some(&mut m.photo), None],
            Self::Photo(m) => [Some(&mut m.media), None, None],
            Self::Video(m) => [Some(&mut m.media), m.thumbnail.as_mut(), m.cover.as_mut()],
            Self::Location(_) | Self::Venue(_) => [None, None, None],
        };

        slots.into_iter().flatten()
    }
}

/// This object represents the content of a poll option to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpolloptionmedia).
#[derive(Clone, Debug, From, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputPollOptionMedia {
    Animation(InputMediaAnimation),
    LivePhoto(InputMediaLivePhoto),
    Location(InputMediaLocation),
    Photo(InputMediaPhoto),
    Sticker(InputMediaSticker),
    Venue(InputMediaVenue),
    Video(InputMediaVideo),
}

impl InputPollOptionMedia {
    /// Returns an iterator of all files in this input poll option media.
    pub(crate) fn files(&self) -> impl Iterator<Item = &InputFile> {
        let slots: [Option<&InputFile>; 3] = match self {
            Self::Animation(m) => [Some(&m.media), m.thumbnail.as_ref(), None],
            Self::LivePhoto(m) => [Some(&m.media), Some(&m.photo), None],
            Self::Photo(m) => [Some(&m.media), None, None],
            Self::Sticker(m) => [Some(&m.media), None, None],
            Self::Video(m) => [Some(&m.media), m.thumbnail.as_ref(), m.cover.as_ref()],
            Self::Location(_) | Self::Venue(_) => [None, None, None],
        };

        slots.into_iter().flatten()
    }

    /// Returns a mutable iterator of all files in this input poll option media.
    pub(crate) fn files_mut(&mut self) -> impl Iterator<Item = &mut InputFile> {
        let slots: [Option<&mut InputFile>; 3] = match self {
            Self::Animation(m) => [Some(&mut m.media), m.thumbnail.as_mut(), None],
            Self::LivePhoto(m) => [Some(&mut m.media), Some(&mut m.photo), None],
            Self::Photo(m) => [Some(&mut m.media), None, None],
            Self::Sticker(m) => [Some(&mut m.media), None, None],
            Self::Video(m) => [Some(&mut m.media), m.thumbnail.as_mut(), m.cover.as_mut()],
            Self::Location(_) | Self::Venue(_) => [None, None, None],
        };

        slots.into_iter().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_poll_media_tagging_and_files() {
        let mut lp = InputPollMedia::from(InputMediaLivePhoto::new(
            InputFile::memory("vid"),
            InputFile::memory("img"),
        ));
        assert_eq!(lp.files().count(), 2);
        assert_eq!(lp.files_mut().count(), 2);
        let lp_json = serde_json::to_value(&lp).unwrap();
        assert_eq!(lp_json["type"], "live_photo");

        let loc =
            InputPollMedia::from(InputMediaLocation::new(12.5, 34.5).horizontal_accuracy(10.0));
        assert_eq!(loc.files().count(), 0);
        let loc_json = serde_json::to_value(&loc).unwrap();
        assert_eq!(loc_json["type"], "location");
        assert_eq!(loc_json["horizontal_accuracy"], 10.0);

        let mut opt_lp = InputPollOptionMedia::from(InputMediaLivePhoto::new(
            InputFile::memory("ov"),
            InputFile::memory("op"),
        ));
        assert_eq!(opt_lp.files().count(), 2);
        assert_eq!(opt_lp.files_mut().count(), 2);

        let venue = InputPollOptionMedia::from(
            InputMediaVenue::new(1.0, 2.0, "Cafe", "123 St").foursquare_id("fs1"),
        );
        assert_eq!(venue.files().count(), 0);
        assert_eq!(serde_json::to_value(&venue).unwrap()["type"], "venue");
    }
}
