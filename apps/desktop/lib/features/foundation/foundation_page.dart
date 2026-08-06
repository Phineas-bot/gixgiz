import 'package:flutter/material.dart';
import 'package:gixgiz_desktop/core/core_client.dart';
import 'package:gixgiz_desktop/features/foundation/foundation_screen.dart';
import 'package:gixgiz_desktop/features/foundation/foundation_state.dart';

class FoundationPage extends StatefulWidget {
  const FoundationPage({required this.coreClient, super.key});

  final CoreClient coreClient;

  @override
  State<FoundationPage> createState() => _FoundationPageState();
}

class _FoundationPageState extends State<FoundationPage> {
  FoundationState _state = const FoundationLoading();

  @override
  void initState() {
    super.initState();
    _checkConnection();
  }

  @override
  void didUpdateWidget(FoundationPage oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (!identical(oldWidget.coreClient, widget.coreClient)) {
      _checkConnection();
    }
  }

  Future<void> _checkConnection() async {
    if (!mounted) {
      return;
    }
    setState(() => _state = const FoundationLoading());

    try {
      final snapshot = await widget.coreClient.checkConnection();
      if (!mounted) {
        return;
      }
      setState(() => _state = _mapSnapshot(snapshot));
    } on Object {
      if (!mounted) {
        return;
      }
      setState(
        () => _state = const FoundationFailed(
          diagnosticCode: 'CORE_CHECK_FAILED',
        ),
      );
    }
  }

  FoundationState _mapSnapshot(CoreConnectionSnapshot snapshot) {
    return switch (snapshot.kind) {
      CoreConnectionKind.ready => const FoundationReadyPlaceholder(),
      CoreConnectionKind.degraded => const FoundationDegradedPlaceholder(
        reason: FoundationDegradedReason.degraded,
      ),
      CoreConnectionKind.unavailable => const FoundationDegradedPlaceholder(
        reason: FoundationDegradedReason.notConnected,
      ),
      CoreConnectionKind.failed => FoundationFailed(
        diagnosticCode: snapshot.diagnosticCode ?? 'CORE_UNAVAILABLE',
      ),
      CoreConnectionKind.cancelled => const FoundationCancelled(),
    };
  }

  @override
  Widget build(BuildContext context) {
    return FoundationScreen(state: _state, onRetry: _checkConnection);
  }
}
