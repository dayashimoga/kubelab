import 'package:flutter/material.dart';
import '../data/curriculum_data.dart';
import '../services/progress_service.dart';
import 'lesson_screen.dart';

class ModulesScreen extends StatefulWidget {
  final MobileTrack track;

  const ModulesScreen({super.key, required this.track});

  @override
  State<ModulesScreen> createState() => _ModulesScreenState();
}

class _ModulesScreenState extends State<ModulesScreen> {
  Set<String> _completedLessons = {};

  @override
  void initState() {
    super.initState();
    _loadCompletion();
  }

  Future<void> _loadCompletion() async {
    final completed = await ProgressService.instance.getCompletedLessons();
    if (mounted) {
      setState(() {
        _completedLessons = completed;
      });
    }
  }

  Color _parseColor(String hex) {
    try {
      final clean = hex.replaceAll('#', '');
      return Color(int.parse('FF$clean', radix: 16));
    } catch (_) {
      return const Color(0xFF06B6D4);
    }
  }

  @override
  Widget build(BuildContext context) {
    final trackColor = _parseColor(widget.track.colorHex);
    final completedCount = widget.track.modules
        .expand((m) => m.lessons)
        .where((l) => _completedLessons.contains(l.id))
        .length;
    final progress = widget.track.totalLessons > 0 ? completedCount / widget.track.totalLessons : 0.0;

    return Scaffold(
      backgroundColor: const Color(0xFF0A0E17),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0F172A),
        title: Text(
          widget.track.title.toUpperCase(),
          style: const TextStyle(
            fontSize: 14,
            fontWeight: FontWeight.bold,
            letterSpacing: 1.1,
            color: Colors.white,
          ),
        ),
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          // Track Hero Header
          Container(
            padding: const EdgeInsets.all(20),
            decoration: BoxDecoration(
              color: const Color(0xFF0F172A),
              borderRadius: BorderRadius.circular(16),
              border: Border.all(color: trackColor.withValues(alpha: 0.4)),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Container(
                      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                      decoration: BoxDecoration(
                        color: trackColor.withValues(alpha: 0.15),
                        borderRadius: BorderRadius.circular(6),
                        border: Border.all(color: trackColor.withValues(alpha: 0.3)),
                      ),
                      child: Text(
                        widget.track.difficulty.toUpperCase(),
                        style: TextStyle(
                          fontSize: 10,
                          fontWeight: FontWeight.bold,
                          color: trackColor,
                          fontFamily: 'monospace',
                        ),
                      ),
                    ),
                    Text(
                      '$completedCount of ${widget.track.totalLessons} Completed',
                      style: const TextStyle(fontSize: 12, color: Color(0xFF94A3B8), fontWeight: FontWeight.bold),
                    ),
                  ],
                ),
                const SizedBox(height: 12),
                Text(
                  widget.track.title,
                  style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white),
                ),
                const SizedBox(height: 6),
                Text(
                  widget.track.description,
                  style: const TextStyle(fontSize: 13, color: Color(0xFF94A3B8), height: 1.4),
                ),
                const SizedBox(height: 16),
                ClipRRect(
                  borderRadius: BorderRadius.circular(4),
                  child: LinearProgressIndicator(
                    value: progress,
                    backgroundColor: const Color(0xFF1E293B),
                    valueColor: AlwaysStoppedAnimation(trackColor),
                    minHeight: 6,
                  ),
                ),
              ],
            ),
          ),
          const SizedBox(height: 20),

          // Modules & Lessons
          ...widget.track.modules.map((module) {
            return Container(
              margin: const EdgeInsets.only(bottom: 20),
              child: Material(
                color: const Color(0xFF0F172A),
                borderRadius: BorderRadius.circular(16),
                child: Container(
                  decoration: BoxDecoration(
                    borderRadius: BorderRadius.circular(16),
                    border: Border.all(color: const Color(0xFF1E293B)),
                  ),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Padding(
                        padding: const EdgeInsets.all(16),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(
                              'MODULE ${module.order}',
                              style: TextStyle(
                                fontSize: 10,
                                fontWeight: FontWeight.bold,
                                fontFamily: 'monospace',
                                color: trackColor,
                                letterSpacing: 1.2,
                              ),
                            ),
                            const SizedBox(height: 4),
                            Text(
                              module.title,
                              style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold, color: Colors.white),
                            ),
                            const SizedBox(height: 4),
                            Text(
                              module.description,
                              style: const TextStyle(fontSize: 12, color: Color(0xFF64748B)),
                            ),
                          ],
                        ),
                      ),
                      const Divider(color: Color(0xFF1E293B), height: 1),
                      ListView.separated(
                        shrinkWrap: true,
                        physics: const NeverScrollableScrollPhysics(),
                        itemCount: module.lessons.length,
                        separatorBuilder: (_, __) => const Divider(color: Color(0xFF1E293B), height: 1),
                        itemBuilder: (context, idx) {
                          final lesson = module.lessons[idx];
                          final isCompleted = _completedLessons.contains(lesson.id);

                          return ListTile(
                            contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                            leading: Container(
                              width: 36,
                              height: 36,
                              alignment: Alignment.center,
                              decoration: BoxDecoration(
                                color: isCompleted ? const Color(0x2610B981) : const Color(0xFF1E293B),
                                shape: BoxShape.circle,
                                border: Border.all(
                                  color: isCompleted ? const Color(0xFF10B981) : const Color(0xFF334155),
                                ),
                              ),
                              child: isCompleted
                                  ? const Icon(Icons.check, color: Color(0xFF10B981), size: 18)
                                  : Text(
                                      '${lesson.order}',
                                      style: const TextStyle(
                                        fontSize: 12,
                                        fontWeight: FontWeight.bold,
                                        color: Colors.white,
                                        fontFamily: 'monospace',
                                      ),
                                    ),
                            ),
                            title: Text(
                              lesson.title,
                              style: const TextStyle(fontSize: 14, fontWeight: FontWeight.bold, color: Colors.white),
                            ),
                            subtitle: Padding(
                              padding: const EdgeInsets.only(top: 4),
                              child: Row(
                                children: [
                                  Text(
                                    '${lesson.durationMinutes}m • +${lesson.xp} XP',
                                    style: const TextStyle(fontSize: 11, color: Color(0xFF64748B)),
                                  ),
                                  const SizedBox(width: 8),
                                  Container(
                                    padding: const EdgeInsets.symmetric(horizontal: 4, vertical: 1),
                                    decoration: BoxDecoration(
                                      color: const Color(0xFF1E293B),
                                      borderRadius: BorderRadius.circular(4),
                                    ),
                                    child: Text(
                                      lesson.associatedLabId,
                                      style: const TextStyle(
                                        fontSize: 9,
                                        color: Color(0xFF06B6D4),
                                        fontFamily: 'monospace',
                                      ),
                                    ),
                                  ),
                                ],
                              ),
                            ),
                            trailing: const Icon(Icons.chevron_right, color: Color(0xFF64748B), size: 20),
                            onTap: () {
                              Navigator.push(
                                context,
                                MaterialPageRoute(
                                  builder: (_) => LessonScreen(
                                    lesson: lesson,
                                    track: widget.track,
                                  ),
                                ),
                              ).then((_) => _loadCompletion());
                            },
                          );
                        },
                      ),
                    ],
                  ),
                ),
              ),
            );
          }).toList(),
        ],
      ),
    );
  }
}
