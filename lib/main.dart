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
  final Timer stopwatchTimer = Timer();
  bool timerMutex = false;
  // stopwatchTimer
  // final

  // late MyTimer timer;
  // late Stream<int> timer;

  @override
  void initState() {
    super.initState();
    // stopwatchTimer = MyTimer();
    // stopwatchTimer.startTimer();
    // startTimer(declaredTimer: stopwatchTimer);
    // stopwatchTimer.startTimer();
    // timer = MyTimer.newInstance() as MyTimer;

    // timer = Timer.newInstance();
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
            ElevatedButton(
                onPressed: () {
                  // startTimer(timer: stopwatchTimer);
                  // print(stopwatchTimer.returnSomething());
                  // setState(() {
                  //   timerMutex = !timerMutex;
                  // });
                  stopwatchTimer.startTimer();
                  setState(() {
                    timerMutex = !timerMutex;
                  });
                },
                child: Container(
                  color: Colors.amber,
                  child: Text("Start"),
                  // child: Text(stopwatchTimer.name),
                )),
            ElevatedButton(
                onPressed: () {
                  // startTimer(timer: stopwatchTimer);
                  // print(stopwatchTimer.returnSomething());
                  // setState(() {
                  //   timerMutex = !timerMutex;
                  // });
                  setState(() {
                    timerMutex = !timerMutex;
                  });
                  stopwatchTimer.stopTimer();
                },
                child: Container(
                  color: Colors.amber,
                  child: Text("Stop"),
                  // child: Text(stopwatchTimer.name),
                )),
            const Text("Time since starting Rust stream"),
            StreamBuilder<int>(
              stream: regTick(),
              builder: (context, snap) {
                final style = Theme.of(context).textTheme.headlineMedium;
                final error = snap.error;
                if (error != null)
                  return Tooltip(
                      message: error.toString(),
                      child: Text('Error', style: style));

                final data = snap.data;
                if (data != null) return Text('$data second(s)', style: style);

                return const CircularProgressIndicator();
              },
            ),
            Builder(builder: (BuildContext context) {
              // return Text("Even more busy");
              return StreamBuilder<int>(
                stream: timerMutex == false ? null : stopwatchTimer.tick(),
                builder: (context, snap) {
                  final style = Theme.of(context).textTheme.headlineMedium;
                  final error = snap.error;
                  if (error != null)
                    return Tooltip(
                        message: error.toString(),
                        child: Text('Error', style: style));

                  final data = snap.data;
                  if (data != null)
                    return Text('$data second(s)', style: style);

                  return Text('$data second(s)', style: style);
                },
              );
            }),
            // StreamBuilder<int>(
            //   stream: tick(timer: stopwatchTimer),
            //   builder: (context, snap) {
            //     final style = Theme.of(context).textTheme.headlineMedium;
            //     final error = snap.error;
            //     if (error != null)
            //       return Tooltip(
            //           message: error.toString(),
            //           child: Text('Error', style: style));

            //     final data = snap.data;
            //     if (data != null) return Text('$data second(s)', style: style);

            //     return const CircularProgressIndicator();
            //   },
            // )
          ],
        ),
      ),
    );
  }
}