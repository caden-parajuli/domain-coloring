import 'dart:async';
import 'dart:typed_data';
import 'dart:ui' as ui;

// Render bounding boxes
// import 'package:flutter/rendering.dart';

import 'package:flutter/material.dart';

import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Domain Coloring',
      theme: ThemeData.dark(useMaterial3: true),
      home: const MyHomePage(title: 'Domain Coloring'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();
  final int imageWidth = 1000;
  final int imageHeight = 1000;
  bool makePlot = false;
  bool plotted = false;
  late Uint8List bmp;
  final functionController = TextEditingController();

  @override
  void initState() {
    super.initState();
  }

  @override
  void dispose() {
    // Clean up the controller when the widget is disposed.
    functionController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    // This method is rerun every time setState is called.

    // Display bounding boxes
    // debugPaintSizeEnabled = true;

    // Plot if needed, otherwise show a placeholder
    Widget plot;

    print("makePlot: $makePlot");

    if (makePlot) {
      if (!plotted) {
        print("plotting...");
        plot = FutureBuilder(
            future: api.colorBmp(
              width: imageWidth,
              height: imageHeight,
              funStr: functionController.text,
              options: const DCOptions(xmin: -5, xmax: 5, ymin: -5, ymax: 5),
            ),
            builder: (BuildContext context, AsyncSnapshot<Uint8List> snapshot) {
              if (snapshot.hasData) {
                plotted = true;
                bmp = snapshot.data as Uint8List;
                return Image.memory(
                  bmp,
                  fit: BoxFit.contain,
                  alignment: Alignment.topLeft,
                );
              } else if (snapshot.hasError) {
                return const Text("Error!");
              } else {
                return const Text("Plotting...");
              }
            });
      } else {
        plot = Image.memory(
          bmp,
          fit: BoxFit.contain,
          alignment: Alignment.topLeft,
        );
      }
    } else {
      plot = const Placeholder(
        fallbackWidth: 100,
        fallbackHeight: 100,
      );
    }

    return Scaffold(
      appBar: AppBar(
        // Here we take the value from the MyHomePage object that was created by
        // the App.build method, and use it to set our appbar title.
        title: Text(widget.title),
      ),
      body: Center(
        child: Row(
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            AspectRatio(aspectRatio: 1.0, child: plot),
            Flexible(
              fit: FlexFit.loose,
              flex: 1,
              child: Form(
                key: _formKey,
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.start,
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Padding(
                      padding: const EdgeInsets.symmetric(vertical: 5.0),
                      child: TextFormField(
                        controller: functionController,
                        decoration: const InputDecoration(
                          border: OutlineInputBorder(),
                          labelText: 'f(z)',
                          hintText: 'Enter a function of z',
                        ),
                        validator: (value) {
                          if (value == null || value.isEmpty) {
                            return 'Please enter some text';
                          }
                          return null;
                        },
                      ),
                    ),
                    FilledButton(
                        onPressed: () {
                          // Validate will return true if the form is valid, or false if
                          // the form is invalid.
                          if (_formKey.currentState!.validate()) {
                            setState(() {
                              makePlot = true;
                              plotted = false;
                            });
                          }
                        },
                        child: const Text("Plot")),
                    const Spacer(flex: 6),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
