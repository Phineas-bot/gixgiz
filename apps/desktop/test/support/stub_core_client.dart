import 'package:gixgiz_desktop/core/core_client.dart';

class StubCoreClient extends CoreClient {
  const StubCoreClient(this.snapshot);

  final CoreConnectionSnapshot snapshot;

  @override
  Future<CoreConnectionSnapshot> checkConnection() async => snapshot;
}
