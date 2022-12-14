// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.57.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member

import "bridge_definitions.dart";
import 'dart:convert';
import 'dart:async';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'package:meta/meta.dart';
import 'dart:convert';
import 'dart:async';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:meta/meta.dart';
import 'dart:ffi' as ffi;

class AudiotagsImpl implements Audiotags {
  final AudiotagsPlatform _platform;
  factory AudiotagsImpl(ExternalLibrary dylib) =>
      AudiotagsImpl.raw(AudiotagsPlatform(dylib));

  /// Only valid on web/WASM platforms.
  factory AudiotagsImpl.wasm(FutureOr<WasmModule> module) =>
      AudiotagsImpl(module as ExternalLibrary);
  AudiotagsImpl.raw(this._platform);
  Future<Tag> read({required String path, dynamic hint}) {
    var arg0 = _platform.api2wire_String(path);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_read(port_, arg0),
      parseSuccessData: _wire2api_tag,
      constMeta: kReadConstMeta,
      argValues: [path],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kReadConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "read",
        argNames: ["path"],
      );

  Future<void> write({required String path, required Tag data, dynamic hint}) {
    var arg0 = _platform.api2wire_String(path);
    var arg1 = _platform.api2wire_box_autoadd_tag(data);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_write(port_, arg0, arg1),
      parseSuccessData: _wire2api_unit,
      constMeta: kWriteConstMeta,
      argValues: [path, data],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kWriteConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "write",
        argNames: ["path", "data"],
      );

  void dispose() {
    _platform.dispose();
  }
// Section: wire2api

  String _wire2api_String(dynamic raw) {
    return raw as String;
  }

  double _wire2api_box_autoadd_f64(dynamic raw) {
    return raw as double;
  }

  int _wire2api_box_autoadd_i32(dynamic raw) {
    return raw as int;
  }

  double _wire2api_f64(dynamic raw) {
    return raw as double;
  }

  int _wire2api_i32(dynamic raw) {
    return raw as int;
  }

  String? _wire2api_opt_String(dynamic raw) {
    return raw == null ? null : _wire2api_String(raw);
  }

  double? _wire2api_opt_box_autoadd_f64(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_f64(raw);
  }

  int? _wire2api_opt_box_autoadd_i32(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_i32(raw);
  }

  Uint8List? _wire2api_opt_uint_8_list(dynamic raw) {
    return raw == null ? null : _wire2api_uint_8_list(raw);
  }

  Tag _wire2api_tag(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 7)
      throw Exception('unexpected arr length: expect 7 but see ${arr.length}');
    return Tag(
      title: _wire2api_opt_String(arr[0]),
      artist: _wire2api_opt_String(arr[1]),
      album: _wire2api_opt_String(arr[2]),
      year: _wire2api_opt_box_autoadd_i32(arr[3]),
      genre: _wire2api_opt_String(arr[4]),
      duration: _wire2api_opt_box_autoadd_f64(arr[5]),
      picture: _wire2api_opt_uint_8_list(arr[6]),
    );
  }

  int _wire2api_u8(dynamic raw) {
    return raw as int;
  }

  Uint8List _wire2api_uint_8_list(dynamic raw) {
    return raw as Uint8List;
  }

  void _wire2api_unit(dynamic raw) {
    return;
  }
}

// Section: api2wire

@protected
double api2wire_f64(double raw) {
  return raw;
}

@protected
int api2wire_i32(int raw) {
  return raw;
}

@protected
int api2wire_u8(int raw) {
  return raw;
}

// Section: finalizer

