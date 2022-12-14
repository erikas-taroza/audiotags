import './ffi.dart';

class AudioTags
{
    static Future<Tag> read(String path) async
    {
        return await api.read(path: path);
    }

    static Future<void> write(String path, Tag tag) async
    {
        return await api.write(path: path, data: tag);
    }
}