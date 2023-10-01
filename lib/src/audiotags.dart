import './ffi.dart';

export 'bridge_definitions.dart'
    show
        Tag,
        Picture,
        MimeType,
        PictureType,
        AudioTagsError,
        AudioTagsError_InvalidPath,
        AudioTagsError_NoTags,
        AudioTagsError_OpenFile,
        AudioTagsError_Write;

class AudioTags {
  /// Read the metadata at the given path. Returns
  /// a [Tag] or null if there is no metadata.
  static Future<Tag?> read(String path) async {
    try {
      return await api.read(path: path);
    } on AudioTagsError catch (e) {
      switch (e) {
        case AudioTagsError_NoTags():
          return null;
        default:
          rethrow;
      }
    }
  }

  /// Write the metadata at the given path. Previous metadata will
  /// be cleared and the metadata in [tag] will be written.
  ///
  /// Can throw a [AudioTagsError].
  static Future<void> write(String path, Tag tag) async {
    return await api.write(path: path, data: tag);
  }
}
