import 'package:flutter/material.dart';
import 'package:gixgiz_desktop/l10n/app_localizations.dart';
import 'package:gixgiz_desktop/shared/page_header.dart';

class AboutScreen extends StatelessWidget {
  const AboutScreen({
    required this.applicationId,
    required this.releaseName,
    super.key,
  });

  final String applicationId;
  final String releaseName;

  @override
  Widget build(BuildContext context) {
    final localizations = AppLocalizations.of(context);

    return SafeArea(
      child: SingleChildScrollView(
        padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 28),
        child: Align(
          alignment: Alignment.topCenter,
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 840),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                PageHeader(sectionTitle: localizations.aboutTitle),
                const SizedBox(height: 32),
                Text(
                  localizations.aboutSummary,
                  style: Theme.of(context).textTheme.headlineSmall,
                ),
                const SizedBox(height: 24),
                _DetailRow(
                  label: localizations.releaseLabel,
                  value: releaseName,
                ),
                const Divider(height: 32),
                _DetailRow(
                  label: localizations.applicationIdLabel,
                  value: applicationId,
                ),
                const Divider(height: 32),
                _DetailRow(
                  label: localizations.coreConnectionLabel,
                  value: localizations.coreNotConnectedValue,
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class _DetailRow extends StatelessWidget {
  const _DetailRow({required this.label, required this.value});

  final String label;
  final String value;

  @override
  Widget build(BuildContext context) {
    return Wrap(
      alignment: WrapAlignment.spaceBetween,
      runSpacing: 8,
      spacing: 24,
      children: [
        Text(label, style: Theme.of(context).textTheme.titleMedium),
        SelectableText(value, style: Theme.of(context).textTheme.bodyLarge),
      ],
    );
  }
}
