import './ffi.dart';

class AudioTags
{
    /// Read the metadata at the given path. Returns
    /// a [Tag].
    static Future<Tag> read(String path) async
    {
        return await api.read(path: path);
    }

    /// Write the metadata at the given path. Previous metadata will
    /// be cleared and the metadata in [tag] will be written.
    static Future<void> write(String path, Tag tag) async
    {
        return await api.write(path: path, data: tag);
    }
}