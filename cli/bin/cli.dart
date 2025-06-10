import "package:args/command_runner.dart";

import "../commands/build/build.dart";
import "../commands/codegen.dart";
import "../commands/publish.dart";
import "../commands/update.dart";
import "../mixins/package_info.dart";

void main(List<String> args) {
  final CommandRunner<int> runner = CommandRunner(
    "package",
    "A tool to help with maintaining the package.",
  );

  PackageInfo.init();

  runner
    ..addCommand(BuildCommand())
    ..addCommand(CodegenCommand())
    ..addCommand(UpdateCommand())
    ..addCommand(PublishCommand())
    ..run(args);
}
