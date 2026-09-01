import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:kubelab_mobile/main.dart';
import 'package:kubelab_mobile/data/curriculum_data.dart';
import 'package:kubelab_mobile/screens/lesson_screen.dart';
import 'package:kubelab_mobile/screens/progress_sync_screen.dart';
import 'package:kubelab_mobile/screens/notifications_screen.dart';
import 'package:kubelab_mobile/screens/settings_screen.dart';

void main() {
  setUpAll(() async {
    final file = File('assets/data/curriculum.json');
    if (await file.exists()) {
      final jsonStr = await file.readAsString();
      CurriculumRepository.initializeFromJson(jsonStr);
    }
  });

  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  testWidgets('KubeLab App Smoke & Tab Navigation Test', (WidgetTester tester) async {
    await tester.pumpWidget(const KubeLabApp());
    await tester.pumpAndSettle();

    // 1. Initial tab: TracksScreen
    expect(find.text('15 ENGINEERING TRACKS'), findsOneWidget);
    expect(find.text('Linux & Container Fundamentals'), findsOneWidget);

    // 2. Switch to Skill Tree tab
    await tester.tap(find.byIcon(Icons.account_tree).last);
    await tester.pumpAndSettle();
    expect(find.text('15-TRACK SKILL TREE DAG'), findsOneWidget);
    expect(find.text('Kubernetes Core Architecture & Workloads'), findsOneWidget);

    // 3. Switch to Profile tab
    await tester.tap(find.byIcon(Icons.person).last);
    await tester.pumpAndSettle();
    expect(find.text('LEARNER PROFILE'), findsOneWidget);
    expect(find.text('Cloud-Native Engineer'), findsOneWidget);
  });

  testWidgets('Progress Sync Screen Renders Cleanly', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: ProgressSyncScreen()));
    await tester.pumpAndSettle();
    expect(find.text('CLOUD PROGRESS SYNC'), findsOneWidget);
  });

  testWidgets('Notifications Screen Renders Cleanly', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: NotificationsScreen()));
    await tester.pumpAndSettle();
    expect(find.text('NOTIFICATIONS & ALERTS'), findsOneWidget);
  });

  testWidgets('Settings Screen Renders Cleanly', (WidgetTester tester) async {
    await tester.pumpWidget(const MaterialApp(home: SettingsScreen()));
    await tester.pumpAndSettle();
    expect(find.text('SETTINGS & PREFERENCES'), findsOneWidget);
  });

  testWidgets('Lesson Screen Renders Cleanly and Links to Quiz/Handoff', (WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: LessonScreen(),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('LAUNCH LAB'), findsOneWidget);
    expect(find.text('OPEN LIVE LAB WORKSPACE'), findsOneWidget);
    expect(find.text('ASK AI SOCRATIC TUTOR'), findsOneWidget);
  });
}
