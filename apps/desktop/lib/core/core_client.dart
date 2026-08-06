enum CoreConnectionKind { ready, degraded, unavailable, failed, cancelled }

class CoreConnectionSnapshot {
  const CoreConnectionSnapshot({required this.kind, this.diagnosticCode});

  final CoreConnectionKind kind;
  final String? diagnosticCode;
}

abstract interface class CoreClient {
  Future<CoreConnectionSnapshot> checkConnection();
}

class DisconnectedCoreClient implements CoreClient {
  const DisconnectedCoreClient();

  @override
  Future<CoreConnectionSnapshot> checkConnection() async {
    return const CoreConnectionSnapshot(kind: CoreConnectionKind.unavailable);
  }
}
