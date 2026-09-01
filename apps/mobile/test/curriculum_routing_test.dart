import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:kubelab_mobile/data/curriculum_data.dart';
import 'package:kubelab_mobile/screens/tracks_screen.dart';
import 'package:kubelab_mobile/screens/modules_screen.dart';
import 'package:kubelab_mobile/screens/lesson_screen.dart';
import 'package:kubelab_mobile/screens/quiz_screen.dart';
import 'package:kubelab_mobile/screens/ai_tutor_sheet.dart';

void main() {
  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  group('Curriculum Routing & Referential Integrity Tests', () {
    test('Curriculum repository contains exactly 15 tracks and 154 lessons', () {
      expect(CurriculumRepository.tracks.length, 15);
      final totalLessons = CurriculumRepository.tracks
          .expand((t) => t.modules)
          .expand((m) => m.lessons)
          .length;
      expect(totalLessons, 154);
      expect(CurriculumRepository.quizzes.length, 154);
    });

    testWidgets('TracksScreen renders 15 engineering tracks header and cards', (WidgetTester tester) async {
      await tester.pumpWidget(const MaterialApp(home: TracksScreen()));
      await tester.pumpAndSettle();

      expect(find.text('15 ENGINEERING TRACKS'), findsOneWidget);
      expect(find.text('Linux & Container Fundamentals'), findsOneWidget);
      expect(find.text('Kubernetes Core Architecture & Workloads'), findsOneWidget);
    });

    testWidgets('Navigating to different tracks opens unique modules and lessons', (WidgetTester tester) async {
      // 1. Test Linux track
      final linuxTrack = CurriculumRepository.getTrackBySlug('linux-containers')!;
      await tester.pumpWidget(MaterialApp(home: ModulesScreen(track: linuxTrack)));
      await tester.pumpAndSettle();

      expect(find.text('LINUX & CONTAINER FUNDAMENTALS'), findsOneWidget);
      expect(find.text('Linux Filesystem Hierarchy & POSIX Permissions'), findsOneWidget);

      // 2. Test Security track
      final secTrack = CurriculumRepository.getTrackBySlug('security')!;
      await tester.pumpWidget(MaterialApp(home: ModulesScreen(track: secTrack)));
      await tester.pumpAndSettle();

      expect(find.text('ZERO-TRUST KUBERNETES SECURITY & RBAC'), findsOneWidget);
      expect(find.text('Restricting Pod Access with RBAC Roles and RoleBindings'), findsOneWidget);

      // 3. Test Service Mesh track
      final meshTrack = CurriculumRepository.getTrackBySlug('service-mesh')!;
      await tester.pumpWidget(MaterialApp(home: ModulesScreen(track: meshTrack)));
      await tester.pumpAndSettle();

      expect(find.text('SERVICE MESH WITH ISTIO & ENVOY PROXY'), findsOneWidget);
      expect(find.text('Deploying Istio Service Mesh Operator'), findsOneWidget);
    });

    testWidgets('LessonScreen displays unique markdown content, warnings, and lab handoff', (WidgetTester tester) async {
      final lesson = CurriculumRepository.tracks[0].modules[0].lessons[0];
      await tester.pumpWidget(MaterialApp(home: LessonScreen(lesson: lesson)));
      await tester.pumpAndSettle();

      expect(find.text(lesson.title), findsWidgets);
      expect(find.text('CONTINUE ON DESKTOP'), findsOneWidget);
      expect(find.text('COMMON PRODUCTION MISTAKES'), findsOneWidget);
      expect(find.text('PRODUCTION HARDENING & SECURITY'), findsOneWidget);
    });

    testWidgets('QuizScreen loads unique lesson questions and evaluates correctly', (WidgetTester tester) async {
      final quiz = CurriculumRepository.quizzes.values.first;
      await tester.pumpWidget(MaterialApp(home: QuizScreen(lessonQuiz: quiz)));
      await tester.pumpAndSettle();

      expect(find.text(quiz.title.toUpperCase()), findsOneWidget);
      expect(find.text(quiz.questions[0].prompt), findsOneWidget);

      // Tap first option (which is correct for generated quizzes)
      await tester.tap(find.text(quiz.questions[0].options[0]));
      await tester.pumpAndSettle();

      await tester.tap(find.text('SUBMIT ANSWER'));
      await tester.pumpAndSettle();

      expect(find.text('Score: 100 XP'), findsOneWidget);
    });

    testWidgets('AI Tutor sheet renders mode chips and responds', (WidgetTester tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Builder(
              builder: (ctx) => ElevatedButton(
                onPressed: () => AiTutorSheet.show(ctx, topicTitle: 'Kubernetes Pods'),
                child: const Text('OPEN TUTOR'),
              ),
            ),
          ),
        ),
      );

      await tester.tap(find.text('OPEN TUTOR'));
      await tester.pumpAndSettle();

      expect(find.text('AI SOCRATIC TUTOR'), findsOneWidget);
      expect(find.text('EXPLAIN'), findsOneWidget);
      expect(find.text('SOCRATIC'), findsOneWidget);
      expect(find.text('HINT'), findsOneWidget);
      expect(find.text('DIAGNOSE'), findsOneWidget);
      expect(find.text('REVIEW'), findsOneWidget);
    });
  });
}
