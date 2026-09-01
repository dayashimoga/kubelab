import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:kubelab_mobile/data/curriculum_data.dart';
import 'package:kubelab_mobile/screens/tracks_screen.dart';
import 'package:kubelab_mobile/screens/lesson_screen.dart';
import 'package:kubelab_mobile/screens/progress_sync_screen.dart';
import 'package:kubelab_mobile/screens/notifications_screen.dart';
import 'package:kubelab_mobile/screens/settings_screen.dart';

/// Helper to ensure curriculum is loaded exactly once before all tests.
Future<void> _ensureCurriculumLoaded() async {
  if (CurriculumRepository.tracks.isNotEmpty) return;
  final file = File('assets/data/curriculum.json');
  if (await file.exists()) {
    final jsonStr = await file.readAsString();
    CurriculumRepository.initializeFromJson(jsonStr);
  }
}

void main() {
  setUpAll(() async {
    await _ensureCurriculumLoaded();
  });

  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  // =====================================================================
  // GROUP 1: Data Pipeline & Repository Integrity
  // =====================================================================
  group('Data Pipeline & Repository Integrity', () {
    test('CurriculumRepository.initializeFromJson loads non-empty tracks', () {
      expect(CurriculumRepository.tracks, isNotEmpty,
          reason: 'P0: tracks list must not be empty after initialization');
    });

    test('Authoritative counts: TRACKS=15 MODULES=30 LESSONS=154', () {
      expect(CurriculumRepository.tracks.length, 15);

      final totalModules = CurriculumRepository.tracks
          .expand((t) => t.modules)
          .length;
      expect(totalModules, 30);

      final totalLessons = CurriculumRepository.tracks
          .expand((t) => t.modules)
          .expand((m) => m.lessons)
          .length;
      expect(totalLessons, 154);
    });

    test('Every track has non-empty id, slug, title, description, modules', () {
      for (final track in CurriculumRepository.tracks) {
        expect(track.id, isNotEmpty, reason: 'Track id must not be empty');
        expect(track.slug, isNotEmpty, reason: 'Track slug must not be empty');
        expect(track.title, isNotEmpty, reason: 'Track title must not be empty');
        expect(track.description, isNotEmpty, reason: 'Track description must not be empty');
        expect(track.modules, isNotEmpty,
            reason: 'Track "${track.slug}" must have at least 1 module');
      }
    });

    test('Every lesson has non-empty markdown content (no placeholders)', () {
      for (final track in CurriculumRepository.tracks) {
        for (final mod in track.modules) {
          for (final lesson in mod.lessons) {
            expect(lesson.contentMarkdown.length, greaterThan(200),
                reason: 'Lesson "${lesson.id}" contentMarkdown is too short');
            expect(lesson.contentMarkdown.toLowerCase().contains('coming soon'), false,
                reason: 'Lesson "${lesson.id}" contains placeholder text');
            expect(lesson.contentMarkdown.toLowerCase().contains('todo:'), false,
                reason: 'Lesson "${lesson.id}" contains TODO');
          }
        }
      }
    });

    test('Every lesson has a valid associatedLabId and associatedQuizId', () {
      for (final track in CurriculumRepository.tracks) {
        for (final mod in track.modules) {
          for (final lesson in mod.lessons) {
            expect(lesson.associatedLabId, isNotEmpty,
                reason: 'Lesson "${lesson.id}" missing associatedLabId');
            expect(lesson.associatedQuizId, isNotEmpty,
                reason: 'Lesson "${lesson.id}" missing associatedQuizId');
          }
        }
      }
    });

    test('QUIZZES=154 and every quiz has >=10 questions', () {
      expect(CurriculumRepository.quizzes.length, 154);

      for (final entry in CurriculumRepository.quizzes.entries) {
        expect(entry.value.questions.length, greaterThanOrEqualTo(10),
            reason: 'Quiz "${entry.key}" has fewer than 10 questions');
      }
    });

    test('Every quiz question has valid prompt, 4 options, explanation', () {
      for (final quiz in CurriculumRepository.quizzes.values) {
        for (final q in quiz.questions) {
          expect(q.prompt, isNotEmpty, reason: 'Question "${q.id}" missing prompt');
          expect(q.options.length, 4,
              reason: 'Question "${q.id}" should have exactly 4 options');
          expect(q.explanation, isNotEmpty,
              reason: 'Question "${q.id}" missing explanation');
          expect(q.correctIndex, inInclusiveRange(0, 3),
              reason: 'Question "${q.id}" correctIndex out of range');
        }
      }
    });

    test('Every lesson has a non-empty diagram (Mermaid source)', () {
      int diagramCount = 0;
      for (final track in CurriculumRepository.tracks) {
        for (final mod in track.modules) {
          for (final lesson in mod.lessons) {
            if (lesson.diagram.isNotEmpty) diagramCount++;
          }
        }
      }
      expect(diagramCount, 154,
          reason: 'All 154 lessons must have a diagram');
    });

    test('No duplicate track IDs or slugs', () {
      final ids = <String>{};
      final slugs = <String>{};
      for (final t in CurriculumRepository.tracks) {
        expect(ids.add(t.id), true, reason: 'Duplicate track id: ${t.id}');
        expect(slugs.add(t.slug), true, reason: 'Duplicate track slug: ${t.slug}');
      }
    });

    test('No duplicate lesson IDs', () {
      final ids = <String>{};
      for (final t in CurriculumRepository.tracks) {
        for (final m in t.modules) {
          for (final l in m.lessons) {
            expect(ids.add(l.id), true, reason: 'Duplicate lesson id: ${l.id}');
          }
        }
      }
    });

    test('getTrackBySlug returns correct tracks', () {
      final linux = CurriculumRepository.getTrackBySlug('linux-containers');
      expect(linux, isNotNull);
      expect(linux!.title, contains('Linux'));

      final missing = CurriculumRepository.getTrackBySlug('nonexistent-track');
      expect(missing, isNull);
    });

    test('getQuizForLesson returns valid quiz for each lesson', () {
      for (final track in CurriculumRepository.tracks) {
        for (final mod in track.modules) {
          for (final lesson in mod.lessons) {
            final quiz = CurriculumRepository.getQuizForLesson(lesson.id);
            expect(quiz, isNotNull,
                reason: 'No quiz found for lesson "${lesson.id}" '
                    '(associatedQuizId: ${lesson.associatedQuizId})');
          }
        }
      }
    });
  });

  // =====================================================================
  // GROUP 2: Widget Rendering with Loaded Data
  // =====================================================================
  group('Widget Rendering with Loaded Data', () {
    testWidgets('TracksScreen renders 15 track cards when data is loaded',
        (WidgetTester tester) async {
      await tester.pumpWidget(const MaterialApp(home: TracksScreen()));
      await tester.pumpAndSettle();

      // Header should show "15 Engineering Tracks"
      expect(find.text('15 Engineering Tracks'), findsOneWidget);

      // Should find at least two specific tracks in the list
      expect(find.text('Linux & Container Fundamentals'), findsOneWidget);
      expect(find.text('Kubernetes Core Architecture & Workloads'), findsOneWidget);
    });

    testWidgets('TracksScreen search filters correctly',
        (WidgetTester tester) async {
      await tester.pumpWidget(const MaterialApp(home: TracksScreen()));
      await tester.pumpAndSettle();

      // Type search query
      await tester.enterText(find.byType(TextField), 'Istio');
      await tester.pumpAndSettle();

      // Should show Service Mesh track
      expect(find.text('Service Mesh with Istio & Envoy Proxy'), findsOneWidget);
      // Should NOT show Linux track
      expect(find.text('Linux & Container Fundamentals'), findsNothing);
    });

    testWidgets('TracksScreen empty search shows all tracks (data pipeline proof)',
        (WidgetTester tester) async {
      await tester.pumpWidget(const MaterialApp(home: TracksScreen()));
      await tester.pumpAndSettle();

      // Data pipeline assertion: all 15 tracks are in the repository
      expect(CurriculumRepository.tracks.length, 15);

      // UI assertion: at least one Card is rendered (ListView.builder virtualizes)
      expect(find.byType(Card), findsWidgets);

      // The count badge shows correct counts
      expect(find.textContaining('15T'), findsOneWidget);
    });

    testWidgets('TracksScreen difficulty filter works',
        (WidgetTester tester) async {
      await tester.pumpWidget(const MaterialApp(home: TracksScreen()));
      await tester.pumpAndSettle();

      // Tap "EXPERT" filter chip
      await tester.tap(find.text('EXPERT'));
      await tester.pumpAndSettle();

      // Should show expert tracks only
      final cardFinder = find.byType(Card);
      final expertCount = CurriculumRepository.tracks
          .where((t) => t.difficulty.toLowerCase() == 'expert')
          .length;
      expect(cardFinder, findsNWidgets(expertCount));
    });

    testWidgets('Progress Sync Screen Renders Cleanly',
        (WidgetTester tester) async {
      await tester.pumpWidget(const MaterialApp(home: ProgressSyncScreen()));
      await tester.pumpAndSettle();
      expect(find.text('CLOUD PROGRESS SYNC'), findsOneWidget);
    });

    testWidgets('Notifications Screen Renders Cleanly',
        (WidgetTester tester) async {
      await tester.pumpWidget(const MaterialApp(home: NotificationsScreen()));
      await tester.pumpAndSettle();
      expect(find.text('NOTIFICATIONS & ALERTS'), findsOneWidget);
    });

    testWidgets('Settings Screen Renders Cleanly',
        (WidgetTester tester) async {
      await tester.pumpWidget(const MaterialApp(home: SettingsScreen()));
      await tester.pumpAndSettle();
      expect(find.text('SETTINGS & PREFERENCES'), findsOneWidget);
    });

    testWidgets('Lesson Screen Renders with actual lesson data',
        (WidgetTester tester) async {
      final lesson = CurriculumRepository.tracks[0].modules[0].lessons[0];
      await tester.pumpWidget(MaterialApp(home: LessonScreen(lesson: lesson)));
      await tester.pumpAndSettle();

      expect(find.text(lesson.title), findsWidgets);
      expect(find.text('LAUNCH LAB'), findsOneWidget);
    });
  });

  // =====================================================================
  // GROUP 3: Empty State Guard (no data scenario)
  // =====================================================================
  group('Empty State Guard', () {
    testWidgets('TracksScreen shows error when repository is empty',
        (WidgetTester tester) async {
      // Temporarily clear tracks
      final originalTracks = CurriculumRepository.tracks;
      final originalQuizzes = CurriculumRepository.quizzes;
      CurriculumRepository.tracks = [];
      CurriculumRepository.quizzes = {};

      await tester.pumpWidget(const MaterialApp(home: TracksScreen()));
      await tester.pumpAndSettle();

      expect(find.text('No Tracks Available'), findsOneWidget);

      // Restore
      CurriculumRepository.tracks = originalTracks;
      CurriculumRepository.quizzes = originalQuizzes;
    });
  });
}
