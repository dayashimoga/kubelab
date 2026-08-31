import 'package:flutter/material.dart';
import 'lesson_screen.dart';

class TracksScreen extends StatelessWidget {
  const TracksScreen({super.key});

  final List<Map<String, dynamic>> tracks = const [
    {
      'title': 'Linux & Containers',
      'lessons': 15,
      'difficulty': 'Beginner',
      'icon': Icons.terminal,
      'color': Color(0xFF06B6D4),
    },
    {
      'title': 'Kubernetes Core Workloads',
      'lessons': 20,
      'difficulty': 'Beginner',
      'icon': Icons.view_in_ar,
      'color': Color(0xFF6366F1),
    },
    {
      'title': 'Networking & Gateway API',
      'lessons': 14,
      'difficulty': 'Intermediate',
      'icon': Icons.hub,
      'color': Color(0xFF10B981),
    },
    {
      'title': 'Security & RBAC Hardening',
      'lessons': 15,
      'difficulty': 'Intermediate',
      'icon': Icons.security,
      'color': Color(0xFFF43F5E),
    },
    {
      'title': 'GitOps with Argo CD',
      'lessons': 12,
      'difficulty': 'Intermediate',
      'icon': Icons.alt_route,
      'color': Color(0xFFA855F7),
    },
    {
      'title': 'OpenTelemetry & Prometheus',
      'lessons': 15,
      'difficulty': 'Advanced',
      'icon': Icons.insights,
      'color': Color(0xFF06B6D4),
    },
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'KUBELAB TRACKS',
          style: TextStyle(fontWeight: FontWeight.w900, letterSpacing: 1.2),
        ),
        backgroundColor: const Color(0xFF0F172A),
        elevation: 0,
      ),
      body: ListView.builder(
        padding: const EdgeInsets.all(16),
        itemCount: tracks.length,
        itemBuilder: (context, index) {
          final t = tracks[index];
          return Card(
            color: const Color(0xFF0F172A),
            margin: const EdgeInsets.only(bottom: 16),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(16),
              side: const BorderSide(color: Color(0xFF1E293B)),
            ),
            child: InkWell(
              borderRadius: BorderRadius.circular(16),
              onTap: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(builder: (_) => const LessonScreen()),
                );
              },
              child: Padding(
                padding: const EdgeInsets.all(20),
                child: Row(
                  children: [
                    Container(
                      width: 50,
                      height: 50,
                      decoration: BoxDecoration(
                        color: (t['color'] as Color).withOpacity(0.15),
                        borderRadius: BorderRadius.circular(12),
                      ),
                      child: Icon(t['icon'] as IconData, color: t['color'] as Color),
                    ),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            t['title'] as String,
                            style: const TextStyle(
                              fontSize: 16,
                              fontWeight: FontWeight.bold,
                              color: Colors.white,
                            ),
                          ),
                          const SizedBox(height: 4),
                          Text(
                            '${t['lessons']} Lessons • ${t['difficulty']}',
                            style: const TextStyle(
                              fontSize: 12,
                              color: Color(0xFF94A3B8),
                            ),
                          ),
                        ],
                      ),
                    ),
                    const Icon(Icons.chevron_right, color: Color(0xFF64748B)),
                  ],
                ),
              ),
            ),
          );
        },
      ),
    );
  }
}
