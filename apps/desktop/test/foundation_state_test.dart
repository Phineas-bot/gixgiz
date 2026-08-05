import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:gixgiz_desktop/app/app_keys.dart';
import 'package:gixgiz_desktop/features/foundation/foundation_screen.dart';
import 'package:gixgiz_desktop/features/foundation/foundation_state.dart';
import 'package:gixgiz_desktop/l10n/app_localizations.dart';

void main() {
  testWidgets('renders loading state', (tester) async {
    await _pumpState(tester, const FoundationLoading());

    expect(find.text('Checking foundation'), findsOneWidget);
    expect(find.byKey(AppKeys.foundationProgress), findsOneWidget);
    expect(find.byKey(AppKeys.primaryAction), findsNothing);
  });

  testWidgets('renders ready placeholder without claiming verified readiness', (
    tester,
  ) async {
    await _pumpState(tester, const FoundationReadyPlaceholder());

    expect(find.text('Presentation placeholder'), findsOneWidget);
    expect(find.textContaining('has not verified'), findsOneWidget);
    expect(find.byKey(AppKeys.primaryAction), findsNothing);
  });

  testWidgets('renders degraded state with recovery action', (tester) async {
    await _pumpState(
      tester,
      const FoundationDegradedPlaceholder(
        reason: FoundationDegradedReason.degraded,
      ),
    );

    expect(find.text('Core connection degraded'), findsOneWidget);
    expect(find.byKey(AppKeys.primaryAction), findsOneWidget);
  });

  testWidgets('renders failed state with expandable safe diagnostic code', (
    tester,
  ) async {
    await _pumpState(
      tester,
      const FoundationFailed(diagnosticCode: 'CORE_TEST_FAILURE'),
    );

    expect(find.text('Foundation check failed'), findsOneWidget);
    expect(find.textContaining('CORE_TEST_FAILURE'), findsNothing);

    await tester.tap(find.byKey(AppKeys.diagnostics));
    await tester.pumpAndSettle();

    expect(find.text('Diagnostic code: CORE_TEST_FAILURE'), findsOneWidget);
  });

  testWidgets('renders cancelled state separately from failure', (
    tester,
  ) async {
    await _pumpState(tester, const FoundationCancelled());

    expect(find.text('Foundation check cancelled'), findsOneWidget);
    expect(find.textContaining('No system changes were made'), findsOneWidget);
    expect(find.byKey(AppKeys.primaryAction), findsOneWidget);
  });
}

Future<void> _pumpState(WidgetTester tester, FoundationState state) async {
  await tester.pumpWidget(
    MaterialApp(
      localizationsDelegates: AppLocalizations.localizationsDelegates,
      supportedLocales: AppLocalizations.supportedLocales,
      home: Scaffold(
        body: FoundationScreen(state: state, onRetry: () {}),
      ),
    ),
  );
  await tester.pump();
}
