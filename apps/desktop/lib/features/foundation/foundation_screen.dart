import 'package:flutter/material.dart';
import 'package:gixgiz_desktop/app/app_keys.dart';
import 'package:gixgiz_desktop/core/core_client.dart';
import 'package:gixgiz_desktop/features/foundation/foundation_state.dart';
import 'package:gixgiz_desktop/l10n/app_localizations.dart';
import 'package:gixgiz_desktop/shared/page_header.dart';

class FoundationScreen extends StatelessWidget {
  const FoundationScreen({
    required this.state,
    required this.onRetry,
    this.primaryActionFocusNode,
    super.key,
  });

  final FoundationState state;
  final VoidCallback onRetry;
  final FocusNode? primaryActionFocusNode;

  @override
  Widget build(BuildContext context) {
    final localizations = AppLocalizations.of(context);
    final status = _statusFor(localizations, state);

    return SafeArea(
      child: FocusTraversalGroup(
        policy: OrderedTraversalPolicy(),
        child: SingleChildScrollView(
          padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 28),
          child: Align(
            alignment: Alignment.topCenter,
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 840),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  PageHeader(sectionTitle: localizations.foundationTitle),
                  const SizedBox(height: 32),
                  _StatusPanel(
                    status: status,
                    state: state,
                    onRetry: onRetry,
                    primaryActionFocusNode: primaryActionFocusNode,
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  _FoundationStatus _statusFor(
    AppLocalizations localizations,
    FoundationState state,
  ) {
    return switch (state) {
      FoundationLoading() => _FoundationStatus(
        title: localizations.loadingTitle,
        message: localizations.loadingMessage,
        icon: Icons.sync,
        tone: _StatusTone.neutral,
      ),
      FoundationReady(:final snapshot) => _FoundationStatus(
        title: localizations.coreReadyTitle,
        message: snapshot.readinessSummary ?? localizations.coreReadyMessage,
        icon: Icons.check_circle_outline,
        tone: _StatusTone.ready,
      ),
      FoundationDegraded(:final snapshot) => _FoundationStatus(
        title: localizations.degradedTitle,
        message: snapshot.readinessSummary ?? localizations.degradedMessage,
        icon: Icons.warning_amber,
        tone: _StatusTone.warning,
      ),
      FoundationUnavailable(:final issue) => _unavailableStatus(
        localizations,
        issue,
      ),
      FoundationFailed(:final issue) => _failedStatus(localizations, issue),
      FoundationCancelled() => _FoundationStatus(
        title: localizations.cancelledTitle,
        message: localizations.cancelledMessage,
        icon: Icons.cancel_outlined,
        tone: _StatusTone.neutral,
      ),
    };
  }

  _FoundationStatus _unavailableStatus(
    AppLocalizations localizations,
    CoreConnectionIssue issue,
  ) {
    return switch (issue) {
      CoreConnectionIssue.missingCore => _FoundationStatus(
        title: localizations.missingCoreTitle,
        message: localizations.missingCoreMessage,
        icon: Icons.extension_off_outlined,
        tone: _StatusTone.error,
      ),
      CoreConnectionIssue.startupTimedOut => _FoundationStatus(
        title: localizations.startupTimedOutTitle,
        message: localizations.startupTimedOutMessage,
        icon: Icons.timer_off_outlined,
        tone: _StatusTone.warning,
      ),
      CoreConnectionIssue.connectionLost => _FoundationStatus(
        title: localizations.connectionLostTitle,
        message: localizations.connectionLostMessage,
        icon: Icons.link_off,
        tone: _StatusTone.warning,
      ),
      _ => _FoundationStatus(
        title: localizations.notConnectedTitle,
        message: localizations.notConnectedMessage,
        icon: Icons.link_off,
        tone: _StatusTone.warning,
      ),
    };
  }

  _FoundationStatus _failedStatus(
    AppLocalizations localizations,
    CoreConnectionIssue issue,
  ) {
    return switch (issue) {
      CoreConnectionIssue.protocolMismatch => _FoundationStatus(
        title: localizations.protocolMismatchTitle,
        message: localizations.protocolMismatchMessage,
        icon: Icons.system_update_alt,
        tone: _StatusTone.error,
      ),
      CoreConnectionIssue.authenticationFailed => _FoundationStatus(
        title: localizations.authenticationFailedTitle,
        message: localizations.authenticationFailedMessage,
        icon: Icons.lock_outline,
        tone: _StatusTone.error,
      ),
      _ => _FoundationStatus(
        title: localizations.failedTitle,
        message: localizations.failedMessage,
        icon: Icons.error_outline,
        tone: _StatusTone.error,
      ),
    };
  }
}

class _StatusPanel extends StatelessWidget {
  const _StatusPanel({
    required this.status,
    required this.state,
    required this.onRetry,
    required this.primaryActionFocusNode,
  });

  final _FoundationStatus status;
  final FoundationState state;
  final VoidCallback onRetry;
  final FocusNode? primaryActionFocusNode;

  @override
  Widget build(BuildContext context) {
    final localizations = AppLocalizations.of(context);
    final colorScheme = Theme.of(context).colorScheme;
    final statusColor = switch (status.tone) {
      _StatusTone.ready => colorScheme.primary,
      _StatusTone.warning => colorScheme.tertiary,
      _StatusTone.error => colorScheme.error,
      _StatusTone.neutral => colorScheme.secondary,
    };
    final snapshot = switch (state) {
      FoundationReady(:final snapshot) ||
      FoundationDegraded(:final snapshot) => snapshot,
      _ => null,
    };

    return Card(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Semantics(
              key: AppKeys.foundationStatus,
              container: true,
              liveRegion: true,
              label: localizations.foundationStatusSemanticLabel(
                status.title,
                status.message,
              ),
              child: ExcludeSemantics(
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Icon(status.icon, color: statusColor, size: 32),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            status.title,
                            style: Theme.of(context).textTheme.titleLarge,
                          ),
                          const SizedBox(height: 8),
                          Text(
                            status.message,
                            style: Theme.of(context).textTheme.bodyLarge,
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ),
            if (state is FoundationLoading) ...[
              const SizedBox(height: 24),
              Semantics(
                key: AppKeys.foundationProgress,
                label: localizations.loadingProgressSemanticLabel,
                value: localizations.inProgressSemanticValue,
                child: const LinearProgressIndicator(),
              ),
            ],
            if (snapshot != null) ...[
              const SizedBox(height: 24),
              _CoreDetails(snapshot: snapshot),
            ],
            if (state
                case FoundationFailed(:final diagnosticCode) ||
                    FoundationUnavailable(:final diagnosticCode)) ...[
              const SizedBox(height: 16),
              ExpansionTile(
                key: AppKeys.diagnostics,
                tilePadding: EdgeInsets.zero,
                childrenPadding: const EdgeInsets.only(bottom: 8),
                title: Text(localizations.diagnosticsLabel),
                children: [
                  Align(
                    alignment: AlignmentDirectional.centerStart,
                    child: SelectableText(
                      localizations.diagnosticCodeLabel(diagnosticCode),
                    ),
                  ),
                ],
              ),
            ],
            if (state is FoundationDegraded ||
                state is FoundationUnavailable ||
                state is FoundationFailed ||
                state is FoundationCancelled) ...[
              const SizedBox(height: 20),
              Align(
                alignment: AlignmentDirectional.centerStart,
                child: FilledButton.icon(
                  key: AppKeys.primaryAction,
                  focusNode: primaryActionFocusNode,
                  onPressed: onRetry,
                  icon: const Icon(Icons.refresh),
                  label: Text(localizations.checkAgainAction),
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }
}

class _CoreDetails extends StatelessWidget {
  const _CoreDetails({required this.snapshot});

  final CoreConnectionSnapshot snapshot;

  @override
  Widget build(BuildContext context) {
    final localizations = AppLocalizations.of(context);
    final textTheme = Theme.of(context).textTheme;
    final readiness =
        snapshot.readiness?.wireValue ?? localizations.unknownValue;
    return Semantics(
      key: AppKeys.coreDetails,
      container: true,
      label: localizations.coreDetailsSemanticLabel(
        snapshot.applicationName ?? localizations.unknownValue,
        snapshot.coreVersion ?? localizations.unknownValue,
        snapshot.protocolVersion?.toString() ?? localizations.unknownValue,
        readiness,
      ),
      child: ExcludeSemantics(
        child: Wrap(
          spacing: 32,
          runSpacing: 16,
          children: [
            _Detail(
              label: localizations.coreApplicationLabel,
              value: snapshot.applicationName ?? localizations.unknownValue,
              textTheme: textTheme,
            ),
            _Detail(
              label: localizations.coreVersionLabel,
              value: snapshot.coreVersion ?? localizations.unknownValue,
              textTheme: textTheme,
            ),
            _Detail(
              label: localizations.protocolVersionLabel,
              value:
                  snapshot.protocolVersion?.toString() ??
                  localizations.unknownValue,
              textTheme: textTheme,
            ),
            _Detail(
              label: localizations.readinessLabel,
              value: readiness,
              textTheme: textTheme,
            ),
          ],
        ),
      ),
    );
  }
}

class _Detail extends StatelessWidget {
  const _Detail({
    required this.label,
    required this.value,
    required this.textTheme,
  });

  final String label;
  final String value;
  final TextTheme textTheme;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 160,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(label, style: textTheme.labelLarge),
          const SizedBox(height: 4),
          Text(value, style: textTheme.bodyLarge),
        ],
      ),
    );
  }
}

enum _StatusTone { ready, warning, error, neutral }

class _FoundationStatus {
  const _FoundationStatus({
    required this.title,
    required this.message,
    required this.icon,
    required this.tone,
  });

  final String title;
  final String message;
  final IconData icon;
  final _StatusTone tone;
}
