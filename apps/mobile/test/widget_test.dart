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
    await tester.tap(find.byIcon(Icons.account_tree).last);
    await tester.pumpAndSettle();
    expect(find.text('SKILL TREE DAG'), findsOneWidget);
    expect(find.text('Kubernetes Workloads'), findsOneWidget);

    // 3. Switch to Profile tab
    await tester.tap(find.byIcon(Icons.person).last);
    await tester.pumpAndSettle();
    expect(find.text('LEARNER PROFILE'), findsOneWidget);
    expect(find.text('Cloud-Native Practitioner'), findsOneWidget);
    expect(find.text('1,250 XP • Level 3'), findsOneWidget);
  });

  testWidgets('App Bar Action Buttons Open Screens', (WidgetTester tester) async {
    await tester.pumpWidget(const KubeLabApp());
    await tester.pumpAndSettle();

    // Tap sync button
    await tester.tap(find.byIcon(Icons.sync));
    await tester.pumpAndSettle();
    expect(find.text('CLOUD PROGRESS SYNC'), findsOneWidget);
    await tester.pageBack();
    await tester.pumpAndSettle();

    // Tap notifications button
    await tester.tap(find.byIcon(Icons.notifications_outlined));
    await tester.pumpAndSettle();
    expect(find.text('NOTIFICATIONS & ALERTS'), findsOneWidget);
    await tester.pageBack();
    await tester.pumpAndSettle();

    // Tap settings button
    await tester.tap(find.byIcon(Icons.settings_outlined));
    await tester.pumpAndSettle();
    expect(find.text('SETTINGS & PREFERENCES'), findsOneWidget);
    await tester.pageBack();
    await tester.pumpAndSettle();
  });

  testWidgets('Lesson Screen Renders Cleanly and Links to Quiz/Handoff', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: LessonScreen(),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Understanding Pods'), findsOneWidget);
    expect(find.text('The Atomic Unit of Kubernetes'), findsOneWidget);
    expect(find.text('CONTINUE ON DESKTOP'), findsOneWidget);
    expect(find.text('TAKE LESSON QUIZ (+300 XP)'), findsOneWidget);
  });
}
