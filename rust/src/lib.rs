mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod mp4;
mod picture;
mod tag;

use anyhow::anyhow;
use lofty::{Accessor, AudioFile, ItemKey, Probe, TagExt, TaggedFile, TaggedFileExt};
use tag::Tag;

/// Returns a `TaggedFile` at the given path.
fn get_file(path: String) -> anyhow::Result<TaggedFile> {
    match Probe::open(path) {
        Ok(file) => match file.read() {
            Ok(file) => Ok(file),
            Err(_) => Err(anyhow!("ERR: Failed to read metadata of file.")),
        },
        Err(_) => Err(anyhow!("ERR: Invalid path provided.")),
    }
}

pub fn read(path: String) -> anyhow::Result<Tag> {
    match path.split('.').last().unwrap() {
        "mp4" | "m4a" | "m4p" | "m4b" | "m4r" | "m4v" => return mp4::read(&path),
        _ => (),
    };

    let file = get_file(path)?;

    let tag = match file.primary_tag() {
        Some(primary_tag) => Ok(primary_tag),
        None => match file.first_tag() {
            Some(first_tag) => Ok(first_tag),
            None => Err(anyhow!("ERR: This file does not have any tags.")),
        },
    }?;

    let duration = file.properties().duration().as_secs() as u32;

    let pictures = tag
        .pictures()
        .iter()
        .map(|picture| {
            picture::Picture::new(
                picture.pic_type().into(),
                picture.mime_type().clone().into(),
                picture.data().to_vec(),
            )
        })
        .collect::<Vec<picture::Picture>>();

    Ok(Tag {
        title: tag.get_string(&ItemKey::TrackTitle).map(|e| e.to_string()),
        artist: tag.get_string(&ItemKey::TrackArtist).map(|e| e.to_string()),
        album: tag.get_string(&ItemKey::AlbumTitle).map(|e| e.to_string()),
        year: tag.year(),
        genre: tag.get_string(&ItemKey::Genre).map(|e| e.to_string()),
        duration: Some(duration),
        pictures,
    })
}

pub fn write(path: String, data: Tag) -> anyhow::Result<()> {
    match path.split('.').last().unwrap() {
        "mp4" | "m4a" | "m4p" | "m4b" | "m4r" | "m4v" => return mp4::write(&path, data),
        _ => (),
    };

    let mut file = get_file(path.clone())?;

    // Remove the existing tag.
    file.clear();

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
        Err(_) => Err(anyhow!("ERR: Failed to write tag to file.")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_tag() {
        let tag = read("/home/erikas/Downloads/test.mp3".to_string()).expect("Could not read tag.");

        println!("{:?}", tag.title);
        println!("{:?}", tag.artist);
        println!("{:?}", tag.album);
        println!("{:?}", tag.year);
        println!("{:?}", tag.genre);
        println!("{:?}", tag.duration);
        println!("{:?}", tag.pictures);
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
                // pictures: Some(vec![255]),
                ..Default::default()
            },
        )
        .expect("Failed to write tag.");
    }
}
