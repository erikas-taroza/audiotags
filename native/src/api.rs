use id3::{Tag as id3Tag, TagLike, frame::{Picture, PictureType}, Frame, Content};

use crate::ID3Error;

#[derive(Default)]
pub struct Tag
{
    path:String,
    internal_tag:Option<id3Tag>,
    title:Option<String>,
    artist:Option<String>,
    album:Option<String>,
    year:Option<i32>,
    genre:Option<String>,
    duration:Option<u32>,
    picture:Option<Vec<u8>>
}

impl Tag
{
    pub fn new(path:&str) -> Self
    {
        Tag
        {
            path: path.to_string(),
            internal_tag: Some(id3Tag::new()),
            title: Some(String::default()),
            artist: Some(String::default()),
            album: Some(String::default()),
            year: Some(i32::default()),
            genre: Some(String::default()),
            duration: Some(u32::default()),
            picture: Some(Vec::default())
        }
    }

    pub fn read(&mut self) -> Result<(), ID3Error>
    {
        let id3tag = id3Tag::read_from_path(self.path.as_str());

        match id3tag
        {
            Err(err) => {
                return Err(ID3Error(format!("{}: {}", err.kind.to_string(), err.description)));
            },

            Ok(tag) => {
                self.title = tag.title().map(|f| f.to_string());
                self.artist = tag.artist().map(|f| f.to_string());
                self.album = tag.album().map(|f| f.to_string());
                self.year = tag.year();
                self.genre = tag.genre().map(|f| f.to_string());
                self.duration = tag.duration();

                let pictures = tag.pictures().collect::<Vec<_>>();
                self.picture = if pictures.len() > 0 {
                    Some(pictures.get(0).unwrap().data.clone())
                } else { None };

                Ok(())
            }
        }
    }

    pub fn write(&self, path:&str) -> Result<(), ID3Error>
    {
        if let Some(internal_tag) = &self.internal_tag
        {
            let result = internal_tag.write_to_path(path, id3::Version::Id3v24);
            
            match result
            {
                Err(err) => {
                    return Err(ID3Error(format!("{}: {}", err.kind.to_string(), err.description)));
                }
                Ok(()) => return Ok(())
            }
        }

        Err(ID3Error("Failed to write metadata.".to_string()))
    }

    pub fn set_title(&mut self, title:String)
    {
        self.title = Some(title.clone());
        if let Some(internal_tag) = &mut self.internal_tag
        { internal_tag.set_title(title); }
    }

    pub fn get_title(&self) -> &Option<String>
    { &self.title }

    pub fn set_artist(&mut self, artist:String)
    {
        self.artist = Some(artist.clone());
        if let Some(internal_tag) = &mut self.internal_tag
        { internal_tag.set_artist(artist); }
    }

    pub fn get_artist(&self) -> &Option<String>
    { &self.artist }

    pub fn set_album(&mut self, album:String)
    {
        self.album = Some(album.clone());
        if let Some(internal_tag) = &mut self.internal_tag
        { internal_tag.set_album(album); }
    }

    pub fn get_album(&self) -> &Option<String>
    { &self.album }

    pub fn set_year(&mut self, year:i32)
    {
        self.year = Some(year.clone());
        if let Some(internal_tag) = &mut self.internal_tag
        { internal_tag.set_year(year); }
    }

    pub fn get_year(&self) -> &Option<i32>
    { &self.year }

    pub fn set_genre(&mut self, genre:String)
    {
        self.genre = Some(genre.clone());
        if let Some(internal_tag) = &mut self.internal_tag
        { internal_tag.set_genre(genre); }
    }

    pub fn get_genre(&self) -> &Option<String>
    { &self.genre }

    pub fn set_duration(&mut self, duration:u32)
    {
        self.duration = Some(duration.clone());
        if let Some(internal_tag) = &mut self.internal_tag
        { internal_tag.set_duration(duration); }
    }

    pub fn get_duration(&self) -> &Option<u32>
    { &self.duration }

    pub fn set_picture(&mut self, picture:Vec<u8>)
    {
        self.picture = Some(picture.clone());
        if let Some(internal_tag) = &mut self.internal_tag
        {
            let picture = Picture {
                mime_type: "image/jpeg".to_string(),
                picture_type: PictureType::CoverFront,
                description: "Artwork".to_string(),
                data: picture
            };

            internal_tag.add_frame(Frame::with_content("APIC", Content::Picture(picture)));
        }
    }

    pub fn get_picture(&self) -> &Option<Vec<u8>>
    { &self.picture }
}


/// TEST

#[cfg(test)]
mod tests {
    use super::*;

    const PATH:&str = "/home/erikas/Music/test2.mp3";

    #[test]
    fn read()
    {
        let mut tag = Tag::new(PATH);
        tag.read().unwrap();

        println!("{:?}", tag.get_title());
        println!("{:?}", tag.get_artist());
        println!("{:?}", tag.get_album());
        println!("{:?}", tag.get_year());
        println!("{:?}", tag.get_genre());
        println!("{:?}", tag.get_duration());
        println!("{:?}", tag.get_picture());
    }

    #[test]
    fn write()
    {
        const PATH:&str = "/home/erikas/Music/test2.mp3";
        let mut tag = Tag::new(PATH);
        tag.read().unwrap();

        tag.set_title("This is a title.".to_string());
        tag.set_artist("This is an artist.".to_string());
        tag.set_album("This is an album.".to_string());
        tag.set_year(2022);
        tag.set_genre("This is a genre.".to_string());
        tag.set_duration(20);
        tag.set_picture(std::fs::read("/home/erikas/Downloads/pfp.jpg").unwrap());
        tag.write(PATH).unwrap();
    }
}