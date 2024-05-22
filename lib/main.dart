import 'package:flutter/material.dart';
import 'package:stopwatch/src/rust/api/stopwatch.dart';
import 'package:stopwatch/src/rust/frb_generated.dart';
// import 'ffi.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);
  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  final StopwatchRemote remote = StopwatchRemote();
  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text("Time since starting Rust stream"),
            Builder(builder: (BuildContext context) {
              return StreamBuilder<int>(
                stream: remote.tick(),
                builder: (context, snap) {
                  final data = snap.data;
                  final style = Theme.of(context).textTheme.headlineMedium;
                  final error = snap.error;

                  Duration dur = data != null
                      ? Duration(milliseconds: data)
                      : Duration(hours: 0);

                  int hours = dur.inHours;
                  int minutes = dur.inMinutes.remainder(60);
                  int seconds = dur.inSeconds.remainder(60);
                  int millis = dur.inMilliseconds.remainder(1000);

                  String formattedTime = "${hours.toString().padLeft(2, '0')}:"
                      "${minutes.toString().padLeft(2, '0')}:"
                      "${seconds.toString().padLeft(2, '0')}.";
                  if (millis < 100) {
                    formattedTime += "0";
                  }
                  if (millis < 10) {
                    formattedTime += "0";
                  }
                  formattedTime += millis.toString();

                  if (error != null)
                    return Tooltip(
                        message: error.toString(),
                        child: Text('Error', style: style));

                  if (data != null) return Text(formattedTime, style: style);

                  return Text('$data second(s)', style: style);
                },
              );
            }),
            SizedBox(
              height: 25,
            ),
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                ElevatedButton(
                    onPressed: () {
                      remote.startTimer();
                    },
                    child: Container(
                      child: Text("Start"),
                    )),
                SizedBox(
                  width: 25,
                ),
                ElevatedButton(
                    onPressed: () {
                      remote.stopTimer();
                    },
                    child: Container(
                      child: Text("Stop"),
                    )),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
