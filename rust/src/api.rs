use crate::tag::Tag;
use anyhow::anyhow;
use lofty::{Accessor, AudioFile, ItemKey, Probe, TagExt, TaggedFile, TaggedFileExt};

/// Returns a `TaggedFile` at the given path.
fn get_file(path: &str) -> anyhow::Result<TaggedFile> {
    match Probe::open(path) {
        Ok(file) => match file.read() {
            Ok(file) => Ok(file),
            Err(err) => Err(anyhow!("Failed to read metadata of file. {err:?}")),
        },
        Err(err) => Err(anyhow!("Invalid path provided. {err:?}")),
    }
}

pub fn read(path: String) -> anyhow::Result<Tag> {
    let file = get_file(&path)?;

    let tag = match file.primary_tag() {
        Some(primary_tag) => Ok(primary_tag),
        None => match file.first_tag() {
            Some(first_tag) => Ok(first_tag),
            None => Err(anyhow!("This file does not have any tags.")),
        },
    }?;

    let mut tag = Tag::from(tag);
    let duration = file.properties().duration().as_secs() as u32;
    tag.duration = Some(duration);

    Ok(tag)
}

pub fn write(path: String, data: Tag) -> anyhow::Result<()> {
    let mut file = get_file(&path)?;

    // Remove the existing tags.
    for tag in file.tags() {
        tag.remove_from_path(&path)?;
    }

    // If there is no data to be written, then return.
    if data.is_empty() {
        return Ok(());
    }

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

    // Track number
    if let Some(track_number) = data.track_number {
        tag.set_track(track_number);
    }

    // Track total
    if let Some(track_total) = data.track_total {
        tag.set_track_total(track_total);
    }

    // Genre
    if let Some(genre) = data.genre {
        tag.insert_text(ItemKey::Genre, genre);
    }

    // Pictures
    for (i, picture) in data.pictures.into_iter().enumerate() {
        tag.set_picture(
            i,
            lofty::Picture::new_unchecked(
                picture.picture_type.into(),
                picture.mime_type.into(),
                None,
                picture.bytes,
            ),
        );
    }

    match tag.save_to_path(path) {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!("Failed to write tag to file. {err:?}")),
    }
}
