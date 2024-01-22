// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.21.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables, unused_field

import 'api/games.dart';
import 'dart:async';
import 'dart:convert';
import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_ProgressPtr => wire
      .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress;

  @protected
  AnyhowException dco_decode_AnyhowException(dynamic raw);

  @protected
  Progress
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
          dynamic raw);

  @protected
  Progress
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
          dynamic raw);

  @protected
  Progress
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
          dynamic raw);

  @protected
  String dco_decode_String(dynamic raw);

  @protected
  bool dco_decode_bool(dynamic raw);

  @protected
  Game dco_decode_box_autoadd_game(dynamic raw);

  @protected
  Game dco_decode_game(dynamic raw);

  @protected
  GameState dco_decode_game_state(dynamic raw);

  @protected
  Games dco_decode_games(dynamic raw);

  @protected
  int dco_decode_i_32(dynamic raw);

  @protected
  List<String> dco_decode_list_String(dynamic raw);

  @protected
  List<Game> dco_decode_list_game(dynamic raw);

  @protected
  List<int> dco_decode_list_prim_u_8_loose(dynamic raw);

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw);

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
  Progress
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
          SseDeserializer deserializer);

  @protected
  Progress
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
          SseDeserializer deserializer);

  @protected
  Progress
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
          SseDeserializer deserializer);

  @protected
  String sse_decode_String(SseDeserializer deserializer);

  @protected
  bool sse_decode_bool(SseDeserializer deserializer);

  @protected
  Game sse_decode_box_autoadd_game(SseDeserializer deserializer);

  @protected
  Game sse_decode_game(SseDeserializer deserializer);

  @protected
  GameState sse_decode_game_state(SseDeserializer deserializer);

  @protected
  Games sse_decode_games(SseDeserializer deserializer);

  @protected
  int sse_decode_i_32(SseDeserializer deserializer);

  @protected
  List<String> sse_decode_list_String(SseDeserializer deserializer);

  @protected
  List<Game> sse_decode_list_game(SseDeserializer deserializer);

  @protected
  List<int> sse_decode_list_prim_u_8_loose(SseDeserializer deserializer);

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer);

  @protected
  int sse_decode_u_64(SseDeserializer deserializer);

  @protected
  int sse_decode_u_8(SseDeserializer deserializer);

  @protected
  void sse_decode_unit(SseDeserializer deserializer);

  @protected
  int sse_decode_usize(SseDeserializer deserializer);

  @protected
  String cst_encode_AnyhowException(AnyhowException raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    throw UnimplementedError();
  }

  @protected
  String cst_encode_String(String raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw;
  }

  @protected
  List<dynamic> cst_encode_box_autoadd_game(Game raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return cst_encode_game(raw);
  }

  @protected
  List<dynamic> cst_encode_game(Game raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return [
      cst_encode_String(raw.name),
      cst_encode_String(raw.exe),
      cst_encode_list_String(raw.args),
      cst_encode_String(raw.icon),
      cst_encode_String(raw.url),
      cst_encode_String(raw.uuid),
      cst_encode_String(raw.sha256),
      cst_encode_game_state(raw.state)
    ];
  }

  @protected
  List<dynamic> cst_encode_games(Games raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return [cst_encode_list_game(raw.games)];
  }

  @protected
  List<dynamic> cst_encode_list_String(List<String> raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw.map(cst_encode_String).toList();
  }

  @protected
  List<dynamic> cst_encode_list_game(List<Game> raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw.map(cst_encode_game).toList();
  }

  @protected
  List<int> cst_encode_list_prim_u_8_loose(List<int> raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw;
  }

  @protected
  Uint8List cst_encode_list_prim_u_8_strict(Uint8List raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return raw;
  }

  @protected
  Object cst_encode_u_64(int raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return castNativeBigInt(raw);
  }

  @protected
  int cst_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
      Progress raw);

  @protected
  int cst_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
      Progress raw);

  @protected
  int cst_encode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
      Progress raw);

  @protected
  bool cst_encode_bool(bool raw);

  @protected
  int cst_encode_game_state(GameState raw);

  @protected
  int cst_encode_i_32(int raw);

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
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
          Progress self, SseSerializer serializer);

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
          Progress self, SseSerializer serializer);

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
          Progress self, SseSerializer serializer);

  @protected
  void sse_encode_String(String self, SseSerializer serializer);

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer);

  @protected
  void sse_encode_box_autoadd_game(Game self, SseSerializer serializer);

  @protected
  void sse_encode_game(Game self, SseSerializer serializer);

  @protected
  void sse_encode_game_state(GameState self, SseSerializer serializer);

  @protected
  void sse_encode_games(Games self, SseSerializer serializer);

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_list_String(List<String> self, SseSerializer serializer);

  @protected
  void sse_encode_list_game(List<Game> self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_8_loose(List<int> self, SseSerializer serializer);

  @protected
  void sse_encode_list_prim_u_8_strict(
      Uint8List self, SseSerializer serializer);

  @protected
  void sse_encode_u_64(int self, SseSerializer serializer);

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer);

  @protected
  void sse_encode_unit(void self, SseSerializer serializer);

  @protected
  void sse_encode_usize(int self, SseSerializer serializer);
}

