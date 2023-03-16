mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod picture;
mod tag;

use anyhow::anyhow;
use lofty::{Accessor, AudioFile, ItemKey, Probe, TagExt, TaggedFile, TaggedFileExt};
use tag::Tag;

/// Returns a `TaggedFile` at the given path.
fn get_file(path: &str) -> anyhow::Result<TaggedFile> {
    match Probe::open(path) {
        Ok(file) => match file.read() {
            Ok(file) => Ok(file),
            Err(_) => Err(anyhow!("ERR: Failed to read metadata of file.")),
        },
        Err(_) => Err(anyhow!("ERR: Invalid path provided.")),
    }
}

pub fn read(path: String) -> anyhow::Result<Tag> {
    let file = get_file(&path)?;

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
    use std::io::Read;

    use super::*;

    #[test]
    fn read_tag_mp3() {
        let tag = read("samples/test.mp3".to_string()).expect("Could not read tag.");

        println!("{:?}", tag.title);
        println!("{:?}", tag.artist);
        println!("{:?}", tag.album);
        println!("{:?}", tag.year);
        println!("{:?}", tag.genre);
        println!("{:?}", tag.duration);
        println!("{:?}", tag.pictures);
    }

    #[test]
    fn clear_tag_mp3() {
        write(
            "samples/test.mp3".to_string(),
            Tag {
                title: None,
                artist: None,
                album: None,
                year: None,
                genre: None,
                duration: None,
                pictures: Vec::new(),
            },
        )
        .expect("Could not write tag.");
        read_tag_mp3();
    }

    #[test]
    fn write_tag_mp3() {
        let picture1 = picture::Picture::new(
            picture::PictureType::CoverFront,
            picture::MimeType::Jpeg,
            std::fs::File::open("samples/picture1.jpg")
                .unwrap()
                .bytes()
                .map(|b| b.unwrap())
                .collect(),
        );

        let picture2 = picture::Picture::new(
            picture::PictureType::CoverBack,
            picture::MimeType::Jpeg,
            std::fs::File::open("samples/picture2.jpg")
                .unwrap()
                .bytes()
                .map(|b| b.unwrap())
                .collect(),
        );

        write(
            "samples/test.mp3".to_string(),
            Tag {
                title: Some("Title".to_string()),
                artist: Some("Artist".to_string()),
                album: Some("Album".to_string()),
                year: Some(2022),
                genre: Some("Genre".to_string()),
                pictures: vec![picture1, picture2],
                ..Default::default()
            },
        )
        .expect("Failed to write tag.");

        read_tag_mp3();
    }

    #[test]
    fn read_tag_mp4() {
        let tag = read("samples/test.mp4".to_string()).expect("Could not read tag.");

        println!("{:?}", tag.title);
        println!("{:?}", tag.artist);
        println!("{:?}", tag.album);
        println!("{:?}", tag.year);
        println!("{:?}", tag.genre);
        println!("{:?}", tag.duration);
        println!("{:?}", tag.pictures);
    }

    #[test]
    fn clear_tag_mp4() {
        write(
            "samples/test.mp4".to_string(),
            Tag {
                title: None,
                artist: None,
                album: None,
                year: None,
                genre: None,
                duration: None,
                pictures: Vec::new(),
            },
        )
        .expect("Could not write tag.");
        read_tag_mp4();
    }

    #[test]
    fn write_tag_mp4() {
        let picture1 = picture::Picture::new(
            picture::PictureType::CoverFront,
            picture::MimeType::Jpeg,
            std::fs::File::open("samples/picture1.jpg")
                .unwrap()
                .bytes()
                .map(|b| b.unwrap())
                .collect(),
        );

        let picture2 = picture::Picture::new(
            picture::PictureType::CoverBack,
            picture::MimeType::Jpeg,
            std::fs::File::open("samples/picture2.jpg")
                .unwrap()
                .bytes()
                .map(|b| b.unwrap())
                .collect(),
        );

        write(
            "samples/test.mp4".to_string(),
            Tag {
                title: Some("Title".to_string()),
                artist: Some("Artist".to_string()),
                album: Some("Album".to_string()),
                year: Some(2022),
                genre: Some("Genre".to_string()),
                pictures: vec![picture1, picture2],
                ..Default::default()
            },
        )
        .expect("Failed to write tag.");

        read_tag_mp4();
    }
}
