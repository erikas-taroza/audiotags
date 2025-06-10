import 'dart:io';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:audiotags/audiotags.dart';
import 'package:permission_handler/permission_handler.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  String path = "";

  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('AudioTags Example'),
        ),
        body: Center(
            child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            ElevatedButton(
              child: const Text("Open"),
              onPressed: () async {
                if (Platform.isAndroid || Platform.isIOS) {
                  await Permission.storage.request();
                }
                FilePickerResult? r = await FilePicker.platform.pickFiles();
                if (r != null) path = r.files.single.path!;
              },
            ),
            const SizedBox(height: 10),
            ElevatedButton(
              child: const Text("Write"),
              onPressed: () async {
                Tag tag = Tag(
                    title: "Title",
                    trackArtist: "Track Artist",
                    album: "Album",
                    albumArtist: "Album Artist",
                    genre: "Genre",
                    year: 2000,
                    trackNumber: 1,
                    trackTotal: 2,
                    discNumber: 1,
                    discTotal: 3,
                    pictures: [
                      Picture(
                          bytes: Uint8List.fromList([0, 0, 0, 0]),
                          mimeType: null,
                          pictureType: PictureType.other)
                    ]);
                AudioTags.write(path, tag);
              },
            ),
            const SizedBox(height: 10),
            ElevatedButton(
              child: const Text("Read"),
              onPressed: () async {
                Tag? tag = await AudioTags.read(path);
                String? title = tag?.title;
                String? trackArtist = tag?.trackArtist;
                String? album = tag?.album;
                String? albumArtist = tag?.albumArtist;
                String? genre = tag?.genre;
                int? year = tag?.year;
                int? trackNumber = tag?.trackNumber;
                int? trackTotal = tag?.trackTotal;
                int? discNumber = tag?.discNumber;
                int? discTotal = tag?.discTotal;
                int? duration = tag?.duration;
                List<Picture>? pictures = tag?.pictures;

                debugPrint("Title: $title");
                debugPrint("Track Artist: $trackArtist");
                debugPrint("Album: $album");
                debugPrint("Album Artist: $albumArtist");
                debugPrint("Genre: $genre");
                debugPrint("Year: ${year.toString()}");
                debugPrint("Track Number: ${trackNumber.toString()}");
                debugPrint("Track Total: ${trackTotal.toString()}");
                debugPrint("Disc Number: ${discNumber.toString()}");
                debugPrint("Disc Total: ${discTotal.toString()}");
                debugPrint("Duration: ${duration.toString()}");
                debugPrint("Pictures: $pictures");
              },
            ),
          ],
        )),
      ),
    );
  }
}
