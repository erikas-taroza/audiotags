import Flutter
import UIKit

public class SwiftAudiotagsPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel = FlutterMethodChannel(name: "audiotags", binaryMessenger: registrar.messenger())
    let instance = SwiftAudiotagsPlugin()
    registrar.addMethodCallDelegate(instance, channel: channel)

    let _ = dummy()
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    result("iOS " + UIDevice.current.systemVersion)
  }

  public static func dummy() -> Int64
  { return dummy_method_to_enforce_bundling() }
}
