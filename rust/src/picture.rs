use std::fmt::Debug;

pub struct Picture {
    pub picture_type: PictureType,
    pub mime_type: MimeType,
    pub bytes: Vec<u8>,
}

impl Picture {
    pub fn new(picture_type: PictureType, mime_type: MimeType, bytes: Vec<u8>) -> Self {
        Picture {
            picture_type,
            mime_type,
            bytes,
        }
    }
}

impl Debug for Picture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Picture")
            .field("picture_type", &self.picture_type)
            .field("format", &self.mime_type)
            .field("bytes", &self.bytes.len())
            .finish()
    }
}

// Almost the same as lofty's PictureType except without
// the undefined type.
/// The type of picture of the song.
#[derive(Debug)]
pub enum PictureType {
    Other,
    Icon,
    OtherIcon,
    CoverFront,
    CoverBack,
    Leaflet,
    Media,
    LeadArtist,
    Artist,
    Conductor,
    Band,
    Composer,
    Lyricist,
    RecordingLocation,
    DuringRecording,
    DuringPerformance,
    ScreenCapture,
    BrightFish,
    Illustration,
    BandLogo,
    PublisherLogo,
}

impl From<lofty::PictureType> for PictureType {
    fn from(value: lofty::PictureType) -> Self {
        match value {
            lofty::PictureType::Other => PictureType::Other,
            lofty::PictureType::Icon => PictureType::Icon,
            lofty::PictureType::OtherIcon => PictureType::OtherIcon,
            lofty::PictureType::CoverFront => PictureType::CoverFront,
            lofty::PictureType::CoverBack => PictureType::CoverBack,
            lofty::PictureType::Leaflet => PictureType::Leaflet,
            lofty::PictureType::Media => PictureType::Media,
            lofty::PictureType::LeadArtist => PictureType::LeadArtist,
            lofty::PictureType::Artist => PictureType::Artist,
            lofty::PictureType::Conductor => PictureType::Conductor,
            lofty::PictureType::Band => PictureType::Band,
            lofty::PictureType::Composer => PictureType::Composer,
            lofty::PictureType::Lyricist => PictureType::Lyricist,
            lofty::PictureType::RecordingLocation => PictureType::RecordingLocation,
            lofty::PictureType::DuringRecording => PictureType::DuringRecording,
            lofty::PictureType::DuringPerformance => PictureType::DuringPerformance,
            lofty::PictureType::ScreenCapture => PictureType::ScreenCapture,
            lofty::PictureType::BrightFish => PictureType::BrightFish,
            lofty::PictureType::Illustration => PictureType::Illustration,
            lofty::PictureType::BandLogo => PictureType::BandLogo,
            lofty::PictureType::PublisherLogo => PictureType::PublisherLogo,
            _ => PictureType::Other,
        }
    }
}

impl From<PictureType> for lofty::PictureType {
    fn from(value: PictureType) -> Self {
        match value {
            PictureType::Other => lofty::PictureType::Other,
            PictureType::Icon => lofty::PictureType::Icon,
            PictureType::OtherIcon => lofty::PictureType::OtherIcon,
            PictureType::CoverFront => lofty::PictureType::CoverFront,
            PictureType::CoverBack => lofty::PictureType::CoverBack,
            PictureType::Leaflet => lofty::PictureType::Leaflet,
            PictureType::Media => lofty::PictureType::Media,
            PictureType::LeadArtist => lofty::PictureType::LeadArtist,
            PictureType::Artist => lofty::PictureType::Artist,
            PictureType::Conductor => lofty::PictureType::Conductor,
            PictureType::Band => lofty::PictureType::Band,
            PictureType::Composer => lofty::PictureType::Composer,
            PictureType::Lyricist => lofty::PictureType::Lyricist,
            PictureType::RecordingLocation => lofty::PictureType::RecordingLocation,
            PictureType::DuringRecording => lofty::PictureType::DuringRecording,
            PictureType::DuringPerformance => lofty::PictureType::DuringPerformance,
            PictureType::ScreenCapture => lofty::PictureType::ScreenCapture,
            PictureType::BrightFish => lofty::PictureType::BrightFish,
            PictureType::Illustration => lofty::PictureType::Illustration,
            PictureType::BandLogo => lofty::PictureType::BandLogo,
            PictureType::PublisherLogo => lofty::PictureType::PublisherLogo,
        }
    }
}

// The same as lofty's MimeType.
// TODO: Support unknown type (code gen doesn't work for it) https://github.com/fzyzcjy/flutter_rust_bridge/issues/1073
/// The MIME type of the picture.
#[derive(Debug)]
pub enum MimeType {
    Png,
    Jpeg,
    Tiff,
    Bmp,
    Gif,
    /// An unknown mimetype. Represented by the string.
    // Unknown(String),
    None,
}

impl From<lofty::MimeType> for MimeType {
    fn from(value: lofty::MimeType) -> Self {
        match value {
            lofty::MimeType::Png => MimeType::Png,
            lofty::MimeType::Jpeg => MimeType::Jpeg,
            lofty::MimeType::Tiff => MimeType::Tiff,
            lofty::MimeType::Bmp => MimeType::Bmp,
            lofty::MimeType::Gif => MimeType::Gif,
            // lofty::MimeType::Unknown(mimetype) => MimeType::Unknown(mimetype),
            _ => MimeType::None,
        }
    }
}

impl From<MimeType> for lofty::MimeType {
    fn from(value: MimeType) -> lofty::MimeType {
        match value {
            MimeType::Png => lofty::MimeType::Png,
            MimeType::Jpeg => lofty::MimeType::Jpeg,
            MimeType::Tiff => lofty::MimeType::Tiff,
            MimeType::Bmp => lofty::MimeType::Bmp,
            MimeType::Gif => lofty::MimeType::Gif,
            // lofty::MimeType::Unknown(mimetype) => MimeType::Unknown(mimetype),
            _ => lofty::MimeType::None,
        }
    }
}
