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
  final String category;

  const QuizQuestion({
    required this.id,
    required this.prompt,
    required this.options,
    required this.correctIndex,
    required this.explanation,
    this.category = 'concept',
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
  bool _quickMode = false; // 5 questions vs full 10 bank

  late final ApiService _apiService;
  late final List<QuizQuestion> _allQuestions;
  late List<QuizQuestion> _activeQuestions;
  late final String _quizId;
  late final String _quizTitle;

  @override
  void initState() {
    super.initState();
    _apiService = widget.apiService ?? ApiService();

    if (widget.lessonQuiz != null) {
      _quizId = widget.lessonQuiz!.id;
      _quizTitle = widget.lessonQuiz!.title;
      _allQuestions = widget.lessonQuiz!.questions.map((q) {
        return QuizQuestion(
          id: q.id,
          prompt: q.prompt,
          options: q.options,
          correctIndex: q.correctIndex,
          explanation: q.explanation,
          category: q.category,
        );
      }).toList();
    } else if (widget.questions != null && widget.questions!.isNotEmpty) {
      _quizId = 'custom-quiz';
      _quizTitle = 'QUIZ: ${widget.trackTitle.toUpperCase()}';
      _allQuestions = widget.questions!;
    } else {
      final firstQuiz = CurriculumRepository.quizzes.values.first;
      _quizId = firstQuiz.id;
      _quizTitle = firstQuiz.title;
      _allQuestions = firstQuiz.questions.map((q) {
        return QuizQuestion(
          id: q.id,
          prompt: q.prompt,
          options: q.options,
          correctIndex: q.correctIndex,
          explanation: q.explanation,
          category: q.category,
        );
      }).toList();
    }

    _activeQuestions = List.from(_allQuestions);
  }

  void _toggleQuizMode(bool quick) {
    setState(() {
      _quickMode = quick;
      _currentIndex = 0;
      _selectedOption = null;
      _submitted = false;
      _score = 0;
      _finished = false;

      if (_quickMode && _allQuestions.length > 5) {
        _activeQuestions = _allQuestions.take(5).toList();
      } else {
        _activeQuestions = List.from(_allQuestions);
      }
    });
  }

  void _handleOptionSelect(int index) {
    if (_submitted) return;
    setState(() {
      _selectedOption = index;
    });
  }

  Future<void> _handleSubmitAnswer() async {
    if (_selectedOption == null) return;

    final q = _activeQuestions[_currentIndex];
    final isCorrect = _selectedOption == q.correctIndex;

    setState(() {
      _submitted = true;
      if (isCorrect) _score += 10;
    });

    try {
      await _apiService.queueOfflineAction('quiz_submission', {
        'quizId': _quizId,
        'questionId': q.id,
        'selected': _selectedOption,
        'correct': isCorrect,
        'points': isCorrect ? 10 : 0,
      });
    } catch (_) {}
  }

  Future<void> _handleNext() async {
    if (_currentIndex < _activeQuestions.length - 1) {
      setState(() {
        _currentIndex++;
        _selectedOption = null;
        _submitted = false;
      });
    } else {
      final maxScore = _activeQuestions.length * 10;
      final percentage = (_score / maxScore * 100).round();
      final xpEarned = (percentage >= 80) ? (widget.lessonXp ?? 150) : ((percentage * (widget.lessonXp ?? 150)) ~/ 100);

      await ProgressService.instance.saveQuizResult(_quizId, _score, maxScore, xpEarned);

      if (widget.lessonId != null && percentage >= 80) {
        await ProgressService.instance.markLessonCompleted(widget.lessonId!, xpEarned);
      }

      setState(() {
        _finished = true;
      });
    }
  }

  void _handleRestart() {
    setState(() {
      _currentIndex = 0;
      _selectedOption = null;
      _submitted = false;
      _score = 0;
      _finished = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    if (_finished) {
      return _buildFinishedView();
    }

    final q = _activeQuestions[_currentIndex];
    final progress = (_currentIndex + 1) / _activeQuestions.length;

    return Scaffold(
      backgroundColor: const Color(0xFF0A0E17),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0F172A),
        title: Text(
          _quizTitle,
          style: const TextStyle(fontSize: 13, fontWeight: FontWeight.bold, color: Colors.white),
          overflow: TextOverflow.ellipsis,
        ),
        actions: [
          Container(
            margin: const EdgeInsets.only(right: 12),
            child: Row(
              children: [
                ActionChip(
                  label: Text(_quickMode ? 'Quick (5 Qs)' : 'Bank (${_allQuestions.length} Qs)'),
                  labelStyle: const TextStyle(fontSize: 10, fontWeight: FontWeight.bold, color: Color(0xFF06B6D4)),
                  backgroundColor: const Color(0xFF1E293B),
                  side: const BorderSide(color: Color(0xFF334155)),
                  padding: EdgeInsets.zero,
                  onPressed: () => _toggleQuizMode(!_quickMode),
                ),
              ],
            ),
          ),
        ],
      ),
      body: SafeArea(
        child: Column(
          children: [
            // Linear Progress Bar
            LinearProgressIndicator(
              value: progress,
              backgroundColor: const Color(0xFF1E293B),
              valueColor: const AlwaysStoppedAnimation<Color>(Color(0xFF06B6D4)),
              minHeight: 4,
            ),

            Expanded(
              child: SingleChildScrollView(
                padding: const EdgeInsets.all(18),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    // Header metadata
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text(
                          'QUESTION ${_currentIndex + 1} OF ${_activeQuestions.length}',
                          style: const TextStyle(
                            fontSize: 11,
                            fontWeight: FontWeight.bold,
                            color: Color(0xFF38BDF8),
                            letterSpacing: 1.2,
                          ),
                        ),
                        Container(
                          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                          decoration: BoxDecoration(
                            color: const Color(0x336366F1),
                            borderRadius: BorderRadius.circular(6),
                          ),
                          child: Text(
                            q.category.toUpperCase(),
                            style: const TextStyle(
                              fontSize: 9,
                              fontWeight: FontWeight.bold,
                              color: Color(0xFFA5B4FC),
                              letterSpacing: 1.0,
                            ),
                          ),
                        ),
                      ],
                    ),
                    const SizedBox(height: 14),

                    // Prompt Card
                    Container(
                      width: double.infinity,
                      padding: const EdgeInsets.all(16),
                      decoration: BoxDecoration(
                        color: const Color(0xFF0F172A),
                        borderRadius: BorderRadius.circular(14),
                        border: Border.all(color: const Color(0xFF1E293B)),
                      ),
                      child: Text(
                        q.prompt,
                        style: const TextStyle(
                          fontSize: 15,
                          fontWeight: FontWeight.w600,
                          color: Colors.white,
                          height: 1.4,
                        ),
                      ),
                    ),
                    const SizedBox(height: 18),

                    // Options List
                    for (int i = 0; i < q.options.length; i++) ...[
                      _buildOptionTile(i, q.options[i], q.correctIndex),
                      const SizedBox(height: 10),
                    ],

                    // Detailed Explanation Reveal
                    if (_submitted) ...[
                      const SizedBox(height: 14),
                      Container(
                        width: double.infinity,
                        padding: const EdgeInsets.all(14),
                        decoration: BoxDecoration(
                          color: const Color(0xFF0F172A),
                          borderRadius: BorderRadius.circular(12),
                          border: Border.all(
                            color: _selectedOption == q.correctIndex
                                ? const Color(0x4D10B981)
                                : const Color(0x4DEF4444),
                          ),
                        ),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Row(
                              children: [
                                Icon(
                                  _selectedOption == q.correctIndex
                                      ? Icons.check_circle
                                      : Icons.cancel,
                                  color: _selectedOption == q.correctIndex
                                      ? const Color(0xFF10B981)
                                      : const Color(0xFFEF4444),
                                  size: 16,
                                ),
                                const SizedBox(width: 6),
                                Text(
                                  _selectedOption == q.correctIndex
                                      ? 'CORRECT ANSWER'
                                      : 'EXPLANATION',
                                  style: TextStyle(
                                    fontSize: 11,
                                    fontWeight: FontWeight.bold,
                                    color: _selectedOption == q.correctIndex
                                        ? const Color(0xFF10B981)
                                        : const Color(0xFFEF4444),
                                    letterSpacing: 1.0,
                                  ),
                                ),
                              ],
                            ),
                            const SizedBox(height: 6),
                            Text(
                              q.explanation,
                              style: const TextStyle(
                                fontSize: 12,
                                color: Color(0xFFCBD5E1),
                                height: 1.4,
                              ),
                            ),
                          ],
                        ),
                      ),
                    ],
                  ],
                ),
              ),
            ),

            // Bottom Action Bar
            Container(
              padding: const EdgeInsets.all(16),
              decoration: const BoxDecoration(
                color: Color(0xFF0F172A),
                border: Border(top: BorderSide(color: Color(0xFF1E293B))),
              ),
              child: SizedBox(
                width: double.infinity,
                height: 48,
                child: ElevatedButton(
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFF06B6D4),
                    foregroundColor: const Color(0xFF0A0E17),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                  ),
                  onPressed: _selectedOption == null
                      ? null
                      : (_submitted ? _handleNext : _handleSubmitAnswer),
                  child: Text(
                    _submitted
                        ? (_currentIndex == _activeQuestions.length - 1
                            ? 'VIEW RESULTS'
                            : 'NEXT QUESTION')
                        : 'SUBMIT ANSWER',
                    style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 13, letterSpacing: 0.5),
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildOptionTile(int index, String text, int correctIndex) {
    Color borderColor = const Color(0xFF1E293B);
    Color bgColor = const Color(0xFF0F172A);
    Color textColor = const Color(0xFFCBD5E1);
    IconData? trailingIcon;
    Color? iconColor;

    if (_submitted) {
      if (index == correctIndex) {
        borderColor = const Color(0xFF10B981);
        bgColor = const Color(0x2610B981);
        textColor = Colors.white;
        trailingIcon = Icons.check_circle;
        iconColor = const Color(0xFF10B981);
      } else if (_selectedOption == index) {
        borderColor = const Color(0xFFEF4444);
        bgColor = const Color(0x26EF4444);
        textColor = const Color(0xFFFCA5A5);
        trailingIcon = Icons.cancel;
        iconColor = const Color(0xFFEF4444);
      }
    } else if (_selectedOption == index) {
      borderColor = const Color(0xFF06B6D4);
      bgColor = const Color(0x2606B6D4);
      textColor = Colors.white;
    }

    return InkWell(
      onTap: () => _handleOptionSelect(index),
      borderRadius: BorderRadius.circular(12),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 150),
        padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 12),
        decoration: BoxDecoration(
          color: bgColor,
          borderRadius: BorderRadius.circular(12),
          border: Border.all(color: borderColor, width: 1.5),
        ),
        child: Row(
          children: [
            Container(
              width: 24,
              height: 24,
              decoration: BoxDecoration(
                shape: BoxShape.circle,
                color: _selectedOption == index
                    ? (_submitted && index != correctIndex ? const Color(0xFFEF4444) : const Color(0xFF06B6D4))
                    : const Color(0xFF1E293B),
              ),
              child: Center(
                child: Text(
                  String.fromCharCode(65 + index),
                  style: TextStyle(
                    fontSize: 11,
                    fontWeight: FontWeight.bold,
                    color: _selectedOption == index ? const Color(0xFF0A0E17) : const Color(0xFF94A3B8),
                  ),
                ),
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: Text(
                text,
                style: TextStyle(fontSize: 13, color: textColor, height: 1.3),
              ),
            ),
            if (trailingIcon != null) ...[
              const SizedBox(width: 8),
              Icon(trailingIcon, color: iconColor, size: 18),
            ],
          ],
        ),
      ),
    );
  }

  Widget _buildFinishedView() {
    final maxScore = _activeQuestions.length * 10;
    final percentage = (_score / maxScore * 100).round();
    final passed = percentage >= 80;
    final xpEarned = passed ? (widget.lessonXp ?? 150) : ((percentage * (widget.lessonXp ?? 150)) ~/ 100);

    return Scaffold(
      backgroundColor: const Color(0xFF0A0E17),
      body: SafeArea(
        child: Center(
          child: SingleChildScrollView(
            padding: const EdgeInsets.all(24),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Icon(
                  passed ? Icons.emoji_events : Icons.replay_circle_filled_outlined,
                  size: 64,
                  color: passed ? const Color(0xFFFBBF24) : const Color(0xFF94A3B8),
                ),
                const SizedBox(height: 16),
                Text(
                  passed ? 'ASSESSMENT PASSED!' : 'KEEP PRACTICING',
                  style: const TextStyle(
                    fontSize: 20,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                    letterSpacing: 1.0,
                  ),
                ),
                const SizedBox(height: 8),
                Text(
                  passed
                      ? 'You have demonstrated competency in this topic.'
                      : 'Review the lesson concepts and try again to earn full XP.',
                  style: const TextStyle(fontSize: 13, color: Color(0xFF94A3B8)),
                  textAlign: TextAlign.center,
                ),
                const SizedBox(height: 24),

                // Score card
                Container(
                  padding: const EdgeInsets.all(20),
                  decoration: BoxDecoration(
                    color: const Color(0xFF0F172A),
                    borderRadius: BorderRadius.circular(16),
                    border: Border.all(color: passed ? const Color(0xFF10B981) : const Color(0xFF334155)),
                  ),
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.spaceAround,
                    children: [
                      Column(
                        children: [
                          Text(
                            '$percentage%',
                            style: TextStyle(
                              fontSize: 26,
                              fontWeight: FontWeight.bold,
                              color: passed ? const Color(0xFF10B981) : const Color(0xFFF59E0B),
                            ),
                          ),
                          const SizedBox(height: 4),
                          const Text('ACCURACY', style: TextStyle(fontSize: 10, color: Color(0xFF94A3B8), fontWeight: FontWeight.bold)),
                        ],
                      ),
                      Container(height: 36, width: 1, color: const Color(0xFF334155)),
                      Column(
                        children: [
                          Text(
                            '+$xpEarned',
                            style: const TextStyle(
                              fontSize: 26,
                              fontWeight: FontWeight.bold,
                              color: Color(0xFF06B6D4),
                            ),
                          ),
                          const SizedBox(height: 4),
                          const Text('XP EARNED', style: TextStyle(fontSize: 10, color: Color(0xFF94A3B8), fontWeight: FontWeight.bold)),
                        ],
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 28),

                ElevatedButton.icon(
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFF06B6D4),
                    foregroundColor: const Color(0xFF0A0E17),
                    minimumSize: const Size.fromHeight(48),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                  ),
                  icon: const Icon(Icons.arrow_back),
                  label: const Text('BACK TO LESSON', style: TextStyle(fontWeight: FontWeight.bold)),
                  onPressed: () => Navigator.of(context).pop(),
                ),
                const SizedBox(height: 12),
                OutlinedButton.icon(
                  style: OutlinedButton.styleFrom(
                    foregroundColor: const Color(0xFF38BDF8),
                    side: const BorderSide(color: Color(0xFF0284C7)),
                    minimumSize: const Size.fromHeight(44),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                  ),
                  icon: const Icon(Icons.refresh),
                  label: const Text('RETAKE QUIZ', style: TextStyle(fontWeight: FontWeight.bold)),
                  onPressed: _handleRestart,
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
