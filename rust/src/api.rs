use anyhow::anyhow;
use lofty::{Accessor, Probe, TaggedFileExt, ItemKey, AudioFile, TaggedFile, Picture, TagExt};

#[derive(Default)]
pub struct Tag
{
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub year: Option<u32>,
    pub genre: Option<String>,
    pub duration: Option<u32>,
    pub picture: Option<Vec<u8>>,
}

impl Tag
{
    /// Returns `true` if the tag has no data.
    fn is_empty(&self) -> bool
    {
        self.title == None
            && self.artist == None
            && self.album == None
            && self.year == None
            && self.genre == None
            && self.duration == None
            && self.picture == None
    }
}

/// Returns a `TaggedFile` at the given path.
fn get_file(path: String) -> anyhow::Result<TaggedFile>
{
    match Probe::open(path)
    {
        Ok(file) => match file.read() {
            Ok(file) => Ok(file),
            Err(_) => Err(anyhow!("ERR: Failed to read metadata of file.")),
        },
        Err(_) => Err(anyhow!("ERR: Invalid path provided.")),
    }
}

pub fn read(path: String) -> anyhow::Result<Tag>
{
    let file = get_file(path)?;

    let tag = match file.primary_tag() {
        Some(primary_tag) => Ok(primary_tag),
        None => match file.first_tag() {
            Some(first_tag) => Ok(first_tag),
            None => Err(anyhow!("ERR: This file does not have any tags.")) 
        },
    }?;

    let duration = file.properties().duration().as_secs() as u32;

    let picture = match tag.get_picture_type(lofty::PictureType::CoverFront)
    {
        Some(picture) => Some(picture.data().to_vec()),
        None => None,
    };

    Ok(Tag
    {
        title: tag.get_string(&ItemKey::TrackTitle).map(|e| e.to_string()),
        artist: tag.get_string(&ItemKey::TrackArtist).map(|e| e.to_string()),
        album: tag.get_string(&ItemKey::AlbumTitle).map(|e| e.to_string()),
        year: tag.year(),
        genre: tag.get_string(&ItemKey::Genre).map(|e| e.to_string()),
        duration: Some(duration),
        picture
    })
}

pub fn write(path: String, data: Tag) -> anyhow::Result<()>
{
    let mut file = get_file(path.clone())?;

    // Remove the existing tag.
    if let Some(_) = file.primary_tag_mut() {
        file.remove(file.primary_tag_type());
    }

    // If there is no data to be written, then return.
    if data.is_empty() { return Ok(()); }

    // Create a new tag.
    file.insert_tag(lofty::Tag::new(file.primary_tag_type()));
    let tag = file.primary_tag_mut().unwrap();

    // Title
    if let Some(title) = data.title {
        tag.insert_text(ItemKey::TrackTitle, title);
    }

    // Artist
    if let Some(artist) = data.artist {
        tag.insert_text(ItemKey::TrackArtist, artist);
    }

    // Album Title
    if let Some(album) = data.album {
        tag.insert_text(ItemKey::AlbumTitle, album);
    }

    // Year
    if let Some(year) = data.year {
        tag.set_year(year);
    }

    // Genre
    if let Some(genre) = data.genre {
        tag.insert_text(ItemKey::Genre, genre);
    }

    // Picture
    if let Some(picture) = data.picture {
        tag.set_picture(
            0,
            Picture::new_unchecked(
                lofty::PictureType::CoverFront,
                lofty::MimeType::Jpeg,
                None,
                picture
            )
        );
    }

    match tag.save_to_path(path)
    {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow!("ERR: Failed to write tag to file.")),
    }
}

#[cfg(test)]
mod tests {
    use crate::api::read;

    use super::*;

    #[test]
    fn read_tag() {
        let tag = read("/home/erikas/Music/1.mp3".to_string()).expect("Could not read tag.");

        println!("{:?}", tag.title);
        println!("{:?}", tag.artist);
        println!("{:?}", tag.album);
        println!("{:?}", tag.year);
        println!("{:?}", tag.genre);
        println!("{:?}", tag.duration);
        println!("{:?}", tag.picture);
    }

    #[test]
    fn write_tag() {
        let _tag = write(
            "/home/erikas/Music/1.mp3".to_string(),
            Tag {
                title: Some("Title".to_string()),
                artist: Some("Artist".to_string()),
                album: Some("Album".to_string()),
                year: Some(2022),
                genre: Some("Genre".to_string()),
                picture: Some(vec![255]),
                ..Default::default()
            },
        )
        .expect("Failed to write tag.");
    }
}
