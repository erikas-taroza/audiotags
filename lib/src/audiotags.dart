import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import './ffi.dart';

class AudioTags
{
    /// Read the metadata at the given path. Returns
    /// a [Tag] or null if there is no metadata.
    static Future<Tag?> read(String path) async
    {
        try {
            return await api.read(path: path);
        }
        on FfiException catch (err) {
            // If there is an error because there is no tag,
            // then return null. Other errors will be thrown.
            if(err.message == "ERR: This file does not have any tags.") return null;
            rethrow;
        }
    }

    /// Write the metadata at the given path. Previous metadata will
    /// be cleared and the metadata in [tag] will be written.
    static Future<void> write(String path, Tag tag) async
    {
        return await api.write(path: path, data: tag);
    }
}