import 'package:flutter/material.dart';
import 'package:frb_example/src/rust/api/simple.dart';
import 'package:frb_example/src/rust/frb_generated.dart';

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
        body: Center(
          child: Column(
            children: [
              ElevatedButton(
                onPressed: () {
                  print('NonOpaque - Parent struct is creating');
                  final parent = ParentStruct.newVariant1();
                  print('NonOpaque - Parent struct is displaying');
                  print(parent.toJsonString());
                  print('NonOpaque - Parent struct is displaying second time');
                  print(parent.toJsonString());
                  print('NonOpaque - DONE DONE');
                },
                child: Text('Non Opaque Variant - Error'),
              ),

              ElevatedButton(
                onPressed: () {
                  print('Opaque - Parent struct is creating');
                  final parent = ParentStruct.newVariant2();
                  print('Opaque - Parent struct is displaying');
                  print(parent.toJsonString());
                  print('Opaque - Parent struct is displaying second time');
                  print(parent.toJsonString());
                  print('Opaque - DONE DONE');
                },
                child: Text('Opaque Variant - Error'),
              ),
              SizedBox(height: 100),
              ElevatedButton(
                onPressed: () {
                  print('Parent struct is creating');
                  final parent = ParentStruct.newVariant3();
                  print('Parent struct is displaying');
                  print(parent.toJsonString());
                  print('Parent struct is displaying second time');
                  print(parent.toJsonString());
                  print('DONE DONE');
                },
                child: Text('Without struct variant- No Issue'),
              ),
              ElevatedButton(
                onPressed: () {
                  print('Parent struct is creating');
                  final parent = ParentStruct.newVariant4();
                  print('Parent struct is displaying');
                  print(parent.toJsonString());
                  print('Parent struct is displaying second time');
                  print(parent.toJsonString());
                  print('DONE DONE');
                },
                child: Text('Variant without offsetdatetime - No Issue'),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
