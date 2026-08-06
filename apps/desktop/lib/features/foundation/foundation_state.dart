sealed class FoundationState {
  const FoundationState();
}

final class FoundationLoading extends FoundationState {
  const FoundationLoading();
}

final class FoundationReadyPlaceholder extends FoundationState {
  const FoundationReadyPlaceholder();
}

enum FoundationDegradedReason { notConnected, degraded }

final class FoundationDegradedPlaceholder extends FoundationState {
  const FoundationDegradedPlaceholder({required this.reason});

  final FoundationDegradedReason reason;
}

final class FoundationFailed extends FoundationState {
  const FoundationFailed({required this.diagnosticCode});

  final String diagnosticCode;
}

final class FoundationCancelled extends FoundationState {
  const FoundationCancelled();
}
