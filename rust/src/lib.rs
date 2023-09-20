mod api;
mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod picture;
mod tag;

#[cfg(test)]
mod tests {
    use std::io::Read;

    use crate::tag::Tag;

    use super::*;
    use api::*;

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
