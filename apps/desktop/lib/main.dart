import 'package:flutter/widgets.dart';
import 'package:gixgiz_desktop/app/gixgiz_app.dart';
import 'package:gixgiz_desktop/core/core_client.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(const GixGizApp(coreClient: DisconnectedCoreClient()));
}
