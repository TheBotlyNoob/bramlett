// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.21.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<Games> fetchGames({dynamic hint}) =>
    RustLib.instance.api.fetchGames(hint: hint);

Future<Uint8List> downloadGame(
        {required Game game, required Progress progress, dynamic hint}) =>
    RustLib.instance.api
        .downloadGame(game: game, progress: progress, hint: hint);

Future<void> extractZip(
        {required List<int> bytes,
        required Game game,
        required Progress progress,
        dynamic hint}) =>
    RustLib.instance.api
        .extractZip(bytes: bytes, game: game, progress: progress, hint: hint);

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<Progress>>
@sealed
class Progress extends RustOpaque {
  Progress.dcoDecode(List<dynamic> wire) : super.dcoDecode(wire, _kStaticData);

  Progress.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_Progress,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_Progress,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_ProgressPtr,
  );

  int getDenominator({dynamic hint}) =>
      RustLib.instance.api.progressGetDenominator(
        that: this,
      );

  int getNumerator({dynamic hint}) => RustLib.instance.api.progressGetNumerator(
        that: this,
      );

  void incrementDenominator({dynamic hint}) =>
      RustLib.instance.api.progressIncrementDenominator(
        that: this,
      );

  void incrementNumerator({dynamic hint}) =>
      RustLib.instance.api.progressIncrementNumerator(
        that: this,
      );

  bool isEmpty({dynamic hint}) => RustLib.instance.api.progressIsEmpty(
        that: this,
      );

  bool isFull({dynamic hint}) => RustLib.instance.api.progressIsFull(
        that: this,
      );

  static Progress newProgress({dynamic hint}) =>
      RustLib.instance.api.progressNew(hint: hint);

  void setDenominator({required int denominator, dynamic hint}) =>
      RustLib.instance.api.progressSetDenominator(
        that: this,
        denominator: denominator,
      );

  void setNumerator({required int numerator, dynamic hint}) =>
      RustLib.instance.api.progressSetNumerator(
        that: this,
        numerator: numerator,
      );
}

class Game {
  final String name;
  final String exe;
  final List<String> args;
  final String icon;
  final String url;
  final String uuid;
  final String sha256;
  final GameState state;

  const Game({
    required this.name,
    required this.exe,
    required this.args,
    required this.icon,
    required this.url,
    required this.uuid,
    required this.sha256,
    required this.state,
  });

  @override
  int get hashCode =>
      name.hashCode ^
      exe.hashCode ^
      args.hashCode ^
      icon.hashCode ^
      url.hashCode ^
      uuid.hashCode ^
      sha256.hashCode ^
      state.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Game &&
          runtimeType == other.runtimeType &&
          name == other.name &&
          exe == other.exe &&
          args == other.args &&
          icon == other.icon &&
          url == other.url &&
          uuid == other.uuid &&
          sha256 == other.sha256 &&
          state == other.state;
}

enum GameState {
  notInstalled,
  installed,
}

class Games {
  final List<Game> games;

  const Games({
    required this.games,
  });

  @override
  int get hashCode => games.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Games &&
          runtimeType == other.runtimeType &&
          games == other.games;
}
