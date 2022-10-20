import 'id3tags_platform_interface.dart';

class Id3tags {
  Future<String?> getPlatformVersion() {
    return Id3tagsPlatform.instance.getPlatformVersion();
  }
}
