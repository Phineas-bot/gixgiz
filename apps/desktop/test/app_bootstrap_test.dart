import 'package:flutter_test/flutter_test.dart';
import 'package:gixgiz_desktop/app/app_keys.dart';
import 'package:gixgiz_desktop/app/gixgiz_app.dart';
import 'package:gixgiz_desktop/core/core_client.dart';

void main() {
  testWidgets('application boots with branded disconnected foundation', (
    tester,
  ) async {
    await tester.pumpWidget(
      const GixGizApp(coreClient: DisconnectedCoreClient()),
    );

    expect(find.byKey(AppKeys.brand), findsOneWidget);
    expect(find.text('Checking foundation'), findsOneWidget);

    await tester.pump();

    expect(find.text('Core not connected'), findsOneWidget);
    expect(find.text('Flutter Demo'), findsNothing);
  });
}
