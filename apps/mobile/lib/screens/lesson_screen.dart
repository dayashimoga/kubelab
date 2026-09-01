import 'package:flutter/material.dart';
import 'desktop_handoff_screen.dart';
import 'quiz_screen.dart';

class LessonScreen extends StatelessWidget {
  const LessonScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Understanding Pods'),
        backgroundColor: const Color(0xFF0F172A),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
              decoration: BoxDecoration(
                color: const Color(0x336366F1), // 20% opacity indigo
                borderRadius: BorderRadius.circular(6),
              ),
              child: const Text(
                'MODULE 1 • KUBERNETES CORE',
                style: TextStyle(
                  color: Color(0xFFA5B4FC),
                  fontSize: 10,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ),
            const SizedBox(height: 12),
            const Text(
              'The Atomic Unit of Kubernetes',
              style: TextStyle(
                fontSize: 22,
                fontWeight: FontWeight.bold,
                color: Colors.white,
              ),
            ),
            const SizedBox(height: 16),
            const Text(
              'In Kubernetes, containers never run in isolation. A Pod encapsulates one or more application containers, shared Linux namespaces, and a single routable IP address held by the pause container.',
              style: TextStyle(
                fontSize: 14,
                color: Color(0xFFCBD5E1),
                height: 1.6,
              ),
            ),
            const SizedBox(height: 24),
            // Continue on Desktop Callout
            InkWell(
              onTap: () {
                Navigator.of(context).push(
                  MaterialPageRoute(
                    builder: (_) => const DesktopHandoffScreen(
                      labId: 'k8s-pod-basics',
                      labTitle: 'Pod Fundamentals (k8s-pod-basics)',
                    ),
                  ),
                );
              },
              borderRadius: BorderRadius.circular(12),
              child: Container(
                padding: const EdgeInsets.all(16),
                decoration: BoxDecoration(
                  color: const Color(0x1A06B6D4), // 10% opacity cyan
                  borderRadius: BorderRadius.circular(12),
                  border: Border.all(color: const Color(0x4D06B6D4)), // 30% opacity cyan
                ),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: const [
                    Row(
                      children: [
                        Icon(Icons.desktop_windows, color: Color(0xFF06B6D4)),
                        SizedBox(width: 8),
                        Text(
                          'CONTINUE ON DESKTOP',
                          style: TextStyle(
                            color: Color(0xFF06B6D4),
                            fontWeight: FontWeight.bold,
                            fontSize: 12,
                          ),
                        ),
                      ],
                    ),
                    SizedBox(height: 8),
                    Text(
                      'This lesson includes a live interactive terminal lab: "k8s-pod-basics". Tap here to generate handoff link for your desktop browser.',
                      style: TextStyle(fontSize: 12, color: Color(0xFF94A3B8)),
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 20),
            ElevatedButton.icon(
              onPressed: () {
                Navigator.of(context).push(
                  MaterialPageRoute(
                    builder: (_) => const QuizScreen(trackTitle: 'Kubernetes Core'),
                  ),
                );
              },
              icon: const Icon(Icons.quiz, color: Colors.black),
              label: const Text('TAKE LESSON QUIZ (+300 XP)', style: TextStyle(fontWeight: FontWeight.bold)),
              style: ElevatedButton.styleFrom(
                backgroundColor: const Color(0xFF06B6D4),
                foregroundColor: const Color(0xFF0A0E17),
                minimumSize: const Size(double.infinity, 48),
                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
