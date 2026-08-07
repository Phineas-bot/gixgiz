import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:gixgiz_desktop/app/app_keys.dart';
import 'package:gixgiz_desktop/core/core_client.dart';
import 'package:gixgiz_desktop/core/generated/core_contracts.g.dart';
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

  testWidgets('renders real core version protocol and readiness', (
    tester,
  ) async {
    await _pumpState(
      tester,
      const FoundationReady(
        snapshot: CoreConnectionSnapshot(
          kind: CoreConnectionKind.ready,
          applicationName: 'GixGiz',
          coreVersion: '0.1.0',
          protocolVersion: 1,
          readiness: ReadinessStatus.ready,
          readinessSummary: 'All mandatory platform services are healthy.',
        ),
      ),
    );

    expect(find.text('Platform core ready'), findsOneWidget);
    expect(find.text('0.1.0'), findsOneWidget);
    expect(find.text('1'), findsOneWidget);
    expect(find.text('ready'), findsOneWidget);
    expect(find.byKey(AppKeys.coreDetails), findsOneWidget);
    expect(find.byKey(AppKeys.primaryAction), findsNothing);
  });

  testWidgets('renders degraded state with recovery action', (tester) async {
    await _pumpState(
      tester,
      const FoundationDegraded(
        snapshot: CoreConnectionSnapshot(
          kind: CoreConnectionKind.degraded,
          coreVersion: '0.1.0',
          protocolVersion: 1,
          readiness: ReadinessStatus.degraded,
        ),
      ),
    );

    expect(find.text('Core connection degraded'), findsOneWidget);
    expect(find.byKey(AppKeys.primaryAction), findsOneWidget);
  });

  testWidgets('renders missing core and startup timeout distinctly', (
    tester,
  ) async {
    await _pumpState(
      tester,
      const FoundationUnavailable(
        issue: CoreConnectionIssue.missingCore,
        diagnosticCode: 'CORE_EXECUTABLE_MISSING',
      ),
    );
    expect(find.text('Core component missing'), findsOneWidget);

    await _pumpState(
      tester,
      const FoundationUnavailable(
        issue: CoreConnectionIssue.startupTimedOut,
        diagnosticCode: 'CORE_STARTUP_TIMEOUT',
      ),
    );
    expect(find.text('Core startup timed out'), findsOneWidget);
  });

  testWidgets('renders protocol and authentication failures distinctly', (
    tester,
  ) async {
    await _pumpState(
      tester,
      const FoundationFailed(
        issue: CoreConnectionIssue.protocolMismatch,
        diagnosticCode: 'CORE_PROTOCOL_INCOMPATIBLE',
      ),
    );
    expect(find.text('Core update required'), findsOneWidget);

    await _pumpState(
      tester,
      const FoundationFailed(
        issue: CoreConnectionIssue.authenticationFailed,
        diagnosticCode: 'CORE_AUTHENTICATION_FAILED',
      ),
    );
    expect(find.text('Core authentication failed'), findsOneWidget);
  });

  testWidgets('renders failed state with expandable safe diagnostic code', (
    tester,
  ) async {
    await _pumpState(
      tester,
      const FoundationFailed(
        issue: CoreConnectionIssue.internalFailure,
        diagnosticCode: 'CORE_TEST_FAILURE',
      ),
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
