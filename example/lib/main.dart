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
  // test vector from https://xmrtests.llcoins.net/addresstests.html
  // seed (mnemonic): hemlock jubilee eden hacksaw boil superior inroads epoxy exhale orders cavernous second brunt saved richly lower upgrade hitched launching deepest mostly playful layout lower eden
  // seed (hex): 29adefc8f67515b4b4bf48031780ab9d071d24f8a674b879ce7f245c37523807
  // private spend: 29adefc8f67515b4b4bf48031780ab9d071d24f8a674b879ce7f245c37523807
  // private view: 3bc0b202cde92fe5719c3cc0a16aa94f88a5d19f8c515d4e35fae361f6f2120e
  // private view (audit address): 4f02594e84985fd78b91bb25dbb184d673b96b8b7539cc648c9c95a095428400
  // public spend: 72170da1793490ea9d0243df46c515444c35104b92b1d75a7d8c5954ba1f49cd
  // public view: 21243cb8d0046baf10619d1fe7f38708095b006ef8e8350963c160478c1c0ff0
  // address: 45wsWad9EwZgF3VpxQumrUCRaEtdyyh6NG8sVD3YRVVJbK1jkpJ3zq8WHLijVzodQ22LxwkdWx7fS2a6JzaRGzkNU8K2Dhi
  // TODO type/model all of these below ⬇️
  dynamic seed = "hemlock jubilee eden hacksaw boil superior inroads epoxy exhale orders cavernous second brunt saved richly lower upgrade hitched launching deepest mostly playful layout lower eden";
  dynamic address = null;

  static const textStyle = TextStyle(fontSize: 25);
  static const spacerSmall = SizedBox(
    height: 10,
    width: double.infinity,
  );

  @override
  void initState() {
    this.address = libxmr.generate_address(this.seed);
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
                  print("generated seed: $seed");

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
