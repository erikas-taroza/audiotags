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
    //TODO: Enter the path 
    // to the mp3 file here ----v
    String path = "/path/to/song.mp3";

    @override
    void initState()
    { super.initState(); }

    @override
    Widget build(BuildContext context)
    {
        return MaterialApp(
            home: Scaffold(
                appBar: AppBar(
                    title: const Text('Plugin example app'),
                ),
                body: Center(
                    child: Column(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: [
                            ElevatedButton(
                                child: const Text("Write"),
                                onPressed: () => AudioTags.write(path, Tag(
                                    title: "Title",
                                    artist: "Artist",
                                    picture: Uint8List.fromList([0, 0, 0, 0])
                                )),
                            ),
                            const SizedBox(height: 10),
                            ElevatedButton(
                                child: const Text("Read"),
                                onPressed: () async {
                                    Tag tag = await AudioTags.read(path);
                                    debugPrint(tag.title);
                                    debugPrint(tag.artist);
                                    debugPrint(tag.picture.toString());
                                },
                            ),
                            //TODO: You will need to setup permissions for mobile:
                            
                            // const SizedBox(height: 10),
                            // ElevatedButton(
                            //     child: const Text("Perms"),
                            //     onPressed: () async {
                            //         var result = await Permission.storage.request();
                            //         debugPrint(result.toString());
                            //         // Testing QOL
                            //         FilePickerResult? r = await FilePicker.platform.pickFiles();
                            //         if(r != null) path = r.files.single.path!;
                            //     },
                            // ),
                        ],
                    )
                ),
            ),
        );
    }
}