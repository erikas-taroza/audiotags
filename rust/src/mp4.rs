use anyhow::anyhow;
use mp4ameta::{self, Img};

use crate::Tag;

pub fn read(path:&String) -> anyhow::Result<Tag>
{
    let mp4tag = mp4ameta::Tag::read_from_path(path);

    match mp4tag
    {
        Err(err) => Err(anyhow!(format!("ERR: {err}"))),
        Ok(mp4tag) => {
            let picture = mp4tag.artwork();

            Ok(Tag {
                title: mp4tag.title().map(|f| f.to_string()),
                artist: mp4tag.artist().map(|f| f.to_string()),
                album: mp4tag.album().map(|f| f.to_string()),
                year: mp4tag.year().map(|f| f.parse::<u32>().unwrap()),
                genre: mp4tag.genre().map(|f| f.to_string()),

                duration: if let Some(dur) = mp4tag.duration() {
                    Some(dur.as_secs() as u32)
                } else { None },

                picture: if let Some(picture) = picture {
                    Some(picture.data.to_vec())
                } else { None }
            })
        }
    }
}

pub fn write(path:&String, data:Tag) -> anyhow::Result<()>
{
    let mp4tag = mp4ameta::Tag::read_from_path(path);

    let mut mp4tag = match mp4tag
    {
        Err(err) => return Err(anyhow!(format!("ERR: {err}"))),
        Ok(mp4tag) => mp4tag
    };

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

    if data.picture.is_some() {
        mp4tag.set_artwork(Img::jpeg(data.picture.unwrap()));
    }

    let result = mp4tag.write_to_path(path);
    match result
    {
        Err(err) => Err(anyhow!(format!("ERR: {err}"))),
        Ok(_) => return Ok(())
    }
}