import 'package:flutter_test/flutter_test.dart';
import 'package:kubelab_mobile/main.dart';

void main() {
  testWidgets('KubeLab App Smoke Test', (WidgetTester tester) async {
    await tester.pumpWidget(const KubeLabApp());
    expect(find.text('KUBELAB TRACKS'), findsOneWidget);
  });
}
