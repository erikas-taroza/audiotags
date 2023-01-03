import Cocoa
import FlutterMacOS

public class AudiotagsPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel = FlutterMethodChannel(name: "audiotags", binaryMessenger: registrar.messenger)
    let instance = AudiotagsPlugin()
    registrar.addMethodCallDelegate(instance, channel: channel)

    let _ = dummy()
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    switch call.method {
    case "getPlatformVersion":
      result("macOS " + ProcessInfo.processInfo.operatingSystemVersionString)
    default:
      result(FlutterMethodNotImplemented)
    }
  }

  public static func dummy() -> Int64
  { return dummy_method_to_enforce_bundling() }
}
