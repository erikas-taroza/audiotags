#import "Id3tagsPlugin.h"
#if __has_include(<id3tags/id3tags-Swift.h>)
#import <id3tags/id3tags-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "id3tags-Swift.h"
#endif

@implementation Id3tagsPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftId3tagsPlugin registerWithRegistrar:registrar];
}
@end
