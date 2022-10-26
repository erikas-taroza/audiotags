use id3::{
    frame::{Picture, PictureType},
    Content, Frame, Tag as id3Tag, TagLike
};

use crate::api::{Tag, Error};

pub fn read(path:&String) -> Result<Tag, Error>
{
    let id3tag = id3Tag::read_from_path(path);

    match id3tag
    {
        Err(err) => Err(Error(format!("{err}"))),
        Ok(id3tag) => {
            let pictures = id3tag.pictures().collect::<Vec<_>>();
            
            Ok(Tag {
                title: id3tag.title().map(|f| f.to_string()),
                artist: id3tag.artist().map(|f| f.to_string()),
                album: id3tag.album().map(|f| f.to_string()),
                year: id3tag.year(),
                genre: id3tag.genre().map(|f| f.to_string()),

                duration: if let Some(dur) = id3tag.duration() {
                    Some(dur as f64)
                } else { None },

                picture: if pictures.len() > 0 {
                    Some(pictures.get(0).unwrap().data.clone())
                } else { None },
            })
        }
    }
}

pub fn write(path:&String, data:Tag) -> Result<(), Error>
{
    let mut id3tag = id3Tag::new();

    if data.title.is_some() {
        id3tag.set_title(data.title.unwrap());
    }

    if data.artist.is_some() {
        id3tag.set_artist(data.artist.unwrap());
    }

    if data.album.is_some() {
        id3tag.set_album(data.album.unwrap());
    }

    if data.year.is_some() {
        id3tag.set_year(data.year.unwrap());
    }

    if data.genre.is_some() {
        id3tag.set_genre(data.genre.unwrap());
    }

    if data.picture.is_some()
    {
        let picture = Picture {
            mime_type: "image/jpeg".to_string(),
            picture_type: PictureType::CoverFront,
            description: "Artwork".to_string(),
            data: data.picture.unwrap(),
        };

        id3tag.add_frame(Frame::with_content("APIC", Content::Picture(picture)));
    }

    let result = id3tag.write_to_path(path, id3::Version::Id3v24);
    match result
    {
        Err(err) => Err(Error(format!("{err}"))),
        Ok(()) => return Ok(())
    }
}