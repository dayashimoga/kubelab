import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

class DesktopHandoffScreen extends StatefulWidget {
  final String? labId;
  final String? labTitle;

  const DesktopHandoffScreen({super.key, this.labId = 'k8s-pod-basics', this.labTitle = 'Pod Fundamentals'});

  @override
  State<DesktopHandoffScreen> createState() => _DesktopHandoffScreenState();
}

class _DesktopHandoffScreenState extends State<DesktopHandoffScreen> {
  late final String _handoffCode;
  late final String _handoffUrl;

  @override
  void initState() {
    super.initState();
    final randomSuffix = (1000 + DateTime.now().millisecond % 9000).toString();
    _handoffCode = 'KL-${widget.labId?.toUpperCase()}-$randomSuffix';
    _handoffUrl = 'https://kubelab.io/handoff/$_handoffCode';
  }

  void _copyToClipboard(String text, String label) {
    Clipboard.setData(ClipboardData(text: text));
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text('$label copied to clipboard!')),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0A0E17),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0F172A),
        title: const Text(
          'DESKTOP LAB HANDOFF',
          style: TextStyle(
            fontSize: 15,
            fontWeight: FontWeight.bold,
            letterSpacing: 1.1,
            color: Color(0xFF06B6D4),
          ),
        ),
      ),
      body: Padding(
        padding: const EdgeInsets.all(20.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Container(
              padding: const EdgeInsets.all(20),
              decoration: BoxDecoration(
                color: const Color(0xFF0F172A),
                borderRadius: BorderRadius.circular(12),
                border: Border.all(color: const Color(0xFF1E293B)),
              ),
              child: Column(
                children: [
                  const Icon(Icons.laptop_chromebook, size: 64, color: Color(0xFF06B6D4)),
                  const SizedBox(height: 12),
                  const Text(
                    'Continue on Full Desktop Terminal',
                    textAlign: TextAlign.center,
                    style: TextStyle(color: Colors.white, fontSize: 16, fontWeight: FontWeight.bold),
                  ),
                  const SizedBox(height: 6),
                  Text(
                    'Active Lab: ${widget.labTitle}',
                    textAlign: TextAlign.center,
                    style: const TextStyle(color: Color(0xFF94A3B8), fontSize: 13),
                  ),
                  const SizedBox(height: 20),
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                    decoration: BoxDecoration(
                      color: const Color(0xFF0A0E17),
                      borderRadius: BorderRadius.circular(8),
                      border: Border.all(color: const Color(0xFF06B6D4)),
                    ),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Text(
                          _handoffCode,
                          style: const TextStyle(
                            color: Color(0xFF06B6D4),
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                            letterSpacing: 2.0,
                            fontFamily: 'monospace',
                          ),
                        ),
                        const SizedBox(width: 12),
                        IconButton(
                          icon: const Icon(Icons.copy, size: 20, color: Color(0xFF06B6D4)),
                          onPressed: () => _copyToClipboard(_handoffCode, 'Handoff Code'),
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 24),
            ElevatedButton.icon(
              onPressed: () => _copyToClipboard(_handoffUrl, 'Desktop Handoff URL'),
              icon: const Icon(Icons.link),
              label: const Text('COPY DIRECT WORKSPACE LINK'),
              style: ElevatedButton.styleFrom(
                backgroundColor: const Color(0xFF06B6D4),
                foregroundColor: const Color(0xFF0A0E17),
                padding: const EdgeInsets.symmetric(vertical: 14),
                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
              ),
            ),
            const SizedBox(height: 32),
            const Text(
              'HOW DESKTOP HANDOFF WORKS',
              style: TextStyle(color: Color(0xFF64748B), fontSize: 12, fontWeight: FontWeight.bold, letterSpacing: 1.1),
            ),
            const SizedBox(height: 12),
            _stepItem('1', 'Open KubeLab on your desktop browser (Chrome/Firefox).'),
            const SizedBox(height: 8),
            _stepItem('2', 'Navigate to Practice Workspace or paste the handoff URL.'),
            const SizedBox(height: 8),
            _stepItem('3', 'Your sandbox terminal, Monaco YAML editor, and progress sync instantly.'),
          ],
        ),
      ),
    );
  }

  Widget _stepItem(String step, String text) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Container(
          width: 24,
          height: 24,
          alignment: Alignment.center,
          decoration: const BoxDecoration(
            color: Color(0xFF1E293B),
            shape: BoxShape.circle,
          ),
          child: Text(step, style: const TextStyle(color: Color(0xFF06B6D4), fontWeight: FontWeight.bold, fontSize: 12)),
        ),
        const SizedBox(width: 12),
        Expanded(
          child: Text(text, style: const TextStyle(color: Color(0xFF94A3B8), fontSize: 13)),
        ),
      ],
    );
  }
}
