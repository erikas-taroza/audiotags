import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:id3tags/id3tags_method_channel.dart';

void main() {
  MethodChannelId3tags platform = MethodChannelId3tags();
  const MethodChannel channel = MethodChannel('id3tags');

  TestWidgetsFlutterBinding.ensureInitialized();

  setUp(() {
    channel.setMockMethodCallHandler((MethodCall methodCall) async {
      return '42';
    });
  });

  tearDown(() {
    channel.setMockMethodCallHandler(null);
  });

  test('getPlatformVersion', () async {
    expect(await platform.getPlatformVersion(), '42');
  });
}
