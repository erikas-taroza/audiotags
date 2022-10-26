#ifndef FLUTTER_PLUGIN_AUDIOTAGS_PLUGIN_H_
#define FLUTTER_PLUGIN_AUDIOTAGS_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace audiotags {

class AudiotagsPlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar);

  AudiotagsPlugin();

  virtual ~AudiotagsPlugin();

  // Disallow copy and assign.
  AudiotagsPlugin(const AudiotagsPlugin&) = delete;
  AudiotagsPlugin& operator=(const AudiotagsPlugin&) = delete;

 private:
  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

}  // namespace audiotags

#endif  // FLUTTER_PLUGIN_AUDIOTAGS_PLUGIN_H_
