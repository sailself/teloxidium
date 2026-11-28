use crate::{
    payloads,
    requests::Payload,
    types::{
        InputFile, InputFileLike, InputMedia, InputPaidMedia, InputPollMedia, InputPollOptionMedia,
        InputSticker,
    },
};

/// Payloads that need to be sent as `multipart/form-data` because they contain
/// files inside.
pub trait MultipartPayload: Payload {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile));

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile));
}

impl MultipartPayload for payloads::SendPaidMedia {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter().flat_map(InputPaidMedia::files).for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter_mut().flat_map(InputPaidMedia::files_mut).for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::SendMediaGroup {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter().flat_map(InputMedia::files).for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter_mut().flat_map(InputMedia::files_mut).for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::EditMessageMedia {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.files().for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.files_mut().for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::EditMessageMediaInline {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.files().for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.files_mut().for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::CreateNewStickerSet {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.stickers
            .iter()
            .for_each(|InputSticker { sticker: f, .. }: &InputSticker| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.stickers
            .iter_mut()
            .for_each(|InputSticker { sticker: f, .. }: &mut InputSticker| f.move_into(into))
    }
}

impl MultipartPayload for payloads::SendPoll {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.options
            .iter()
            .filter_map(|opt| opt.media.as_ref())
            .flat_map(InputPollOptionMedia::files)
            .chain(self.media.iter().flat_map(InputPollMedia::files))
            .chain(self.explanation_media.iter().flat_map(InputPollMedia::files))
            .for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.options
            .iter_mut()
            .filter_map(|opt| opt.media.as_mut())
            .flat_map(InputPollOptionMedia::files_mut)
            .chain(self.media.iter_mut().flat_map(InputPollMedia::files_mut))
            .chain(self.explanation_media.iter_mut().flat_map(InputPollMedia::files_mut))
            .for_each(|f| f.move_into(into))
    }
}
