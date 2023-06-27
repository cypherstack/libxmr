import 'dart:ffi';
import 'dart:io';
import 'dart:convert' show utf8;

import 'package:ffi/ffi.dart';

import 'libxmr_bindings_generated.dart';

const String _libName = 'libxmr';

/// The dynamic library in which the symbols for [LibxmrBindings] can be found.
final DynamicLibrary _dylib = () {
  if (Platform.isMacOS || Platform.isIOS) {
    return DynamicLibrary.open('$_libName.framework/$_libName');
  }
  if (Platform.isAndroid || Platform.isLinux) {
    return DynamicLibrary.open('$_libName.so');
  }
  if (Platform.isWindows) {
    return DynamicLibrary.open('$_libName.dll');
  }
  throw UnsupportedError('Unknown platform: ${Platform.operatingSystem}');
}();

/// The bindings to the native functions in [_dylib].
final LibxmrBindings _bindings = LibxmrBindings(_dylib);

String generate_seed() {
  final Pointer<Char> seedPtr = _bindings.generate_seed();
  final utf8Pointer = seedPtr.cast<Utf8>();
  final seed = utf8Pointer.toDartString();

  calloc.free(utf8Pointer);

  return seed;
}

String generate_address(String mnemonic) {
  Pointer<Char> mnemonicPtr = mnemonic.toNativeUtf8().cast<Char>(); // TODO may be incorrect
  final Pointer<Char> addressPtr = _bindings.generate_address(mnemonicPtr);

  final utf8Pointer = addressPtr.cast<Utf8>();
  final seed = utf8Pointer.toDartString();

  // calloc.free(utf8Pointer);

  return seed;
}

Pointer<Char> string_to_char_ptr(String str) {
  // print(1);
  // print(str);
  // // Convert the Dart string to a UTF-8 encoded list of characters
  // List<int> utf8Bytes = utf8.encode(str);
  // print(2);
  // print(utf8Bytes);
  //
  // // Allocate memory for the C string using calloc
  // Pointer<Uint8> charPointer = calloc<Uint8>(utf8Bytes.length + 1);
  // print(3);
  // print(charPointer);
  //
  // // Copy the UTF-8 encoded bytes to the allocated memory
  // charPointer.asTypedList(utf8Bytes.length).setAll(0, utf8Bytes);
  // print(4);
  // print(charPointer);
  //
  // // Null-terminate the string
  // charPointer.elementAt(utf8Bytes.length).value = 0;
  // print(5);
  // print(charPointer);
  //
  // // Cast the Pointer<Uint8> to Pointer<Char>
  // final Pointer<Char> charPointerCast = charPointer.cast<Char>();
  // print(6);
  // print(charPointer);
  // print(charPointer.address);
  // print(charPointer.value);
  // print(7);
  // print(charPointerCast);
  // print(charPointerCast.address);
  // print(charPointerCast.value);
  //
  // // Return the Pointer<Char> representing the string
  // return charPointerCast;

  List<int> utf8Bytes = utf8.encode(str);
  Pointer<Uint8> strPointer = calloc<Uint8>(utf8Bytes.length + 1);
  for (int i = 0; i < utf8Bytes.length; i++) {
    strPointer.elementAt(i).value = utf8Bytes[i];
  }
  // null terminate the string
  strPointer.elementAt(utf8Bytes.length).value = 0;

  // Don't forget to free the allocated memory when you're done
  // calloc.free(strPointer);

  return strPointer.cast<Char>();
}
