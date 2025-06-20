import "dart:io";

import "../../cli_command.dart";
import "subcommands/android.dart";
import "subcommands/ios.dart";
import "subcommands/linux.dart";
import "subcommands/macos.dart";
import "subcommands/windows.dart";

class BuildCommand extends CliCommand {
  @override
  String get name => "build";

  @override
  List<String> get aliases => ["b"];

  @override
  String get description =>
      "Builds the Rust code for the given target.\nThe build targets are listed based on your OS.";

  BuildCommand() {
    addSubcommand(AndroidBuildCommand());

    if (Platform.isLinux) addSubcommand(LinuxBuildCommand());

    if (Platform.isWindows) addSubcommand(WindowsBuildCommand());

    if (Platform.isMacOS) {
      addSubcommand(IosBuildCommand());
      addSubcommand(MacosBuildCommand());
    }
  }
}
