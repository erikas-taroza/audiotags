use anyhow::anyhow;
use mp4ameta::{self, Img};

use crate::{
    picture::{MimeType, Picture, PictureType},
    Tag,
};

pub fn read(path: &String) -> anyhow::Result<Tag> {
    let mp4tag = mp4ameta::Tag::read_from_path(path);

    match mp4tag {
        Err(err) => match err.kind {
            mp4ameta::ErrorKind::NoTag => Err(anyhow!("ERR: This file does not have any tags.")),
            _ => Err(anyhow!(format!("ERR: {err}"))),
        },
        Ok(mp4tag) => {
            let pictures = mp4tag
                .artworks()
                .map(|picture| {
                    let format = match picture.fmt {
                        mp4ameta::ImgFmt::Bmp => MimeType::Bmp,
                        mp4ameta::ImgFmt::Jpeg => MimeType::Jpeg,
                        mp4ameta::ImgFmt::Png => MimeType::Png,
                    };

                    Picture::new(PictureType::Other, format, picture.data.to_vec())
                })
                .collect::<Vec<Picture>>();

            Ok(Tag {
                title: mp4tag.title().map(|f| f.to_string()),
                artist: mp4tag.artist().map(|f| f.to_string()),
                album: mp4tag.album().map(|f| f.to_string()),
                year: mp4tag.year().map(|f| f.parse::<u32>().unwrap()),
                genre: mp4tag.genre().map(|f| f.to_string()),
                duration: mp4tag.duration().map(|dur| dur.as_secs() as u32),
                pictures,
            })
        }
    }
}

pub fn write(path: &String, data: Tag) -> anyhow::Result<()> {
    let mp4tag = mp4ameta::Tag::read_from_path(path);

    let mut mp4tag = match mp4tag {
        Err(err) => return Err(anyhow!(format!("ERR: {err}"))),
        Ok(mp4tag) => mp4tag,
    };

    mp4tag.clear();

    // If there is no data to be written, then return.
    if data.is_empty() {
        return Ok(());
    }

    if data.title.is_some() {
        mp4tag.set_title(data.title.unwrap());
    }

    if data.artist.is_some() {
        mp4tag.set_artist(data.artist.unwrap());
    }

    if data.album.is_some() {
        mp4tag.set_album(data.album.unwrap());
    }

    if data.year.is_some() {
        mp4tag.set_year(data.year.unwrap().to_string());
    }

    if data.genre.is_some() {
        mp4tag.set_genre(data.genre.unwrap());
    }

    for picture in data.pictures.into_iter() {
        let format = match picture.mime_type {
            MimeType::Bmp => mp4ameta::ImgFmt::Bmp,
            MimeType::Jpeg => mp4ameta::ImgFmt::Jpeg,
            MimeType::Png => mp4ameta::ImgFmt::Png,
            other => {
                return Err(anyhow!(
                    "ERR: The image format {other:?} is not supported for MP4s."
                ))
            }
        };

        mp4tag.add_artwork(Img::new(format, picture.bytes))
    }

    let result = mp4tag.write_to_path(path);
    match result {
        Err(err) => Err(anyhow!(format!("ERR: {err}"))),
        Ok(_) => Ok(()),
    }
}
