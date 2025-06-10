import "dart:io";

import "package:yaml_edit/yaml_edit.dart";

import "../cli_command.dart";

mixin PackageInfo on CliCommand {
  String get frbVersion => "2.7.0";

  String get packageName => _packageName;
  static final String _packageName = "audiotags";

  String get packageVersion => _packageVersion;
  static String _packageVersion = "";

  String get projectRootDirectory => _projectRootDirectory;
  static String _projectRootDirectory = "";

  static void init() {
    if (_projectRootDirectory.isNotEmpty) return;

    _projectRootDirectory = _getProjectRootDirectory();
    _packageVersion = _getPackageVersion();
  }

  static String _getProjectRootDirectory() {
    Directory directory = Directory.current;

    // Starts from the current directory and goes up until it finds the root project directory.
    while (true) {
      if (Directory("${directory.path}/audiotags").existsSync()) {
        return directory.path;
      } else {
        directory = directory.parent;
      }
    }
  }

  static String _getPackageVersion() {
    final File pubspec = File(
      "$_projectRootDirectory/$_packageName/pubspec.yaml",
    );

    final YamlEditor yamlEditor = YamlEditor(pubspec.readAsStringSync());
    return yamlEditor.parseAt(["version"]).value;
  }
}
