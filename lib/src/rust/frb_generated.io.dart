// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.20.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables, unused_field

import 'api/games.dart';
import 'dart:async';
import 'dart:convert';
import 'dart:ffi' as ffi;
import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';
import 'package:uuid/uuid.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_FlutterWatchPtr => wire
          ._rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatchPtr;

  @protected
  AnyhowException dco_decode_AnyhowException(dynamic raw);

  @protected
  FlutterWatch
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch(
          dynamic raw);

  @protected
  FlutterWatch
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch(
          dynamic raw);

  @protected
  FlutterWatch
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch(
          dynamic raw);

  @protected
  String dco_decode_String(dynamic raw);

  @protected
  UuidValue dco_decode_Uuid(dynamic raw);

  @protected
  Game dco_decode_box_autoadd_game(dynamic raw);

  @protected
  Game dco_decode_game(dynamic raw);

  @protected
  Games dco_decode_games(dynamic raw);

  @protected
  List<Game> dco_decode_list_game(dynamic raw);

  @protected
  List<int> dco_decode_list_prim_u_8_loose(dynamic raw);

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw);

  @protected
  (int, int) dco_decode_record_u_64_u_64(dynamic raw);

  @protected
  int dco_decode_u_64(dynamic raw);

  @protected
  int dco_decode_u_8(dynamic raw);

  @protected
  void dco_decode_unit(dynamic raw);

  @protected
  int dco_decode_usize(dynamic raw);

  @protected
  AnyhowException sse_decode_AnyhowException(SseDeserializer deserializer);

  @protected
  FlutterWatch
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch(
          SseDeserializer deserializer);

  @protected
  FlutterWatch
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch(
          SseDeserializer deserializer);

  @protected
  FlutterWatch
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch(
          SseDeserializer deserializer);

  @protected
  String sse_decode_String(SseDeserializer deserializer);

  @protected
  UuidValue sse_decode_Uuid(SseDeserializer deserializer);

  @protected
  Game sse_decode_box_autoadd_game(SseDeserializer deserializer);

  @protected
  Game sse_decode_game(SseDeserializer deserializer);

  @protected
  Games sse_decode_games(SseDeserializer deserializer);

  @protected
  List<Game> sse_decode_list_game(SseDeserializer deserializer);

  @protected
  List<int> sse_decode_list_prim_u_8_loose(SseDeserializer deserializer);

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  (int, int) sse_decode_record_u_64_u_64(SseDeserializer deserializer);

  @protected
  int sse_decode_u_64(SseDeserializer deserializer);

  @protected
  int sse_decode_u_8(SseDeserializer deserializer);

  @protected
  void sse_decode_unit(SseDeserializer deserializer);

  @protected
  int sse_decode_usize(SseDeserializer deserializer);

  @protected
  int sse_decode_i_32(SseDeserializer deserializer);

  @protected
  bool sse_decode_bool(SseDeserializer deserializer);

  @protected
  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_encode_AnyhowException(
      AnyhowException raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    throw UnimplementedError();
  }

  @protected
  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_encode_String(String raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return cst_encode_list_prim_u_8_strict(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_encode_Uuid(UuidValue raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return cst_encode_list_prim_u_8_strict(raw.toBytes());
  }

  @protected
  ffi.Pointer<wire_cst_game> cst_encode_box_autoadd_game(Game raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ptr = wire.cst_new_box_autoadd_game();
    cst_api_fill_to_wire_game(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_cst_list_game> cst_encode_list_game(List<Game> raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ans = wire.cst_new_list_game(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      cst_api_fill_to_wire_game(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_cst_list_prim_u_8_loose> cst_encode_list_prim_u_8_loose(
      List<int> raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ans = wire.cst_new_list_prim_u_8_loose(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  @protected
  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_encode_list_prim_u_8_strict(
      Uint8List raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    final ans = wire.cst_new_list_prim_u_8_strict(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  @protected
  int cst_encode_u_64(int raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw.toInt();
  }

  @protected
  void cst_api_fill_to_wire_box_autoadd_game(
      Game apiObj, ffi.Pointer<wire_cst_game> wireObj) {
    cst_api_fill_to_wire_game(apiObj, wireObj.ref);
  }

  @protected
  void cst_api_fill_to_wire_game(Game apiObj, wire_cst_game wireObj) {
    wireObj.name = cst_encode_String(apiObj.name);
    wireObj.exe = cst_encode_String(apiObj.exe);
    wireObj.icon = cst_encode_String(apiObj.icon);
    wireObj.url = cst_encode_String(apiObj.url);
    wireObj.uuid = cst_encode_Uuid(apiObj.uuid);
  }

  @protected
  void cst_api_fill_to_wire_games(Games apiObj, wire_cst_games wireObj) {
    wireObj.games = cst_encode_list_game(apiObj.games);
  }

  @protected
  void cst_api_fill_to_wire_record_u_64_u_64(
      (int, int) apiObj, wire_cst_record_u_64_u_64 wireObj) {
    wireObj.field0 = cst_encode_u_64(apiObj.$1);
    wireObj.field1 = cst_encode_u_64(apiObj.$2);
  }

  @protected
  int cst_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch(
      FlutterWatch raw);

  @protected
  int cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch(
      FlutterWatch raw);

  @protected
  int cst_encode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch(
      FlutterWatch raw);

  @protected
  int cst_encode_u_8(int raw);

  @protected
  void cst_encode_unit(void raw);

  @protected
  int cst_encode_usize(int raw);

  @protected
  void sse_encode_AnyhowException(
      AnyhowException self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch(
          FlutterWatch self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch(
          FlutterWatch self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch(
          FlutterWatch self, SseSerializer serializer);

  @protected
  void sse_encode_String(String self, SseSerializer serializer);

  @protected
  void sse_encode_Uuid(UuidValue self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_game(Game self, SseSerializer serializer);

  @protected
  void sse_encode_game(Game self, SseSerializer serializer);

  @protected
  void sse_encode_games(Games self, SseSerializer serializer);

  @protected
  void sse_encode_list_game(List<Game> self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_8_loose(List<int> self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_8_strict(
      Uint8List self, SseSerializer serializer);

  @protected
  void sse_encode_record_u_64_u_64((int, int) self, SseSerializer serializer);

  @protected
  void sse_encode_u_64(int self, SseSerializer serializer);

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer);

  @protected
  void sse_encode_unit(void self, SseSerializer serializer);

  @protected
  void sse_encode_usize(int self, SseSerializer serializer);

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer);
}

// Section: wire_class

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names
// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.
// ignore_for_file: type=lint

/// generated by flutter_rust_bridge
class RustLibWire implements BaseWire {
  factory RustLibWire.fromExternalLibrary(ExternalLibrary lib) =>
      RustLibWire(lib.ffiDynamicLibrary);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  RustLibWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  RustLibWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void wire_extract_zip(
    int port_,
    ffi.Pointer<wire_cst_list_prim_u_8_loose> bytes,
    ffi.Pointer<wire_cst_game> game,
  ) {
    return _wire_extract_zip(
      port_,
      bytes,
      game,
    );
  }

  late final _wire_extract_zipPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(
                  ffi.Int64,
                  ffi.Pointer<wire_cst_list_prim_u_8_loose>,
                  ffi.Pointer<wire_cst_game>)>>(
      'frbgen_bramletts_games_wire_extract_zip');
  late final _wire_extract_zip = _wire_extract_zipPtr.asFunction<
      void Function(int, ffi.Pointer<wire_cst_list_prim_u_8_loose>,
          ffi.Pointer<wire_cst_game>)>();

  void wire_fetch_games(
    int port_,
  ) {
    return _wire_fetch_games(
      port_,
    );
  }

  late final _wire_fetch_gamesPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'frbgen_bramletts_games_wire_fetch_games');
  late final _wire_fetch_games =
      _wire_fetch_gamesPtr.asFunction<void Function(int)>();

  WireSyncRust2DartDco wire_get_watcher(
    int obj,
  ) {
    return _wire_get_watcher(
      obj,
    );
  }

  late final _wire_get_watcherPtr =
      _lookup<ffi.NativeFunction<WireSyncRust2DartDco Function(ffi.UintPtr)>>(
          'frbgen_bramletts_games_wire_get_watcher');
  late final _wire_get_watcher =
      _wire_get_watcherPtr.asFunction<WireSyncRust2DartDco Function(int)>();

  void wire_init_app(
    int port_,
  ) {
    return _wire_init_app(
      port_,
    );
  }

  late final _wire_init_appPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'frbgen_bramletts_games_wire_init_app');
  late final _wire_init_app =
      _wire_init_appPtr.asFunction<void Function(int)>();

  void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch(
      ptr,
    );
  }

  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatchPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_bramletts_games_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch');
  late final _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch =
      _rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatchPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch(
    ffi.Pointer<ffi.Void> ptr,
  ) {
    return _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch(
      ptr,
    );
  }

  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatchPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(
          'frbgen_bramletts_games_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch');
  late final _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatch =
      _rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockcrateapigamesFlutterWatchPtr
          .asFunction<void Function(ffi.Pointer<ffi.Void>)>();

  ffi.Pointer<wire_cst_game> cst_new_box_autoadd_game() {
    return _cst_new_box_autoadd_game();
  }

  late final _cst_new_box_autoadd_gamePtr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_cst_game> Function()>>(
          'frbgen_bramletts_games_cst_new_box_autoadd_game');
  late final _cst_new_box_autoadd_game = _cst_new_box_autoadd_gamePtr
      .asFunction<ffi.Pointer<wire_cst_game> Function()>();

  ffi.Pointer<wire_cst_list_game> cst_new_list_game(
    int len,
  ) {
    return _cst_new_list_game(
      len,
    );
  }

  late final _cst_new_list_gamePtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_cst_list_game> Function(
              ffi.Int32)>>('frbgen_bramletts_games_cst_new_list_game');
  late final _cst_new_list_game = _cst_new_list_gamePtr
      .asFunction<ffi.Pointer<wire_cst_list_game> Function(int)>();

  ffi.Pointer<wire_cst_list_prim_u_8_loose> cst_new_list_prim_u_8_loose(
    int len,
  ) {
    return _cst_new_list_prim_u_8_loose(
      len,
    );
  }

  late final _cst_new_list_prim_u_8_loosePtr = _lookup<
          ffi.NativeFunction<
              ffi.Pointer<wire_cst_list_prim_u_8_loose> Function(ffi.Int32)>>(
      'frbgen_bramletts_games_cst_new_list_prim_u_8_loose');
  late final _cst_new_list_prim_u_8_loose = _cst_new_list_prim_u_8_loosePtr
      .asFunction<ffi.Pointer<wire_cst_list_prim_u_8_loose> Function(int)>();

  ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_new_list_prim_u_8_strict(
    int len,
  ) {
    return _cst_new_list_prim_u_8_strict(
      len,
    );
  }

  late final _cst_new_list_prim_u_8_strictPtr = _lookup<
          ffi.NativeFunction<
              ffi.Pointer<wire_cst_list_prim_u_8_strict> Function(ffi.Int32)>>(
      'frbgen_bramletts_games_cst_new_list_prim_u_8_strict');
  late final _cst_new_list_prim_u_8_strict = _cst_new_list_prim_u_8_strictPtr
      .asFunction<ffi.Pointer<wire_cst_list_prim_u_8_strict> Function(int)>();

  int dummy_method_to_enforce_bundling() {
    return _dummy_method_to_enforce_bundling();
  }

  late final _dummy_method_to_enforce_bundlingPtr =
      _lookup<ffi.NativeFunction<ffi.Int64 Function()>>(
          'dummy_method_to_enforce_bundling');
  late final _dummy_method_to_enforce_bundling =
      _dummy_method_to_enforce_bundlingPtr.asFunction<int Function()>();
}

final class wire_cst_list_prim_u_8_loose extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_cst_list_prim_u_8_strict extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_cst_game extends ffi.Struct {
  external ffi.Pointer<wire_cst_list_prim_u_8_strict> name;

  external ffi.Pointer<wire_cst_list_prim_u_8_strict> exe;

  external ffi.Pointer<wire_cst_list_prim_u_8_strict> icon;

  external ffi.Pointer<wire_cst_list_prim_u_8_strict> url;

  external ffi.Pointer<wire_cst_list_prim_u_8_strict> uuid;
}

final class wire_cst_list_game extends ffi.Struct {
  external ffi.Pointer<wire_cst_game> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_cst_games extends ffi.Struct {
  external ffi.Pointer<wire_cst_list_game> games;
}

final class wire_cst_record_u_64_u_64 extends ffi.Struct {
  @ffi.Uint64()
  external int field0;

  @ffi.Uint64()
  external int field1;
}
