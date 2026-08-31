import 'package:flutter/material.dart';

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
                color: const Color(0xFF6366F1).withOpacity(0.2),
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
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: const Color(0xFF06B6D4).withOpacity(0.1),
                borderRadius: BorderRadius.circular(12),
                border: Border.all(color: const Color(0xFF06B6D4).withOpacity(0.3)),
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
                    'This lesson includes a live interactive terminal lab: "k8s-pod-basics". Open desktop browser to access full xterm.js sandbox.',
                    style: TextStyle(fontSize: 12, color: Color(0xFF94A3B8)),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
