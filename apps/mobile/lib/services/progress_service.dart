import 'dart:convert';
import 'package:shared_preferences/shared_preferences.dart';
import '../data/curriculum_data.dart';

class ProgressService {
  static const String _completedLessonsKey = 'kubelab_completed_lessons';
  static const String _completedQuizzesKey = 'kubelab_completed_quizzes';
  static const String _quizScoresKey = 'kubelab_quiz_scores';
  static const String _totalXpKey = 'kubelab_total_xp';
  static const String _streakKey = 'kubelab_streak_days';

  static ProgressService? _instance;
  ProgressService._();

  static ProgressService get instance => _instance ??= ProgressService._();

  Future<Set<String>> getCompletedLessons() async {
    final prefs = await SharedPreferences.getInstance();
    final list = prefs.getStringList(_completedLessonsKey) ?? [];
    return list.toSet();
  }

  Future<void> markLessonCompleted(String lessonId, int xp) async {
    final prefs = await SharedPreferences.getInstance();
    final list = prefs.getStringList(_completedLessonsKey) ?? [];
    if (!list.contains(lessonId)) {
      list.add(lessonId);
      await prefs.setStringList(_completedLessonsKey, list);
      await addXp(xp);
    }
  }

  Future<bool> isLessonCompleted(String lessonId) async {
    final completed = await getCompletedLessons();
    return completed.contains(lessonId);
  }

  Future<Set<String>> getCompletedQuizzes() async {
    final prefs = await SharedPreferences.getInstance();
    final list = prefs.getStringList(_completedQuizzesKey) ?? [];
    return list.toSet();
  }

  Future<void> saveQuizResult(String quizId, int score, int maxScore, int xpEarned) async {
    final prefs = await SharedPreferences.getInstance();
    final list = prefs.getStringList(_completedQuizzesKey) ?? [];
    if (!list.contains(quizId)) {
      list.add(quizId);
      await prefs.setStringList(_completedQuizzesKey, list);
    }

    final rawScores = prefs.getString(_quizScoresKey);
    Map<String, dynamic> scores = {};
    if (rawScores != null) {
      try {
        scores = jsonDecode(rawScores) as Map<String, dynamic>;
      } catch (_) {}
    }
    scores[quizId] = {
      'score': score,
      'maxScore': maxScore,
      'xp': xpEarned,
      'timestamp': DateTime.now().toIso8601String(),
    };
    await prefs.setString(_quizScoresKey, jsonEncode(scores));
    await addXp(xpEarned);
  }

  Future<int> getTotalXp() async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.getInt(_totalXpKey) ?? 0;
  }

  Future<void> addXp(int xp) async {
    if (xp <= 0) return;
    final prefs = await SharedPreferences.getInstance();
    final current = prefs.getInt(_totalXpKey) ?? 0;
    await prefs.setInt(_totalXpKey, current + xp);
  }

  Future<double> getTrackProgressPercentage(String trackSlug) async {
    final track = CurriculumRepository.getTrackBySlug(trackSlug);
    if (track == null || track.totalLessons == 0) return 0.0;

    final completed = await getCompletedLessons();
    int count = 0;
    for (final module in track.modules) {
      for (final lesson in module.lessons) {
        if (completed.contains(lesson.id)) {
          count++;
        }
      }
    }
    return count / track.totalLessons;
  }

  Future<int> getLearnerLevel() async {
    final xp = await getTotalXp();
    if (xp < 500) return 1;
    if (xp < 1500) return 2;
    if (xp < 3500) return 3;
    if (xp < 7000) return 4;
    if (xp < 12000) return 5;
    if (xp < 20000) return 6;
    if (xp < 30000) return 7;
    return 8;
  }
}
