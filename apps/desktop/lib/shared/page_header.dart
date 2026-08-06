import 'package:flutter/material.dart';
import 'package:gixgiz_desktop/app/app_keys.dart';
import 'package:gixgiz_desktop/l10n/app_localizations.dart';

class PageHeader extends StatelessWidget {
  const PageHeader({required this.sectionTitle, super.key});

  final String sectionTitle;

  @override
  Widget build(BuildContext context) {
    final localizations = AppLocalizations.of(context);

    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Icon(
          Icons.hub_outlined,
          size: 36,
          color: Theme.of(context).colorScheme.primary,
          semanticLabel: localizations.appMarkSemanticLabel,
        ),
        const SizedBox(width: 16),
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                localizations.appTitle,
                key: AppKeys.brand,
                style: Theme.of(context).textTheme.headlineMedium,
              ),
              const SizedBox(height: 4),
              Text(
                sectionTitle,
                style: Theme.of(context).textTheme.titleMedium?.copyWith(
                  color: Theme.of(context).colorScheme.onSurfaceVariant,
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }
}
