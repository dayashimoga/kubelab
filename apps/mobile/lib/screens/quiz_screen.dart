import 'package:flutter/material.dart';
import '../services/api_service.dart';

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
  final String trackTitle;
  final List<QuizQuestion> questions;
  final ApiService? apiService;

  const QuizScreen({
    super.key,
    this.trackTitle = 'Kubernetes Core',
    this.questions = _defaultQuestions,
    this.apiService,
  });

  static const List<QuizQuestion> _defaultQuestions = [
    QuizQuestion(
      id: 'q1',
      prompt: 'Which Pod Security Standard profile strictly forbids running privileged containers and hostPath mounts?',
      options: [
        'Privileged',
        'Baseline',
        'Restricted',
        'Default',
      ],
      correctIndex: 2,
      explanation: 'The Restricted profile enforces pod hardening best practices, disallowing privileged containers, host namespaces, and hostPath volumes.',
    ),
    QuizQuestion(
      id: 'q2',
      prompt: 'What Kubernetes resource provides declarative, zero-trust traffic filtering between pods?',
      options: [
        'Ingress',
        'NetworkPolicy',
        'ServiceAccount',
        'LimitRange',
      ],
      correctIndex: 1,
      explanation: 'NetworkPolicy resources let you specify how groups of pods are allowed to communicate with each other and other network endpoints.',
    ),
    QuizQuestion(
      id: 'q3',
      prompt: 'In GitOps with Argo CD, what happens when a cluster resource is modified out-of-band and selfHeal is enabled?',
      options: [
        'The cluster change is committed back to Git',
        'Argo CD halts all sync operations',
        'Argo CD automatically overwrites the mutation with Git desired state',
        'The application status turns permanently Error',
      ],
      correctIndex: 2,
      explanation: 'With self-heal enabled, Argo CD detects live state drift and automatically reverts the cluster to match the version-controlled Git manifest.',
    ),
  ];

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

  @override
  void initState() {
    super.initState();
    _apiService = widget.apiService ?? ApiService();
  }

  void _handleOptionSelect(int index) {
    if (_submitted) return;
    setState(() {
      _selectedOption = index;
    });
  }

  Future<void> _handleSubmitAnswer() async {
    if (_selectedOption == null) return;

    final q = widget.questions[_currentIndex];
    final isCorrect = _selectedOption == q.correctIndex;

    setState(() {
      _submitted = true;
      if (isCorrect) _score += 100;
    });

    // Queue action for cloud sync
    try {
      await _apiService.queueOfflineAction('quiz_submission', {
        'questionId': q.id,
        'selected': _selectedOption,
        'correct': isCorrect,
        'points': isCorrect ? 100 : 0,
      });
    } catch (_) {}
  }

  void _handleNext() {
    if (_currentIndex < widget.questions.length - 1) {
      setState(() {
        _currentIndex++;
        _selectedOption = null;
        _submitted = false;
      });
    } else {
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
          'QUIZ: ${widget.trackTitle.toUpperCase()}',
          style: const TextStyle(
            fontSize: 14,
            fontWeight: FontWeight.bold,
            letterSpacing: 1.1,
            color: Color(0xFF06B6D4),
          ),
        ),
      ),
      body: _finished ? _buildSummary() : _buildQuestion(),
    );
  }

  Widget _buildQuestion() {
    final q = widget.questions[_currentIndex];
    return Padding(
      padding: const EdgeInsets.all(20.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(
                'Question ${_currentIndex + 1} of ${widget.questions.length}',
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
              value: (_currentIndex + 1) / widget.questions.length,
              backgroundColor: const Color(0xFF1E293B),
              valueColor: const AnimatedStoppedAnimation(Color(0xFF06B6D4)),
              minHeight: 6,
            ),
          ),
          const SizedBox(height: 24),
          Text(
            q.prompt,
            style: const TextStyle(
              fontSize: 17,
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
                    bgCol = const Color(0xFF10B981).withOpacity(0.15);
                  } else if (isSelected) {
                    borderCol = const Color(0xFFEF4444);
                    bgCol = const Color(0xFFEF4444).withOpacity(0.15);
                  }
                } else if (isSelected) {
                  borderCol = const Color(0xFF06B6D4);
                  bgCol = const Color(0xFF06B6D4).withOpacity(0.1);
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
                            style: const TextStyle(color: Colors.white, fontSize: 14),
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
                style: const TextStyle(color: Color(0xFFCBD5E1), fontSize: 13),
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
                  ? (_currentIndex < widget.questions.length - 1 ? 'NEXT QUESTION' : 'VIEW RESULTS')
                  : 'SUBMIT ANSWER',
              style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 14),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSummary() {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.stars, size: 80, color: Color(0xFF10B981)),
            const SizedBox(height: 16),
            const Text(
              'Quiz Completed!',
              style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
            ),
            const SizedBox(height: 8),
            Text(
              'Earned +$_score XP',
              style: const TextStyle(fontSize: 18, color: Color(0xFF06B6D4), fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 24),
            ElevatedButton(
              onPressed: () => Navigator.of(context).pop(),
              style: ElevatedButton.styleFrom(
                backgroundColor: const Color(0xFF06B6D4),
                foregroundColor: const Color(0xFF0A0E17),
                padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 14),
                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
              ),
              child: const Text('BACK TO CURRICULUM', style: TextStyle(fontWeight: FontWeight.bold)),
            ),
          ],
        ),
      ),
    );
  }
}
