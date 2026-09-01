import 'package:flutter/material.dart';
import 'package:flutter_markdown/flutter_markdown.dart';
import '../data/curriculum_data.dart';
import '../services/progress_service.dart';
import '../components/mermaid_renderer.dart';
import 'quiz_screen.dart';
import 'lab_workspace_screen.dart';
import 'desktop_handoff_screen.dart';
import 'ai_tutor_sheet.dart';

class LessonScreen extends StatefulWidget {
  final MobileLesson? lesson;
  final MobileTrack? track;

  const LessonScreen({
    super.key,
    this.lesson,
    this.track,
  });

  @override
  State<LessonScreen> createState() => _LessonScreenState();
}

class _LessonScreenState extends State<LessonScreen> {
  late final MobileLesson _lesson;
  late final MobileTrack _track;
  bool _isCompleted = false;

  @override
  void initState() {
    super.initState();
    if (widget.lesson != null) {
      _lesson = widget.lesson!;
    } else {
      _lesson = CurriculumRepository.tracks.first.modules.first.lessons.first;
    }

    if (widget.track != null) {
      _track = widget.track!;
    } else {
      _track = CurriculumRepository.getTrackBySlug(_lesson.trackSlug) ?? CurriculumRepository.tracks.first;
    }

    _checkCompletion();
  }

