import "dart:io";

import "package:collection/collection.dart";
import "package:github/github.dart";
import "package:io/io.dart";
import "package:pub_updater/pub_updater.dart";

import "../cli_command.dart";
import "../mixins/cli_logger.dart";
import "../mixins/package_info.dart";
import "../mixins/process_runner.dart";

class PublishCommand extends CliCommand
    with CliLogger, ProcessRunner, PackageInfo {
  @override
  String get name => "publish";

  @override
  List<String> get aliases => ["p"];

  @override
  String get description => "Publishes the package to pub.dev.";

  PublishCommand() {
    argParser.addFlag(
      "dry-run",
      abbr: "d",
      help:
          "Performs the steps to publish the package but doesn't publish to pub.dev.",
    );
  }

  @override
  Future<int> run() async {
    final bool isDryRun = argResults?.flag("dry-run") ?? false;

    logger.info("Checking pub.dev release...");
    // Check if release already exists on pub.
    final PubUpdater pub = PubUpdater();
    final String pubLatestVersion = await pub.getLatestVersion("audiotags");

    if (pubLatestVersion == packageVersion) {
      logger.err(
        "The version $packageVersion is already published to pub.dev.",
      );
      return ExitCode.unavailable.code;
    }

    logger.info("Checking GitHub release...");

    final GitHub github = GitHub(auth: findAuthenticationFromEnvironment());
    final RepositorySlug repo = RepositorySlug("erikas-taroza", "audiotags");
    final Release release = await github.repositories.listReleases(repo).first;

    final String releaseVersion = release.name?.split("v")[1] ?? "";
    logger.detail("Latest release: $releaseVersion");

    if (releaseVersion != packageVersion) {
      logger.err(
        "The version defined in pubspec.yaml ($packageVersion) does not match the version in the GitHub release ($releaseVersion).",
      );
      github.dispose();
      return ExitCode.unavailable.code;
    }

    // Ensure that all the release assets are available.
    final List<String>? releaseAssetsNames =
        release.assets?.map((e) => e.name ?? "").toList();
    final List<String> expectedAssets = [
      "android.tar.gz",
      "linux.tar.gz",
      "windows.tar.gz",
      "ios.zip",
      "macos.zip",
    ];

    if (release.assets == null ||
        releaseAssetsNames == null ||
        !UnorderedIterableEquality<String>().equals(
          releaseAssetsNames,
          expectedAssets,
        )) {
      logger.err("""Not all release assets are published.
Expected: $expectedAssets
Found: $releaseAssetsNames""");
      github.dispose();
      return ExitCode.unavailable.code;
    }

    // Run the pub publish command.
    final Process process = await Process.start(
      "flutter",
      ["pub", "publish", if (isDryRun) "--dry-run"],
      workingDirectory: "$projectRootDirectory/$packageName",
      mode: ProcessStartMode.inheritStdio,
    );

    final int result = await process.exitCode;

    if (result != ExitCode.success.code) {
      github.dispose();
      return result;
    }

    logger.success("Done!");
    github.dispose();
    return ExitCode.success.code;
  }
}
