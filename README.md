# Audiotags

Read and write audio metadata in Flutter. Supports multiple formats.

## Usage
### Read
```dart
Tag? tag = await AudioTags.read(path);
String? title = tag?.title;
String? artist = tag?.artist;
String? album = tag?.album;
String? genre = tag?.genre;
int? year = tag?.year;
int? duration = tag?.duration;
List<Picture>? pictures = tag?.pictures;
```

### Write
```dart
Tag tag = Tag(
    title: "Title",
    artist: "Artist",
    album: "Album",
    genre: "Genre",
    year: 2000,
    pictures: [
        Picture(
            bytes: Uint8List.fromList([0, 0, 0, 0]),
            mimeType: MimeType.None,
            pictureType: PictureType.Other
        )
    ]
);
AudioTags.write(path, tag);
```

## Supported Formats
This plugin uses a Rust crate called [`lofty`](https://github.com/Serial-ATA/lofty-rs) to write and read metadata.

The supported formats are listed [here](https://github.com/Serial-ATA/lofty-rs/blob/main/SUPPORTED_FORMATS.md).