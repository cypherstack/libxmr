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
  // mnemonic: hemlock jubilee eden hacksaw boil superior inroads epoxy exhale orders cavernous second brunt saved richly lower upgrade hitched launching deepest mostly playful layout lower eden
  // seed (hex): 29adefc8f67515b4b4bf48031780ab9d071d24f8a674b879ce7f245c37523807
  // private spend: 29adefc8f67515b4b4bf48031780ab9d071d24f8a674b879ce7f245c37523807
  // private view: 3bc0b202cde92fe5719c3cc0a16aa94f88a5d19f8c515d4e35fae361f6f2120e
  // private view (audit address): 4f02594e84985fd78b91bb25dbb184d673b96b8b7539cc648c9c95a095428400
  // public spend: 72170da1793490ea9d0243df46c515444c35104b92b1d75a7d8c5954ba1f49cd
  // public view: 21243cb8d0046baf10619d1fe7f38708095b006ef8e8350963c160478c1c0ff0
  // address: 45wsWad9EwZgF3VpxQumrUCRaEtdyyh6NG8sVD3YRVVJbK1jkpJ3zq8WHLijVzodQ22LxwkdWx7fS2a6JzaRGzkNU8K2Dhi
  // TODO type/model all of these below ⬇️
  String mnemonic = "hemlock jubilee eden hacksaw boil superior inroads epoxy exhale orders cavernous second brunt saved richly lower upgrade hitched launching deepest mostly playful layout lower eden";
  String address = "";
  String subaddress = "";

  static const textStyle = TextStyle(fontSize: 25);
  static const spacerSmall = SizedBox(
    height: 10,
    width: double.infinity,
  );

  @override
  void initState() {
    this.address = libxmr.generateAddress(mnemonic: mnemonic);
    this.subaddress = libxmr.generateAddress(mnemonic: mnemonic, account: 0, index: 1);
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
                  final mnemonic = libxmr.generateMnemonic();
                  print("generated mnemonic: $mnemonic");

                  setState(() {
                    this.mnemonic = mnemonic;
                  });
                },
                child: const Text(
                  "Generate mnemonic",
                  style: textStyle,
                ),
              ),
              spacerSmall,
              spacerSmall,
              Text(
                'mnemonic: $mnemonic',
                style: textStyle,
                textAlign: TextAlign.center,
              ),
              spacerSmall,
              spacerSmall,
              TextButton(
                onPressed: () {
                  final address = libxmr.generateAddress(mnemonic: this.mnemonic);

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
            spacerSmall,
            TextButton(
              onPressed: () {
                final subaddress = libxmr.generateAddress(mnemonic: this.mnemonic, account: 0, index: 1);

                setState(() {
                  this.subaddress = subaddress;
                });
              },
              child: const Text(
                "Generate subaddress",
                style: textStyle,
              ),
            ),
            spacerSmall,
            spacerSmall,
            Text(
              'subaddress: $subaddress',
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
