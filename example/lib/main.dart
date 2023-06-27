import 'package:flutter/material.dart';
import 'package:libxmr/libxmr.dart' as libxmr;

void main() {
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  dynamic seed = null; // TODO type/model
  dynamic address = null; // TODO type/model

  static const textStyle = TextStyle(fontSize: 25);
  static const spacerSmall = SizedBox(
    height: 10,
    width: double.infinity,
  );

  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('libxmr example app'),
        ),
        body: SingleChildScrollView(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              spacerSmall,
              spacerSmall,
              TextButton(
                onPressed: () {
                  final seed = libxmr.generate_seed();

                  setState(() {
                    this.seed = seed;
                  });
                },
                child: const Text(
                  "Generate seed",
                  style: textStyle,
                ),
              ),
              spacerSmall,
              spacerSmall,
              Text(
                'seed: $seed',
                style: textStyle,
                textAlign: TextAlign.center,
              ),
              spacerSmall,
              Text(
                'seed variable type: ${seed.runtimeType}',
                style: textStyle,
                textAlign: TextAlign.center,
              ),
              spacerSmall,
              spacerSmall,
              TextButton(
                onPressed: () {
                  final address = libxmr.generate_address(seed);

                  setState(() {
                    this.address = address;
                  });
                },
                child: const Text(
                  "Generate address",
                  style: textStyle,
                ),
              ),
              spacerSmall,
              spacerSmall,
              Text(
                'address: $address',
                style: textStyle,
                textAlign: TextAlign.center,
              ),
              spacerSmall,
              Text(
                'address variable type: ${address.runtimeType}',
                style: textStyle,
                textAlign: TextAlign.center,
              ),
            ],
          ),
        ),
      ),
    );
  }
}
