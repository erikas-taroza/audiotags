use crate::picture::Picture;

/// Represents the metadata of the file.
#[derive(Default)]
pub struct Tag {
    /// The title of the song.
    pub title: Option<String>,
    /// The artist of the song.
    pub artist: Option<String>,
    /// The album the song is from.
    pub album: Option<String>,
    /// The year that this song was made.
    pub year: Option<u32>,
    /// The genre of the song.
    pub genre: Option<String>,
    /// The duration of the song. Setting this field
    /// when writing will do nothing.
    pub duration: Option<u32>,
    /// All the pictures of the song.
    pub pictures: Vec<Picture>,
}

impl Tag {
    /// Returns `true` if the tag has no data.
    pub fn is_empty(&self) -> bool {
        self.title.is_none()
            && self.artist.is_none()
            && self.album.is_none()
            && self.year.is_none()
            && self.genre.is_none()
            && self.duration.is_none()
            && self.pictures.is_empty()
    }
}
