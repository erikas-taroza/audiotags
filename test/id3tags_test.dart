import 'package:flutter_test/flutter_test.dart';
import 'package:id3tags/id3tags.dart';
import 'package:id3tags/id3tags_platform_interface.dart';
import 'package:id3tags/id3tags_method_channel.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

class MockId3tagsPlatform
    with MockPlatformInterfaceMixin
    implements Id3tagsPlatform {

  @override
  Future<String?> getPlatformVersion() => Future.value('42');
}

void main() {
  final Id3tagsPlatform initialPlatform = Id3tagsPlatform.instance;

  test('$MethodChannelId3tags is the default instance', () {
    expect(initialPlatform, isInstanceOf<MethodChannelId3tags>());
  });

  test('getPlatformVersion', () async {
    Id3tags id3tagsPlugin = Id3tags();
    MockId3tagsPlatform fakePlatform = MockId3tagsPlatform();
    Id3tagsPlatform.instance = fakePlatform;

    expect(await id3tagsPlugin.getPlatformVersion(), '42');
  });
}
