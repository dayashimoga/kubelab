import 'package:flutter/material.dart';
import '../services/api_service.dart';
import '../services/progress_service.dart';
import '../data/curriculum_data.dart';

class QuizQuestion {
  final String id;
  final String prompt;
  final List<String> options;
  final int correctIndex;
  final String explanation;

  const QuizQuestion({
    required this.id,
    required this.prompt,
    required this.options,
    required this.correctIndex,
    required this.explanation,
  });
}

class QuizScreen extends StatefulWidget {
  final MobileLessonQuiz? lessonQuiz;
  final String trackTitle;
  final String? lessonId;
  final int? lessonXp;
  final List<QuizQuestion>? questions;
  final ApiService? apiService;

  const QuizScreen({
    super.key,
    this.lessonQuiz,
    this.trackTitle = 'Kubernetes Core',
    this.lessonId,
    this.lessonXp,
    this.questions,
    this.apiService,
  });

  @override
  State<QuizScreen> createState() => _QuizScreenState();
}

class _QuizScreenState extends State<QuizScreen> {
  int _currentIndex = 0;
  int? _selectedOption;
  bool _submitted = false;
  int _score = 0;
  bool _finished = false;

  late final ApiService _apiService;
  late final List<QuizQuestion> _questions;
  late final String _quizId;
  late final String _quizTitle;

  @override
  void initState() {
    super.initState();
    _apiService = widget.apiService ?? ApiService();

    if (widget.lessonQuiz != null) {
      _quizId = widget.lessonQuiz!.id;
      _quizTitle = widget.lessonQuiz!.title;
      _questions = widget.lessonQuiz!.questions.map((q) {
        return QuizQuestion(
          id: q.id,
          prompt: q.prompt,
          options: q.options,
          correctIndex: q.correctIndex,
          explanation: q.explanation,
        );
      }).toList();
    } else if (widget.questions != null && widget.questions!.isNotEmpty) {
      _quizId = 'custom-quiz';
      _quizTitle = 'QUIZ: ${widget.trackTitle.toUpperCase()}';
      _questions = widget.questions!;
    } else {
      // Default to first quiz in repository
      final firstQuiz = CurriculumRepository.quizzes.values.first;
      _quizId = firstQuiz.id;
      _quizTitle = firstQuiz.title;
      _questions = firstQuiz.questions.map((q) {
        return QuizQuestion(
          id: q.id,
          prompt: q.prompt,
          options: q.options,
          correctIndex: q.correctIndex,
          explanation: q.explanation,
        );
      }).toList();
    }
  }

  void _handleOptionSelect(int index) {
    if (_submitted) return;
    setState(() {
      _selectedOption = index;
    });
  }

  Future<void> _handleSubmitAnswer() async {
    if (_selectedOption == null) return;

    final q = _questions[_currentIndex];
    final isCorrect = _selectedOption == q.correctIndex;

    setState(() {
      _submitted = true;
      if (isCorrect) _score += 100;
    });

    // Queue action for cloud sync
    try {
      await _apiService.queueOfflineAction('quiz_submission', {
        'quizId': _quizId,
        'questionId': q.id,
        'selected': _selectedOption,
        'correct': isCorrect,
        'points': isCorrect ? 100 : 0,
      });
    } catch (_) {}
  }

