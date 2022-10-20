#ifndef FLUTTER_PLUGIN_ID3TAGS_PLUGIN_H_
#define FLUTTER_PLUGIN_ID3TAGS_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace id3tags {

class Id3tagsPlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar);

  Id3tagsPlugin();

  virtual ~Id3tagsPlugin();

  // Disallow copy and assign.
  Id3tagsPlugin(const Id3tagsPlugin&) = delete;
  Id3tagsPlugin& operator=(const Id3tagsPlugin&) = delete;

 private:
  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

}  // namespace id3tags

#endif  // FLUTTER_PLUGIN_ID3TAGS_PLUGIN_H_
