// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.32.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// The type `ChannelPair` is not used by any `pub` functions, thus it is ignored.
// The type `RemoteControl` is not used by any `pub` functions, thus it is ignored.

Stream<int> regTick({dynamic hint}) => RustLib.instance.api.regTick(hint: hint);

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<StopwatchRemote>>
@sealed
class StopwatchRemote extends RustOpaque {
  StopwatchRemote.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  StopwatchRemote.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_StopwatchRemote,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_StopwatchRemote,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_StopwatchRemotePtr,
  );

  factory StopwatchRemote({dynamic hint}) =>
      RustLib.instance.api.stopwatchRemoteNew(hint: hint);

  Future<void> startTimer({dynamic hint}) =>
      RustLib.instance.api.stopwatchRemoteStartTimer(that: this, hint: hint);

  Future<void> stopTimer({dynamic hint}) =>
      RustLib.instance.api.stopwatchRemoteStopTimer(that: this, hint: hint);

  Stream<int> tick({dynamic hint}) =>
      RustLib.instance.api.stopwatchRemoteTick(that: this, hint: hint);
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<Timer>>
@sealed
class Timer extends RustOpaque {
  Timer.dcoDecode(List<dynamic> wire) : super.dcoDecode(wire, _kStaticData);

  Timer.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_Timer,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_Timer,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_TimerPtr,
  );

  factory Timer({dynamic hint}) => RustLib.instance.api.timerNew(hint: hint);

  String returnSomething({dynamic hint}) =>
      RustLib.instance.api.timerReturnSomething(that: this, hint: hint);

  Future<void> startTimer({dynamic hint}) =>
      RustLib.instance.api.timerStartTimer(that: this, hint: hint);

  Future<void> stopTimer({dynamic hint}) =>
      RustLib.instance.api.timerStopTimer(that: this, hint: hint);
}
