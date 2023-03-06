import 'dart:io';
import 'dart:math';

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

class _MyAppState extends State<MyApp>
{
    String path = "";

    @override
    void initState()
    { super.initState(); }

    @override
    Widget build(BuildContext context)
    {
        return MaterialApp(
            home: Scaffold(
                appBar: AppBar(
                    title: const Text('Audiotags Example'),
                ),
                body: Center(
                    child: Column(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: [
                            ElevatedButton(
                                child: const Text("Open"),
                                onPressed: () async {
                                    if(Platform.isAndroid || Platform.isIOS) await Permission.storage.request();
                                    FilePickerResult? r = await FilePicker.platform.pickFiles();
                                    if(r != null) path = r.files.single.path!;
                                },
                            ),
                            const SizedBox(height: 10),
                            ElevatedButton(
                                child: const Text("Write"),
                                onPressed: () {
                                    Tag tag = Tag(
                                        title: "Title",
                                        artist: "Artist",
                                        album: "Album",
                                        genre: "Genre",
                                        year: 2000,
                                        picture: Uint8List.fromList([0, 0, 0, 0])
                                    );
                                    AudioTags.write(path, tag);
                                },
                            ),
                            const SizedBox(height: 10),
                            ElevatedButton(
                                child: const Text("Read"),
                                onPressed: () async {
                                    Tag? tag = await AudioTags.read(path);
                                    String? title = tag?.title;
                                    String? artist = tag?.artist;
                                    String? album = tag?.album;
                                    String? genre = tag?.genre;
                                    int? year = tag?.year;
                                    int? duration = tag?.duration;
                                    List<int>? pictureBytes = tag?.picture;

                                    debugPrint(title);
                                    debugPrint(artist);
                                    debugPrint(album);
                                    debugPrint(genre);
                                    debugPrint(year.toString());
                                    debugPrint(duration.toString());
                                    debugPrint(pictureBytes?.sublist(0, min(pictureBytes.length, 10)).toString());
                                },
                            ),
                        ],
                    )
                ),
            ),
        );
    }
}