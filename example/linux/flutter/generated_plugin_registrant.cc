//
//  Generated file. Do not edit.
//

// clang-format off

#include "generated_plugin_registrant.h"

#include <id3tags/id3tags_plugin.h>

void fl_register_plugins(FlPluginRegistry* registry) {
  g_autoptr(FlPluginRegistrar) id3tags_registrar =
      fl_plugin_registry_get_registrar_for_plugin(registry, "Id3tagsPlugin");
  id3tags_plugin_register_with_registrar(id3tags_registrar);
}
