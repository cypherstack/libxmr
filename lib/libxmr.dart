import 'dart:ffi';
import 'dart:io';

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