  Future<void> _handleNext() async {
    if (_currentIndex < _questions.length - 1) {
      setState(() {
        _currentIndex++;
        _selectedOption = null;
        _submitted = false;
      });
    } else {
      // Quiz completed! Save progress
      final maxScore = _questions.length * 100;
      final xpEarned = widget.lessonXp ?? _score;
      await ProgressService.instance.saveQuizResult(_quizId, _score, maxScore, xpEarned);

      if (widget.lessonId != null) {
        await ProgressService.instance.markLessonCompleted(widget.lessonId!, xpEarned);
      }

      setState(() {
        _finished = true;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0A0E17),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0F172A),
        title: Text(
          _quizTitle.toUpperCase(),
          style: const TextStyle(
            fontSize: 13,
            fontWeight: FontWeight.bold,
            letterSpacing: 1.1,
            color: Color(0xFF06B6D4),
          ),
          overflow: TextOverflow.ellipsis,
        ),
      ),
      body: _finished ? _buildSummary() : _buildQuestion(),
    );
  }

  Widget _buildQuestion() {
    final q = _questions[_currentIndex];
    return Padding(
      padding: const EdgeInsets.all(20.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(
                'Question ${_currentIndex + 1} of ${_questions.length}',
                style: const TextStyle(color: Color(0xFF64748B), fontWeight: FontWeight.bold),
              ),
              Text(
                'Score: $_score XP',
                style: const TextStyle(color: Color(0xFF10B981), fontWeight: FontWeight.bold),
              ),
            ],
          ),
          const SizedBox(height: 12),
          ClipRRect(
            borderRadius: BorderRadius.circular(4),
            child: LinearProgressIndicator(
              value: (_currentIndex + 1) / _questions.length,
              backgroundColor: const Color(0xFF1E293B),
              valueColor: const AlwaysStoppedAnimation(Color(0xFF06B6D4)),
              minHeight: 6,
            ),
          ),
          const SizedBox(height: 24),
          Text(
            q.prompt,
            style: const TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.w600,
              color: Colors.white,
              height: 1.4,
            ),
          ),
          const SizedBox(height: 24),
          Expanded(
            child: ListView.separated(
              itemCount: q.options.length,
              separatorBuilder: (_, __) => const SizedBox(height: 12),
              itemBuilder: (context, index) {
                final isSelected = _selectedOption == index;
                Color borderCol = const Color(0xFF1E293B);
                Color bgCol = const Color(0xFF0F172A);

                if (_submitted) {
                  if (index == q.correctIndex) {
                    borderCol = const Color(0xFF10B981);
                    bgCol = const Color(0x2610B981);
                  } else if (isSelected) {
                    borderCol = const Color(0xFFEF4444);
                    bgCol = const Color(0x26EF4444);
                  }
                } else if (isSelected) {
                  borderCol = const Color(0xFF06B6D4);
                  bgCol = const Color(0x1A06B6D4);
                }

                return InkWell(
                  onTap: () => _handleOptionSelect(index),
                  borderRadius: BorderRadius.circular(8),
                  child: Container(
                    padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
                    decoration: BoxDecoration(
                      color: bgCol,
                      borderRadius: BorderRadius.circular(8),
                      border: Border.all(color: borderCol, width: isSelected || _submitted ? 2 : 1),
                    ),
                    child: Row(
                      children: [
                        Text(
                          String.fromCharCode(65 + index),
                          style: TextStyle(
                            fontWeight: FontWeight.bold,
                            color: isSelected ? const Color(0xFF06B6D4) : const Color(0xFF64748B),
                          ),
                        ),
                        const SizedBox(width: 12),
                        Expanded(
                          child: Text(
                            q.options[index],
                            style: const TextStyle(color: Colors.white, fontSize: 13, height: 1.3),
                          ),
                        ),
                      ],
                    ),
                  ),
                );
              },
            ),
          ),
          if (_submitted) ...[
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: const Color(0xFF0F172A),
                borderRadius: BorderRadius.circular(8),
                border: Border.all(
                  color: _selectedOption == q.correctIndex
                      ? const Color(0xFF10B981)
                      : const Color(0xFFEF4444),
                ),
              ),
              child: Text(
                q.explanation,
                style: const TextStyle(color: Color(0xFFCBD5E1), fontSize: 13, height: 1.3),
              ),
            ),
            const SizedBox(height: 12),
          ],
          ElevatedButton(
            onPressed: _selectedOption == null
                ? null
                : (_submitted ? _handleNext : _handleSubmitAnswer),
            style: ElevatedButton.styleFrom(
              backgroundColor: const Color(0xFF06B6D4),
              foregroundColor: const Color(0xFF0A0E17),
              padding: const EdgeInsets.symmetric(vertical: 14),
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
            ),
            child: Text(
              _submitted
                  ? (_currentIndex < _questions.length - 1 ? 'NEXT QUESTION' : 'VIEW RESULTS')
                  : 'SUBMIT ANSWER',
              style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 14),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSummary() {
    final maxScore = _questions.length * 100;
    final percentage = (maxScore > 0) ? ((_score / maxScore) * 100).toInt() : 100;
    final isPassed = percentage >= 66;

    return Center(
      child: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              isPassed ? Icons.stars : Icons.refresh_rounded,
              size: 80,
              color: isPassed ? const Color(0xFF10B981) : const Color(0xFFF59E0B),
            ),
            const SizedBox(height: 16),
            Text(
              isPassed ? 'Mastery Achieved!' : 'Review & Retry',
              style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
            ),
            const SizedBox(height: 8),
            Text(
              'Scored $_score / $maxScore XP ($percentage%)',
              style: const TextStyle(fontSize: 16, color: Color(0xFF06B6D4), fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 24),
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                OutlinedButton(
                  onPressed: () {
                    setState(() {
                      _currentIndex = 0;
                      _selectedOption = null;
                      _submitted = false;
                      _score = 0;
                      _finished = false;
                    });
                  },
                  style: OutlinedButton.styleFrom(
                    foregroundColor: const Color(0xFF06B6D4),
                    side: const BorderSide(color: Color(0xFF06B6D4)),
                    padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                  ),
                  child: const Text('RETRY QUIZ', style: TextStyle(fontWeight: FontWeight.bold)),
                ),
                const SizedBox(width: 12),
                ElevatedButton(
                  onPressed: () => Navigator.of(context).pop(),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFF06B6D4),
                    foregroundColor: const Color(0xFF0A0E17),
                    padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                  ),
                  child: const Text('CONTINUE', style: TextStyle(fontWeight: FontWeight.bold)),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
