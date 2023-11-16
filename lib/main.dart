import 'dart:async';
import 'dart:typed_data';
import 'dart:ui' as ui;

import 'package:flutter/material.dart';

import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Domain Coloring',
      theme: ThemeData(
        // This is the theme of your application.
        //
        // Try running your application with "flutter run". You'll see the
        // application has a blue toolbar. Then, without quitting the app, try
        // changing the primarySwatch below to Colors.green and then invoke
        // "hot reload" (press "r" in the console where you ran "flutter run",
        // or simply save your changes to "hot reload" in a Flutter IDE).
        // Notice that the counter didn't reset back to zero; the application
        // is not restarted.
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(title: 'Domain Coloring'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

extension<L, R> on (FutureOr<L>, FutureOr<R>) {
  // A convenience method enabled by Dart 3, which will be useful later.
  Future<(L, R)> join() async {
    final fut =
        await Future.wait([Future.value(this.$1), Future.value(this.$2)]);
    return (fut[0] as L, fut[1] as R);
  }
}

class _MyHomePageState extends State<MyHomePage> {
  // These futures belong to the state and are only initialized once,
  // in the initState method.

  // late Future<Platform> platform;
  // late Future<bool> isRelease;

  final int imageWidth = 500;
  final int imageHeight = 500;
  final bool makePlot = true;
  bool plotted = false;

  // late final Uint32List pixels;
  late final Uint8List bmp;

  @override
  void initState() {
    super.initState();
    // pixels = Uint32List(imageWidth * imageHeight);
    // bmp = Uint8List(3 * imageWidth * imageHeight);
    // platform = api.platform();
    // isRelease = api.rustReleaseMode();
  }

  @override
  Widget build(BuildContext context) {
    // This method is rerun every time setState is called.
    //
    // The Flutter framework has been optimized to make rerunning build methods
    // fast, so that you can just rebuild anything that needs updating rather
    // than having to individually change instances of widgets.

    // Plot if needed, otherwise place a placeholder
    Widget plot;

    if (makePlot) {
      if (!plotted) {
        plot = FutureBuilder(
            future: api.colorBmp(
                width: imageWidth,
                height: imageHeight,
                funStr: "z",
                options: const DCOptions(xmin: -5, xmax: 5, ymin: -5, ymax: 5)),
            builder: (BuildContext context, AsyncSnapshot<Uint8List> snapshot) {
              if (snapshot.hasData) {
                plotted = true;
                bmp = snapshot.data as Uint8List;
                return Image.memory(bmp);
              } else if (snapshot.hasError) {
                return const Text("Error!");
              } else {
                return const Text("Plotting...");
              }
            });
      } else {
        plot = Image.memory(bmp);
      }
      // ui.decodeImageFromPixels(pixels.buffer.asUint8List(), imageWidth,
      //     imageHeight, ui.PixelFormat.rgba8888, (ui.Image image) {
      //   plot = RawImage(
      //     image: image,
      //   );
      // });
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
          children: [
            Column(children: [plot]),
            const Column(),
          ],
        ),
        //     FutureBuilder(
        //       // We await for both futures in a tuple, then uwnrap their results inside the builder.
        //       // Recent versions of Dart allow the type of the snapshot to be correctly inferred.
        //       // Since Future.wait predates Dart 3 and does not understand tuples, we use the join method
        //       // declared earlier to concurrently await two futures while preserving type safety.
        //       future: (platform, isRelease).join(),
        //       builder: (context, snap) {
        //         final style = Theme.of(context).textTheme.headlineMedium;
        //         if (snap.error != null) {
        //           // An error has been encountered, so give an appropriate response and
        //           // pass the error details to an unobstructive tooltip.
        //           debugPrint(snap.error.toString());
        //           return Tooltip(
        //             message: snap.error.toString(),
        //             child: Text('Unknown OS', style: style),
        //           );
        //         }

        //         // Guard return here, the data is not ready yet.
        //         final data = snap.data;
        //         if (data == null) return const CircularProgressIndicator();

        //         final (platform, release) = data;
        //         final releaseText = release ? 'Release' : 'Debug';

        //         // Another feature introduced in Dart 3 is switch expressions,
        //         // allowing exhaustive matching over enums or sealed classes
        //         // similar to Rust's match expressions. Note that all possible values
        //         // of Platform are present here; should additional values be added,
        //         // this expression would not compile.
        //         final text = switch (platform) {
        //           Platform.Android => 'Android',
        //           Platform.Ios => 'iOS',
        //           Platform.MacApple => 'MacOS with Apple Silicon',
        //           Platform.MacIntel => 'MacOS',
        //           Platform.Windows => 'Windows',
        //           Platform.Unix => 'Unix',
        //           Platform.Wasm => 'the Web',
        //           Platform.Unknown => 'Unknown OS',
        //         };
        //         return Text('$text ($releaseText)', style: style);
        //       },
        //     )
        //   ],
        // ),
      ),
    );
  }
}
