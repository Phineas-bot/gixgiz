import 'package:flutter/material.dart';
import 'package:gixgiz_desktop/app/app_identity.dart';
import 'package:gixgiz_desktop/app/app_routes.dart';
import 'package:gixgiz_desktop/app/app_theme.dart';
import 'package:gixgiz_desktop/core/core_client.dart';
import 'package:gixgiz_desktop/features/about/about_screen.dart';
import 'package:gixgiz_desktop/features/foundation/foundation_page.dart';
import 'package:gixgiz_desktop/l10n/app_localizations.dart';
import 'package:gixgiz_desktop/shared/app_shell.dart';

class GixGizApp extends StatelessWidget {
  const GixGizApp({required this.coreClient, super.key});

  final CoreClient coreClient;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      onGenerateTitle: (context) => AppLocalizations.of(context).appTitle,
      theme: AppTheme.light,
      darkTheme: AppTheme.dark,
      themeMode: ThemeMode.system,
      localizationsDelegates: AppLocalizations.localizationsDelegates,
      supportedLocales: AppLocalizations.supportedLocales,
      initialRoute: AppRoutes.foundation,
      routes: {
        AppRoutes.foundation: (context) => AppShell(
          selectedIndex: 0,
          child: FoundationPage(coreClient: coreClient),
        ),
        AppRoutes.about: (context) => const AppShell(
          selectedIndex: 1,
          child: AboutScreen(
            applicationId: AppIdentity.applicationId,
            releaseName: AppIdentity.releaseName,
          ),
        ),
      },
    );
  }
}
