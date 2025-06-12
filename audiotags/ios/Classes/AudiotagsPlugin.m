#import "AudiotagsPlugin.h"
#if __has_include(<audiotags/audiotags-Swift.h>)
#import <audiotags/audiotags-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "audiotags-Swift.h"
#endif

@implementation AudiotagsPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftAudiotagsPlugin registerWithRegistrar:registrar];
}
@end
