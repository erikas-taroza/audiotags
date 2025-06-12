#include "include/audiotags/audiotags_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "audiotags_plugin.h"

void AudiotagsPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  audiotags::AudiotagsPlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
