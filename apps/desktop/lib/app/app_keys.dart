import 'package:flutter/widgets.dart';

abstract final class AppKeys {
  static const brand = ValueKey<String>('app.brand');
  static const foundationNavigation = ValueKey<String>('navigation.foundation');
  static const aboutNavigation = ValueKey<String>('navigation.about');
  static const foundationStatus = ValueKey<String>('foundation.status');
  static const foundationProgress = ValueKey<String>('foundation.progress');
  static const primaryAction = ValueKey<String>('foundation.primary_action');
  static const diagnostics = ValueKey<String>('foundation.diagnostics');
  static const coreDetails = ValueKey<String>('foundation.core_details');
}
