import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'id3tags_method_channel.dart';

abstract class Id3tagsPlatform extends PlatformInterface {
  /// Constructs a Id3tagsPlatform.
  Id3tagsPlatform() : super(token: _token);

  static final Object _token = Object();

  static Id3tagsPlatform _instance = MethodChannelId3tags();

  /// The default instance of [Id3tagsPlatform] to use.
  ///
  /// Defaults to [MethodChannelId3tags].
  static Id3tagsPlatform get instance => _instance;

  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [Id3tagsPlatform] when
  /// they register themselves.
  static set instance(Id3tagsPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }
}
