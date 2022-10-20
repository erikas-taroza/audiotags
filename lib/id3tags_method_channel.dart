import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'id3tags_platform_interface.dart';

/// An implementation of [Id3tagsPlatform] that uses method channels.
class MethodChannelId3tags extends Id3tagsPlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('id3tags');

  @override
  Future<String?> getPlatformVersion() async {
    final version = await methodChannel.invokeMethod<String>('getPlatformVersion');
    return version;
  }
}
