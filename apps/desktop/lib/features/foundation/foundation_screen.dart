import 'package:flutter/material.dart';
import 'package:gixgiz_desktop/app/app_keys.dart';
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
      FoundationReadyPlaceholder() => _FoundationStatus(
        title: localizations.readyPlaceholderTitle,
        message: localizations.readyPlaceholderMessage,
        icon: Icons.check_circle_outline,
        tone: _StatusTone.ready,
      ),
      FoundationDegradedPlaceholder(
        reason: FoundationDegradedReason.notConnected,
      ) =>
        _FoundationStatus(
          title: localizations.notConnectedTitle,
          message: localizations.notConnectedMessage,
          icon: Icons.link_off,
          tone: _StatusTone.warning,
        ),
      FoundationDegradedPlaceholder() => _FoundationStatus(
        title: localizations.degradedTitle,
        message: localizations.degradedMessage,
        icon: Icons.warning_amber,
        tone: _StatusTone.warning,
      ),
      FoundationFailed() => _FoundationStatus(
        title: localizations.failedTitle,
        message: localizations.failedMessage,
        icon: Icons.error_outline,
        tone: _StatusTone.error,
      ),
      FoundationCancelled() => _FoundationStatus(
        title: localizations.cancelledTitle,
        message: localizations.cancelledMessage,
        icon: Icons.cancel_outlined,
        tone: _StatusTone.neutral,
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
            if (state case FoundationFailed(:final diagnosticCode)) ...[
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
            if (state is FoundationDegradedPlaceholder ||
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
