#include "include/id3tags/id3tags_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "id3tags_plugin.h"

void Id3tagsPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  id3tags::Id3tagsPlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
