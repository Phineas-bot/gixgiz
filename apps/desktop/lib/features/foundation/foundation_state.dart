import 'package:gixgiz_desktop/core/core_client.dart';

sealed class FoundationState {
  const FoundationState();
}

final class FoundationLoading extends FoundationState {
  const FoundationLoading();
}

final class FoundationReady extends FoundationState {
  const FoundationReady({required this.snapshot});

  final CoreConnectionSnapshot snapshot;
}

final class FoundationDegraded extends FoundationState {
  const FoundationDegraded({required this.snapshot});

  final CoreConnectionSnapshot snapshot;
}

final class FoundationUnavailable extends FoundationState {
  const FoundationUnavailable({
    required this.issue,
    required this.diagnosticCode,
  });

  final CoreConnectionIssue issue;
  final String diagnosticCode;
}

final class FoundationFailed extends FoundationState {
  const FoundationFailed({required this.issue, required this.diagnosticCode});

  final CoreConnectionIssue issue;
  final String diagnosticCode;
}

final class FoundationCancelled extends FoundationState {
  const FoundationCancelled();
}
