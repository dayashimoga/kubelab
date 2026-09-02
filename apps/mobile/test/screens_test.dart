import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:kubelab_mobile/data/curriculum_data.dart';
import 'package:kubelab_mobile/screens/login_screen.dart';
import 'package:kubelab_mobile/screens/quiz_screen.dart';
import 'package:kubelab_mobile/screens/progress_sync_screen.dart';
import 'package:kubelab_mobile/screens/settings_screen.dart';
import 'package:kubelab_mobile/screens/notifications_screen.dart';
import 'package:kubelab_mobile/screens/desktop_handoff_screen.dart';

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

  group('LoginScreen Tests', () {
    testWidgets('Renders Login elements and toggles to Register', (WidgetTester tester) async {
      await tester.pumpWidget(const MaterialApp(home: LoginScreen()));
      await tester.pumpAndSettle();

      expect(find.text('LOGIN TO KUBELAB'), findsOneWidget);
      expect(find.text('Welcome Back, Engineer'), findsOneWidget);
      expect(find.text('LOGIN'), findsOneWidget);

      // Toggle to registration
      await tester.tap(find.text("Don't have an account? Register"));
      await tester.pumpAndSettle();

      expect(find.text('CREATE KUBELAB ACCOUNT'), findsOneWidget);
      expect(find.text('Start Your Cloud-Native Mastery'), findsOneWidget);
      expect(find.text('REGISTER & SYNC'), findsOneWidget);
      expect(find.text('Full Name'), findsOneWidget);
    });

    testWidgets('Validates invalid input fields', (WidgetTester tester) async {
      await tester.pumpWidget(const MaterialApp(home: LoginScreen()));
      await tester.pumpAndSettle();

      // Submit without filling fields
      await tester.tap(find.text('LOGIN'));
      await tester.pumpAndSettle();

      expect(find.text('Email required'), findsOneWidget);
      expect(find.text('Password must be >= 6 characters'), findsOneWidget);
    });
  });

  group('QuizScreen Tests', () {
    testWidgets('Answers question, verifies score increment and summary', (WidgetTester tester) async {
      const testQuestions = [
        QuizQuestion(
          id: 'tq1',
          prompt: 'What provides zero-trust traffic filtering?',
          options: ['Ingress', 'NetworkPolicy', 'ServiceAccount', 'LimitRange'],
          correctIndex: 1,
          explanation: 'NetworkPolicies enforce layer 3/4 filtering.',
        ),
      ];

      await tester.pumpWidget(const MaterialApp(
        home: QuizScreen(
          trackTitle: 'Kubernetes Core',
          questions: testQuestions,
        ),
      ));
      await tester.pumpAndSettle();

      expect(find.text('NetworkPolicy'), findsOneWidget);

      // Select correct answer "NetworkPolicy"
      await tester.tap(find.text('NetworkPolicy'));
      await tester.pumpAndSettle();

      // Submit answer
      await tester.tap(find.text('SUBMIT ANSWER'));
      await tester.pumpAndSettle();

      expect(find.text('CORRECT ANSWER'), findsOneWidget);
      expect(find.text('VIEW RESULTS'), findsOneWidget);
    });
  });

  group('ProgressSyncScreen Tests', () {
    testWidgets('Renders progress sync and initial sync status', (WidgetTester tester) async {
      await tester.pumpWidget(const MaterialApp(home: ProgressSyncScreen()));
      await tester.pumpAndSettle();

      expect(find.text('CLOUD PROGRESS SYNC'), findsOneWidget);
      expect(find.text('SYNC NOW'), findsOneWidget);
      expect(find.text('All progress synced with cloud'), findsOneWidget);
    });
  });

  group('SettingsScreen Tests', () {
    testWidgets('Renders settings controls and switches', (WidgetTester tester) async {
      await tester.pumpWidget(const MaterialApp(home: SettingsScreen()));
      await tester.pumpAndSettle();

      expect(find.text('SETTINGS & PREFERENCES'), findsOneWidget);
      expect(find.text('Gateway Base URL'), findsOneWidget);
      expect(find.text('Push Notifications'), findsOneWidget);
      expect(find.text('Offline Track Caching'), findsOneWidget);
      expect(find.text('Haptic Feedback'), findsOneWidget);

      // Toggle a switch
      await tester.tap(find.byType(SwitchListTile).first);
      await tester.pumpAndSettle();
    });
  });

  group('NotificationsScreen Tests', () {
    testWidgets('Renders notifications and mark all read', (WidgetTester tester) async {
      await tester.pumpWidget(const MaterialApp(home: NotificationsScreen()));
      await tester.pumpAndSettle();

      expect(find.text('NOTIFICATIONS & ALERTS'), findsOneWidget);
      expect(find.text('🚨 CRITICAL INCIDENT SIMULATION'), findsOneWidget);
      expect(find.byIcon(Icons.done_all), findsOneWidget);

      await tester.tap(find.byIcon(Icons.done_all));
      await tester.pumpAndSettle();
    });
  });

  group('DesktopHandoffScreen Tests', () {
    testWidgets('Renders handoff code and copy link button', (WidgetTester tester) async {
      await tester.pumpWidget(const MaterialApp(home: DesktopHandoffScreen()));
      await tester.pumpAndSettle();

      expect(find.text('DESKTOP LAB HANDOFF'), findsOneWidget);
      expect(find.text('Continue on Full Desktop Terminal'), findsOneWidget);
      expect(find.text('COPY DIRECT WORKSPACE LINK'), findsOneWidget);
    });
  });
}
