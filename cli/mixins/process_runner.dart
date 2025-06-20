import "dart:convert";
import "dart:io";

import "package:mason_logger/mason_logger.dart";

import "../cli_command.dart";

mixin ProcessRunner on CliCommand {
  Future<int> runProcess(
    String executable,
    List<String> arguments, {
    String? workingDirectory,
    Map<String, String>? environment,
    Logger? logger,
  }) async {
    Process process = await Process.start(
      executable,
      arguments,
      workingDirectory: workingDirectory,
      environment: environment,
    );

    final List<String> errs = [];

    process.stdout.transform(utf8.decoder).transform(LineSplitter()).listen((
      out,
    ) {
      logger?.detail(out);
    });

    process.stderr.transform(utf8.decoder).transform(LineSplitter()).listen((
      out,
    ) {
      errs.add(out);
    });

    int exitCode = await process.exitCode;

    for (String err in errs) {
      if (exitCode != ExitCode.success.code) {
        logger?.err(err);
      } else {
        logger?.detail(err);
      }
    }

    return process.exitCode;
  }
}
