import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:kubelab_mobile/main.dart';
import 'package:kubelab_mobile/screens/lesson_screen.dart';

void main() {
  testWidgets('KubeLab App Smoke & Tab Navigation Test', (WidgetTester tester) async {
    await tester.pumpWidget(const KubeLabApp());
    await tester.pumpAndSettle();

    // 1. Initial tab: TracksScreen
    expect(find.text('KUBELAB TRACKS'), findsOneWidget);
    expect(find.text('Linux & Containers'), findsOneWidget);

    // 2. Switch to Skill Tree tab
    await tester.tap(find.byIcon(Icons.account_tree));
    await tester.pumpAndSettle();
    expect(find.text('SKILL TREE DAG'), findsOneWidget);
    expect(find.text('Kubernetes Workloads'), findsOneWidget);

    // 3. Switch to Profile tab
    await tester.tap(find.byIcon(Icons.person));
    await tester.pumpAndSettle();
    expect(find.text('LEARNER PROFILE'), findsOneWidget);
    expect(find.text('Cloud-Native Practitioner'), findsOneWidget);
    expect(find.text('1,250 XP • Level 3'), findsOneWidget);
  });

  testWidgets('Lesson Screen Renders Cleanly', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: LessonScreen(),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Understanding Pods'), findsOneWidget);
    expect(find.text('The Atomic Unit of Kubernetes'), findsOneWidget);
    expect(find.text('CONTINUE ON DESKTOP'), findsOneWidget);
  });
}
