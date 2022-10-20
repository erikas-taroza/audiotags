use id3::{
    frame::{Picture, PictureType},
    Content, Frame, Tag as id3Tag, TagLike,
};

use crate::ID3Error;

#[derive(Default)]
pub struct Tag {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    year: Option<i32>,
    genre: Option<String>,
    duration: Option<u32>,
    picture: Option<Vec<u8>>,
}

pub fn read(path: String) -> Result<Tag, ID3Error> {
    let id3tag = id3Tag::read_from_path(path);

    match id3tag {
        Err(err) => {
            return Err(ID3Error(format!(
                "{}: {}",
                err.kind.to_string(),
                err.description
            )));
        }

        Ok(tag) => {
            let pictures = tag.pictures().collect::<Vec<_>>();
            Ok(Tag {
                title: tag.title().map(|f| f.to_string()),
                artist: tag.artist().map(|f| f.to_string()),
                album: tag.album().map(|f| f.to_string()),
                year: tag.year(),
                genre: tag.genre().map(|f| f.to_string()),
                duration: tag.duration(),
                picture: if pictures.len() > 0 {
                    Some(pictures.get(0).unwrap().data.clone())
                } else {
                    None
                },
            })
        }
    }
}

pub fn write(path: String, data: Tag) -> Result<(), ID3Error> {
    let mut tag: id3Tag = id3Tag::new();

    if data.title.is_some() {
        tag.set_title(data.title.unwrap());
    }
    if data.artist.is_some() {
        tag.set_artist(data.artist.unwrap());
    }
    if data.album.is_some() {
        tag.set_album(data.album.unwrap());
    }
    if data.year.is_some() {
        tag.set_year(data.year.unwrap());
    }
    if data.genre.is_some() {
        tag.set_genre(data.genre.unwrap());
    }
    if data.duration.is_some() {
        tag.set_duration(data.duration.unwrap());
    }

    if data.picture.is_some() {
        let picture = Picture {
            mime_type: "image/jpeg".to_string(),
            picture_type: PictureType::CoverFront,
            description: "Artwork".to_string(),
            data: data.picture.unwrap(),
        };

        tag.add_frame(Frame::with_content("APIC", Content::Picture(picture)));
    }

    let result = tag.write_to_path(path, id3::Version::Id3v24);
    match result {
        Err(err) => {
            return Err(ID3Error(format!(
                "{}: {}",
                err.kind.to_string(),
                err.description
            )))
        }
        Ok(()) => return Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use crate::api::read;

    use super::*;

    #[test]
    fn read_tag() {
        let tag = read("/home/erikas/Music/test2.mp3".to_string()).expect("Could not read tag.");

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
            "/home/erikas/Music/test2.mp3".to_string(),
            Tag {
                title: Some("Title".to_string()),
                artist: Some("Artist".to_string()),
                album: Some("Album".to_string()),
                year: Some(2022),
                genre: Some("Genre".to_string()),
                duration: Some(777),
                picture: Some(vec![255]),
            },
        )
        .expect("Failed to write tag.");
    }
}