class AudiotagsPlatform extends FlutterRustBridgeBase<AudiotagsWire> {
  AudiotagsPlatform(ffi.DynamicLibrary dylib) : super(AudiotagsWire(dylib));

// Section: api2wire

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_String(String raw) {
    return api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<ffi.Double> api2wire_box_autoadd_f64(double raw) {
    return inner.new_box_autoadd_f64_0(api2wire_f64(raw));
  }

  @protected
  ffi.Pointer<ffi.Int32> api2wire_box_autoadd_i32(int raw) {
    return inner.new_box_autoadd_i32_0(api2wire_i32(raw));
  }

  @protected
  ffi.Pointer<wire_Tag> api2wire_box_autoadd_tag(Tag raw) {
    final ptr = inner.new_box_autoadd_tag_0();
    _api_fill_to_wire_tag(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_opt_String(String? raw) {
    return raw == null ? ffi.nullptr : api2wire_String(raw);
  }

  @protected
  ffi.Pointer<ffi.Double> api2wire_opt_box_autoadd_f64(double? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_f64(raw);
  }

  @protected
  ffi.Pointer<ffi.Int32> api2wire_opt_box_autoadd_i32(int? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_i32(raw);
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_opt_uint_8_list(Uint8List? raw) {
    return raw == null ? ffi.nullptr : api2wire_uint_8_list(raw);
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list_0(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }
// Section: finalizer

// Section: api_fill_to_wire

  void _api_fill_to_wire_box_autoadd_tag(
      Tag apiObj, ffi.Pointer<wire_Tag> wireObj) {
    _api_fill_to_wire_tag(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_tag(Tag apiObj, wire_Tag wireObj) {
    wireObj.title = api2wire_opt_String(apiObj.title);
    wireObj.artist = api2wire_opt_String(apiObj.artist);
    wireObj.album = api2wire_opt_String(apiObj.album);
    wireObj.year = api2wire_opt_box_autoadd_i32(apiObj.year);
    wireObj.genre = api2wire_opt_String(apiObj.genre);
    wireObj.duration = api2wire_opt_box_autoadd_f64(apiObj.duration);
    wireObj.picture = api2wire_opt_uint_8_list(apiObj.picture);
  }
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.

/// generated by flutter_rust_bridge
class AudiotagsWire implements FlutterRustBridgeWireBase {
  @internal
  late final dartApi = DartApiDl(init_frb_dart_api_dl);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  AudiotagsWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  AudiotagsWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr
      .asFunction<void Function(DartPostCObjectFnType)>();

  Object get_dart_object(
    int ptr,
  ) {
    return _get_dart_object(
      ptr,
    );
  }

  late final _get_dart_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Handle Function(uintptr_t)>>(
          'get_dart_object');
  late final _get_dart_object =
      _get_dart_objectPtr.asFunction<Object Function(int)>();

  void drop_dart_object(
    int ptr,
  ) {
    return _drop_dart_object(
      ptr,
    );
  }

  late final _drop_dart_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(uintptr_t)>>(
          'drop_dart_object');
  late final _drop_dart_object =
      _drop_dart_objectPtr.asFunction<void Function(int)>();

  int new_dart_opaque(
    Object handle,
  ) {
    return _new_dart_opaque(
      handle,
    );
  }

  late final _new_dart_opaquePtr =
      _lookup<ffi.NativeFunction<uintptr_t Function(ffi.Handle)>>(
          'new_dart_opaque');
  late final _new_dart_opaque =
      _new_dart_opaquePtr.asFunction<int Function(Object)>();

  int init_frb_dart_api_dl(
    ffi.Pointer<ffi.Void> obj,
  ) {
    return _init_frb_dart_api_dl(
      obj,
    );
  }

  late final _init_frb_dart_api_dlPtr =
      _lookup<ffi.NativeFunction<ffi.IntPtr Function(ffi.Pointer<ffi.Void>)>>(
          'init_frb_dart_api_dl');
  late final _init_frb_dart_api_dl = _init_frb_dart_api_dlPtr
      .asFunction<int Function(ffi.Pointer<ffi.Void>)>();

  void wire_read(
    int port_,
    ffi.Pointer<wire_uint_8_list> path,
  ) {
    return _wire_read(
      port_,
      path,
    );
  }

  late final _wire_readPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>('wire_read');
  late final _wire_read = _wire_readPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_write(
    int port_,
    ffi.Pointer<wire_uint_8_list> path,
    ffi.Pointer<wire_Tag> data,
  ) {
    return _wire_write(
      port_,
      path,
      data,
    );
  }

  late final _wire_writePtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>,
              ffi.Pointer<wire_Tag>)>>('wire_write');
  late final _wire_write = _wire_writePtr.asFunction<
      void Function(
          int, ffi.Pointer<wire_uint_8_list>, ffi.Pointer<wire_Tag>)>();

  ffi.Pointer<ffi.Double> new_box_autoadd_f64_0(
    double value,
  ) {
    return _new_box_autoadd_f64_0(
      value,
    );
  }

  late final _new_box_autoadd_f64_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Double> Function(ffi.Double)>>(
          'new_box_autoadd_f64_0');
  late final _new_box_autoadd_f64_0 = _new_box_autoadd_f64_0Ptr
      .asFunction<ffi.Pointer<ffi.Double> Function(double)>();

  ffi.Pointer<ffi.Int32> new_box_autoadd_i32_0(
    int value,
  ) {
    return _new_box_autoadd_i32_0(
      value,
    );
  }

  late final _new_box_autoadd_i32_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Int32> Function(ffi.Int32)>>(
          'new_box_autoadd_i32_0');
  late final _new_box_autoadd_i32_0 = _new_box_autoadd_i32_0Ptr
      .asFunction<ffi.Pointer<ffi.Int32> Function(int)>();

  ffi.Pointer<wire_Tag> new_box_autoadd_tag_0() {
    return _new_box_autoadd_tag_0();
  }

  late final _new_box_autoadd_tag_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_Tag> Function()>>(
          'new_box_autoadd_tag_0');
  late final _new_box_autoadd_tag_0 =
      _new_box_autoadd_tag_0Ptr.asFunction<ffi.Pointer<wire_Tag> Function()>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list_0(
    int len,
  ) {
    return _new_uint_8_list_0(
      len,
    );
  }

  late final _new_uint_8_list_0Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_uint_8_list> Function(
              ffi.Int32)>>('new_uint_8_list_0');
  late final _new_uint_8_list_0 = _new_uint_8_list_0Ptr
      .asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void free_WireSyncReturn(
    WireSyncReturn ptr,
  ) {
    return _free_WireSyncReturn(
      ptr,
    );
  }

  late final _free_WireSyncReturnPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturn)>>(
          'free_WireSyncReturn');
  late final _free_WireSyncReturn =
      _free_WireSyncReturnPtr.asFunction<void Function(WireSyncReturn)>();
}

class _Dart_Handle extends ffi.Opaque {}

class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

class wire_Tag extends ffi.Struct {
  external ffi.Pointer<wire_uint_8_list> title;

  external ffi.Pointer<wire_uint_8_list> artist;

  external ffi.Pointer<wire_uint_8_list> album;

  external ffi.Pointer<ffi.Int32> year;

  external ffi.Pointer<wire_uint_8_list> genre;

  external ffi.Pointer<ffi.Double> duration;

  external ffi.Pointer<wire_uint_8_list> picture;
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<ffi.Bool Function(DartPort, ffi.Pointer<ffi.Void>)>>;
typedef DartPort = ffi.Int64;
typedef uintptr_t = ffi.UnsignedLong;