  Future<void> _checkCompletion() async {
    final completed = await ProgressService.instance.isLessonCompleted(_lesson.id);
    if (mounted) {
      setState(() {
        _isCompleted = completed;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final quiz = CurriculumRepository.getQuizForLesson(_lesson.id) ??
        CurriculumRepository.getQuizById(_lesson.associatedQuizId);

    return Scaffold(
      backgroundColor: const Color(0xFF0A0E17),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0F172A),
        title: Text(
          _lesson.title,
          style: const TextStyle(fontSize: 14, fontWeight: FontWeight.bold, color: Colors.white),
          overflow: TextOverflow.ellipsis,
        ),
        actions: [
          IconButton(
            icon: const Icon(Icons.psychology, color: Color(0xFF06B6D4)),
            tooltip: 'AI Socratic Tutor',
            onPressed: () {
              AiTutorSheet.show(
                context,
                topicTitle: _lesson.title,
                labId: _lesson.associatedLabId,
              );
            },
          ),
          IconButton(
            icon: const Icon(Icons.desktop_windows_outlined, color: Color(0xFF94A3B8)),
            tooltip: 'Desktop Handoff',
            onPressed: () {
              Navigator.of(context).push(
                MaterialPageRoute(
                  builder: (_) => DesktopHandoffScreen(
                    labId: _lesson.associatedLabId,
                    labTitle: _lesson.title,
                  ),
                ),
              );
            },
          ),
        ],
      ),
      body: SafeArea(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(18),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Breadcrumbs & Module Header
              Row(
                children: [
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                    decoration: BoxDecoration(
                      color: const Color(0x336366F1),
                      borderRadius: BorderRadius.circular(6),
                    ),
                    child: Text(
                      _track.title.toUpperCase(),
                      style: const TextStyle(
                        color: Color(0xFFA5B4FC),
                        fontSize: 10,
                        fontWeight: FontWeight.bold,
                        letterSpacing: 1.1,
                      ),
                    ),
                  ),
                  const SizedBox(width: 8),
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                    decoration: BoxDecoration(
                      color: const Color(0x2610B981),
                      borderRadius: BorderRadius.circular(6),
                    ),
                    child: Text(
                      '+${_lesson.xp} XP',
                      style: const TextStyle(
                        color: Color(0xFF10B981),
                        fontSize: 10,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
                  const Spacer(),
                  if (_isCompleted)
                    Container(
                      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                      decoration: BoxDecoration(
                        color: const Color(0x2610B981),
                        borderRadius: BorderRadius.circular(6),
                      ),
                      child: const Row(
                        children: [
                          Icon(Icons.check_circle, color: Color(0xFF10B981), size: 12),
                          SizedBox(width: 4),
                          Text(
                            'COMPLETED',
                            style: TextStyle(
                              color: Color(0xFF10B981),
                              fontSize: 10,
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                        ],
                      ),
                    ),
                ],
              ),
              const SizedBox(height: 12),

              // Lesson Title
              Text(
                _lesson.title,
                style: const TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  color: Colors.white,
                  height: 1.3,
                ),
              ),
              const SizedBox(height: 8),

              // Lesson Summary
              Text(
                _lesson.summary,
                style: const TextStyle(
                  fontSize: 13,
                  color: Color(0xFF94A3B8),
                  height: 1.5,
                ),
              ),
              const SizedBox(height: 16),

              // Quick Action Bar: Launch Lab & Take Quiz
              Row(
                children: [
                  Expanded(
                    child: ElevatedButton.icon(
                      style: ElevatedButton.styleFrom(
                        backgroundColor: const Color(0xFF06B6D4),
                        foregroundColor: const Color(0xFF0A0E17),
                        padding: const EdgeInsets.symmetric(vertical: 12),
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
                      ),
                      icon: const Icon(Icons.terminal, size: 18),
                      label: const Text('LAUNCH LAB', style: TextStyle(fontWeight: FontWeight.bold, fontSize: 12)),
                      onPressed: () {
                        Navigator.of(context).push(
                          MaterialPageRoute(
                            builder: (_) => LabWorkspaceScreen(
                              labId: _lesson.associatedLabId,
                              labTitle: _lesson.title,
                              trackSlug: _track.slug,
                            ),
                          ),
                        );
                      },
                    ),
                  ),
                  const SizedBox(width: 10),
                  Expanded(
                    child: OutlinedButton.icon(
                      style: OutlinedButton.styleFrom(
                        foregroundColor: const Color(0xFF38BDF8),
                        side: const BorderSide(color: Color(0xFF0284C7)),
                        padding: const EdgeInsets.symmetric(vertical: 12),
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
                      ),
                      icon: const Icon(Icons.quiz_outlined, size: 18),
                      label: const Text('TAKE QUIZ', style: TextStyle(fontWeight: FontWeight.bold, fontSize: 12)),
                      onPressed: () {
                        Navigator.of(context).push(
                          MaterialPageRoute(
                            builder: (_) => QuizScreen(
                              lessonQuiz: quiz,
                              trackTitle: _track.title,
                              lessonId: _lesson.id,
                              lessonXp: _lesson.xp,
                            ),
                          ),
                        ).then((_) => _checkCompletion());
                      },
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 20),

              // Architecture Diagram Section with MermaidRenderer
              if (_lesson.diagram.isNotEmpty) ...[
                MermaidRenderer(
                  diagramCode: _lesson.diagram,
                  title: 'Architecture & Process Flow: ${_lesson.title}',
                  isDark: true,
                ),
              ],

              // Markdown Body Content
              MarkdownBody(
                data: _lesson.contentMarkdown,
                selectable: true,
                styleSheet: MarkdownStyleSheet(
                  h1: const TextStyle(color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold),
                  h2: const TextStyle(color: Color(0xFF06B6D4), fontSize: 15, fontWeight: FontWeight.bold),
                  h3: const TextStyle(color: Colors.white, fontSize: 13, fontWeight: FontWeight.bold),
                  p: const TextStyle(color: Color(0xFFCBD5E1), fontSize: 13, height: 1.6),
                  code: const TextStyle(
                    color: Color(0xFF67E8F9),
                    backgroundColor: Color(0xFF0F172A),
                    fontFamily: 'monospace',
                    fontSize: 12,
                  ),
                  codeblockDecoration: BoxDecoration(
                    color: const Color(0xFF0F172A),
                    borderRadius: BorderRadius.circular(10),
                    border: Border.all(color: const Color(0xFF1E293B)),
                  ),
                  tableHead: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 12),
                  tableBody: const TextStyle(color: Color(0xFFCBD5E1), fontSize: 12),
                  tableBorder: TableBorder.all(color: const Color(0xFF1E293B)),
                  listBullet: const TextStyle(color: Color(0xFF06B6D4)),
                ),
              ),
              const SizedBox(height: 20),

              // Common Mistakes Callout Card
              if (_lesson.commonMistakes.isNotEmpty) ...[
                Container(
                  padding: const EdgeInsets.all(14),
                  decoration: BoxDecoration(
                    color: const Color(0x1AF59E0B),
                    borderRadius: BorderRadius.circular(12),
                    border: Border.all(color: const Color(0x4DF59E0B)),
                  ),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Row(
                        children: [
                          Icon(Icons.warning_amber_rounded, color: Color(0xFFF59E0B), size: 18),
                          SizedBox(width: 8),
                          Text(
                            'COMMON PRODUCTION PITFALLS',
                            style: TextStyle(
                              color: Color(0xFFF59E0B),
                              fontWeight: FontWeight.bold,
                              fontSize: 11,
                              letterSpacing: 1.1,
                            ),
                          ),
                        ],
                      ),
                      const SizedBox(height: 10),
                      for (final mistake in _lesson.commonMistakes) ...[
                        Padding(
                          padding: const EdgeInsets.only(bottom: 6),
                          child: Row(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              const Text('• ', style: TextStyle(color: Color(0xFFF59E0B), fontWeight: FontWeight.bold)),
                              Expanded(
                                child: Text(
                                  mistake,
                                  style: const TextStyle(color: Color(0xFFCBD5E1), fontSize: 12, height: 1.4),
                                ),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ],
                  ),
                ),
                const SizedBox(height: 16),
              ],

              // Production & Security Guidance Card
              if (_lesson.productionGuidance.isNotEmpty) ...[
                Container(
                  padding: const EdgeInsets.all(14),
                  decoration: BoxDecoration(
                    color: const Color(0x1A10B981),
                    borderRadius: BorderRadius.circular(12),
                    border: Border.all(color: const Color(0x4D10B981)),
                  ),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Row(
                        children: [
                          Icon(Icons.shield_outlined, color: Color(0xFF10B981), size: 18),
                          SizedBox(width: 8),
                          Text(
                            'SECURITY & RELIABILITY BEST PRACTICES',
                            style: TextStyle(
                              color: Color(0xFF10B981),
                              fontWeight: FontWeight.bold,
                              fontSize: 11,
                              letterSpacing: 1.1,
                            ),
                          ),
                        ],
                      ),
                      const SizedBox(height: 8),
                      Text(
                        _lesson.productionGuidance,
                        style: const TextStyle(color: Color(0xFFCBD5E1), fontSize: 12, height: 1.4),
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 24),
              ],

              // Bottom Actions
              ElevatedButton.icon(
                style: ElevatedButton.styleFrom(
                  backgroundColor: const Color(0xFF06B6D4),
                  foregroundColor: const Color(0xFF0A0E17),
                  minimumSize: const Size.fromHeight(48),
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                ),
                icon: const Icon(Icons.terminal, size: 20),
                label: const Text('OPEN LIVE LAB WORKSPACE', style: TextStyle(fontWeight: FontWeight.bold, fontSize: 13)),
                onPressed: () {
                  Navigator.of(context).push(
                    MaterialPageRoute(
                      builder: (_) => LabWorkspaceScreen(
                        labId: _lesson.associatedLabId,
                        labTitle: _lesson.title,
                        trackSlug: _track.slug,
                      ),
                    ),
                  );
                },
              ),
              const SizedBox(height: 12),
              OutlinedButton.icon(
                style: OutlinedButton.styleFrom(
                  foregroundColor: const Color(0xFF94A3B8),
                  side: const BorderSide(color: Color(0xFF334155)),
                  minimumSize: const Size.fromHeight(44),
                  shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                ),
                icon: const Icon(Icons.psychology, size: 18, color: Color(0xFF06B6D4)),
                label: const Text('ASK AI SOCRATIC TUTOR', style: TextStyle(fontWeight: FontWeight.bold, fontSize: 12)),
                onPressed: () {
                  AiTutorSheet.show(
                    context,
                    topicTitle: _lesson.title,
                    labId: _lesson.associatedLabId,
                  );
                },
              ),
              const SizedBox(height: 24),
            ],
          ),
        ),
      ),
    );
  }
}
