import 'package:flutter/material.dart';
import 'dart:async';

import 'package:flutter/services.dart';
import 'package:id3tags/id3tags.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  final _id3tagsPlugin = Id3Tags();

  @override
  void initState() {
    super.initState();
  }

    @override
    Widget build(BuildContext context)
    {
        return MaterialApp(
            home: Scaffold(
                appBar: AppBar(
                    title: const Text('Plugin example app'),
                ),
                body: Center(
                    child: ElevatedButton(
                        child: const Text("Write"),
                        onPressed: () async => Id3Tags.write("/home/erikas/Music/test2.mp3", Tag(
                            title: "Test",
                            artist: "testsetestest"
                        )),
                    )
                ),
            ),
        );
    }
}
