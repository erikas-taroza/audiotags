import "package:mason_logger/mason_logger.dart";

import "../cli_command.dart";

mixin CliLogger on CliCommand {
  final Logger _logger = Logger();

  Logger get logger {
    _logger.level = argResults?["verbose"] ?? false
        ? Level.verbose
        : Level.info;
    return _logger;
  }
}
