use audiotags::{Tag as atag, Album, Picture};

#[derive(Debug)]
pub struct Error(pub String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "There was an error with audiotags:\n {}", self.0)
    }
}

impl std::error::Error for Error {}

#[derive(Default)]
pub struct Tag {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub year: Option<i32>,
    pub genre: Option<String>,
    pub duration: Option<f64>,
    pub picture: Option<Vec<u8>>,
}

pub fn read(path: String) -> Result<Tag, Error> {
    let tag = atag::default().read_from_path(path);

    match tag {
        Err(err) => return Err(Error(format!("{err}"))),

        Ok(tag) => {
            Ok(Tag {
                title: tag.title().map(|f| f.to_string()),
                artist: tag.artist().map(|f| f.to_string()),

                album: if let Some(album) = tag.album() {
                    Some(album.title.to_string())
                } else { None },

                year: tag.year(),
                genre: tag.genre().map(|f| f.to_string()),
                duration: tag.duration(),

                picture: if let Some(pic) = tag.album_cover() {
                    Some(pic.data.to_vec())
                } else { None },
            })
        }
    }
}

pub fn write(path: String, data: Tag) -> Result<(), Error> {
    let mut tag = atag::new().read_from_path(path.clone()).unwrap();

    if data.title.is_some() {
        tag.set_title(data.title.unwrap().as_str());
    }
    if data.artist.is_some() {
        tag.set_artist(data.artist.unwrap().as_str());
    }
    if data.album.is_some() {
        tag.set_album(Album::with_title(data.album.unwrap().as_str()));
    }
    if data.year.is_some() {
        tag.set_year(data.year.unwrap());
    }
    if data.genre.is_some() {
        tag.set_genre(data.genre.unwrap().as_str());
    }

    if data.picture.is_some() {
        let picture = Picture {
            mime_type: audiotags::MimeType::Jpeg,
            data: &data.picture.unwrap(),
        };

        tag.set_album_cover(picture);
    }

    let result = tag.write_to_path(path.as_str());
    match result {
        Err(err) => return Err(Error(format!("{err}"))),
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
                duration: Some(777_f64),
                picture: Some(vec![255]),
            },
        )
        .expect("Failed to write tag.");
    }
}
