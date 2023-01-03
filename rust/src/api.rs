use anyhow::anyhow;

use crate::{id3, mp4};

#[derive(Default)]
pub struct Tag
{
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub year: Option<i32>,
    pub genre: Option<String>,
    pub duration: Option<f64>,
    pub picture: Option<Vec<u8>>,
}

pub fn read(path: String) -> anyhow::Result<Tag>
{
    let extension = path.split(".").last().unwrap();

    match extension
    {
        "mp3" => id3::read(&path),
        "mp4"
        | "m4a"
        | "m4p"
        | "m4b"
        | "m4r"
        | "m4v" => mp4::read(&path),
        _ => Err(anyhow!("ERR: Unsupported file type for reading.".to_string()))
    }
}

pub fn write(path:String, data:Tag) -> anyhow::Result<()>
{
    let extension = path.split(".").last().unwrap();

    match extension
    {
        "mp3" => id3::write(&path, data),
        "mp4"
        | "m4a"
        | "m4p"
        | "m4b"
        | "m4r"
        | "m4v" => mp4::write(&path, data),
        _ => Err(anyhow!("ERR: Unsupported file type for writing.".to_string()))
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
                duration: Some(777_f64),
                picture: Some(vec![255]),
            },
        )
        .expect("Failed to write tag.");
    }
}
