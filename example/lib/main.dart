import 'package:flutter/material.dart';
import 'package:bixat_screen_recorder/bixat_screen_recorder.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: StreamBuilder<String?>(
          stream: startScreenCapture(),
          builder: (context, snapshot) {
            return Center(
              child: Text(
                  'Action: ${snapshot.data} Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'),
            );
          }
        ),
      ),
    );
  }
}