// Section: wire_class

class RustLibWire implements BaseWire {
  RustLibWire.fromExternalLibrary(ExternalLibrary lib);

  dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_get_denominator(Object that) =>
          wasmModule.wire_Progress_get_denominator(that);

  dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_get_numerator(Object that) =>
          wasmModule.wire_Progress_get_numerator(that);

  dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_increment_denominator(Object that) =>
          wasmModule.wire_Progress_increment_denominator(that);

  dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_increment_numerator(Object that) =>
          wasmModule.wire_Progress_increment_numerator(that);

  dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_is_empty(Object that) =>
          wasmModule.wire_Progress_is_empty(that);

  dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_is_full(Object that) =>
          wasmModule.wire_Progress_is_full(that);

  dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_new() => wasmModule.wire_Progress_new();

  dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_set_denominator(Object that, Object denominator) =>
          wasmModule.wire_Progress_set_denominator(that, denominator);

  dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_set_numerator(Object that, Object numerator) =>
          wasmModule.wire_Progress_set_numerator(that, numerator);

  void wire_download_game(
          NativePortType port_, List<dynamic> game, Object progress) =>
      wasmModule.wire_download_game(port_, game, progress);

  void wire_extract_zip(NativePortType port_, List<int> bytes,
          List<dynamic> game, Object progress) =>
      wasmModule.wire_extract_zip(port_, bytes, game, progress);

  void wire_fetch_games(NativePortType port_) =>
      wasmModule.wire_fetch_games(port_);

  void wire_init_app(NativePortType port_) => wasmModule.wire_init_app(port_);

  void wire_run_game(NativePortType port_, List<dynamic> game) =>
      wasmModule.wire_run_game(port_, game);

  void rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
          dynamic ptr) =>
      wasmModule
          .rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
              ptr);

  void rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
          dynamic ptr) =>
      wasmModule
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
              ptr);
}

@JS('wasm_bindgen')
external RustLibWasmModule get wasmModule;

@JS()
@anonymous
class RustLibWasmModule implements WasmModule {
  @override
  external Object /* Promise */ call([String? moduleName]);

  @override
  external RustLibWasmModule bind(dynamic thisArg, String moduleName);

  external dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_get_denominator(Object that);

  external dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_get_numerator(Object that);

  external dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_increment_denominator(Object that);

  external dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_increment_numerator(Object that);

  external dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_is_empty(Object that);

  external dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_is_full(Object that);

  external dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_new();

  external dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_set_denominator(Object that, Object denominator);

  external dynamic /* flutter_rust_bridge::for_generated::WireSyncRust2DartDco */
      wire_Progress_set_numerator(Object that, Object numerator);

  external void wire_download_game(
      NativePortType port_, List<dynamic> game, Object progress);

  external void wire_extract_zip(NativePortType port_, List<int> bytes,
      List<dynamic> game, Object progress);

  external void wire_fetch_games(NativePortType port_);

  external void wire_init_app(NativePortType port_);

  external void wire_run_game(NativePortType port_, List<dynamic> game);

  external void
      rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
          dynamic ptr);

  external void
      rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockProgress(
          dynamic ptr);
}
