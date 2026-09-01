import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;

enum TutorMode { explain, socratic, hint, diagnose, review }

class TutorMessage {
  final String sender; // 'user' | 'tutor'
  final String text;
  final TutorMode mode;
  final DateTime timestamp;

  TutorMessage({
    required this.sender,
    required this.text,
    required this.mode,
    DateTime? timestamp,
  }) : timestamp = timestamp ?? DateTime.now();
}

class AiTutorSheet extends StatefulWidget {
  final String topicTitle;
  final String? codeSnippet;
  final String? labId;

  const AiTutorSheet({
    super.key,
    required this.topicTitle,
    this.codeSnippet,
    this.labId,
  });

  static Future<void> show(BuildContext context, {
    required String topicTitle,
    String? codeSnippet,
    String? labId,
  }) {
    return showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (_) => AiTutorSheet(
        topicTitle: topicTitle,
        codeSnippet: codeSnippet,
        labId: labId,
      ),
    );
  }

  @override
  State<AiTutorSheet> createState() => _AiTutorSheetState();
}

class _AiTutorSheetState extends State<AiTutorSheet> {
  TutorMode _selectedMode = TutorMode.explain;
  final TextEditingController _inputController = TextEditingController();
  final ScrollController _scrollController = ScrollController();
  final List<TutorMessage> _messages = [];
  bool _isLoading = false;
  bool _isBackendOnline = false;

  @override
  void initState() {
    super.initState();
    _checkBackendStatus();
    _messages.add(
      TutorMessage(
        sender: 'tutor',
        text: 'Hello! I am your KubeLab AI Socratic Tutor for **${widget.topicTitle}**.\n\nSelect a mode below or ask me any question about architecture, YAML manifests, or debugging!',
        mode: TutorMode.explain,
      ),
    );
  }

  Future<void> _checkBackendStatus() async {
    try {
      final response = await http.get(Uri.parse('http://10.0.2.2:8080/health')).timeout(const Duration(seconds: 2));
      if (mounted) {
        setState(() {
          _isBackendOnline = response.statusCode == 200;
        });
      }
    } catch (_) {
      if (mounted) {
        setState(() {
          _isBackendOnline = false;
        });
      }
    }
  }

  Future<void> _sendMessage(String text) async {
    if (text.trim().isEmpty) return;

    final userText = text.trim();
    _inputController.clear();

    setState(() {
      _messages.add(TutorMessage(sender: 'user', text: userText, mode: _selectedMode));
      _isLoading = true;
    });
    _scrollToBottom();

    // Call real API or local pedagogical heuristic engine
    String reply = '';
    try {
      if (_isBackendOnline) {
        final resp = await http.post(
          Uri.parse('http://10.0.2.2:8080/v1/ai-tutor/chat'),
          headers: {'Content-Type': 'application/json'},
          body: jsonEncode({
            'mode': _selectedMode.name,
            'topic': widget.topicTitle,
            'prompt': userText,
            'code_snippet': widget.codeSnippet,
            'lab_id': widget.labId,
          }),
        ).timeout(const Duration(seconds: 8));

        if (resp.statusCode == 200) {
          final data = jsonDecode(resp.body);
          reply = data['reply'] ?? data['message'] ?? '';
        }
      }
    } catch (_) {}

    if (reply.isEmpty) {
      // Local Socratic & Heuristic responses
      reply = _generateLocalTutorResponse(_selectedMode, userText, widget.topicTitle);
    }

    if (mounted) {
      setState(() {
        _isLoading = false;
        _messages.add(TutorMessage(sender: 'tutor', text: reply, mode: _selectedMode));
      });
      _scrollToBottom();
    }
  }

  String _generateLocalTutorResponse(TutorMode mode, String query, String topic) {
    switch (mode) {
      case TutorMode.explain:
        return '### Deep Conceptual Breakdown: $topic\n\nIn Kubernetes architecture, **$topic** coordinates desired state via the reconciliation loop. When you apply declarations, controllers continuously reconcile actual cluster state until it converges.\n\n**Key Rule**: Always specify resource requests and decouple configuration from container images using ConfigMaps/Secrets.';
      case TutorMode.socratic:
        return '### Socratic Inquiry\n\nTo understand how to approach "$query":\n\n1. What Kubernetes controller manages this lifecycle?\n2. What would happen to in-flight requests if the container receives SIGTERM without a readiness probe?\n\n*Think about how kube-proxy updates endpoint tables.*';
      case TutorMode.hint:
        return '### Progressive Hint\n\nInspect the live object status using:\n```bash\nkubectl describe pod <name> -n <namespace>\nkubectl get events --sort-by=.metadata.creationTimestamp\n```\nLook closely at the `Events:` section for probe failures or scheduling rejections.';
      case TutorMode.diagnose:
        return '### Diagnostic Analysis\n\nCommon failure patterns for "$topic":\n- **OOMKilled (137)**: Memory limits exceeded.\n- **CrashLoopBackOff**: Application error on startup.\n- **Pending**: Insufficient node CPU/memory or taints/tolerations mismatch.\n- **503 Service Unavailable**: No healthy ready pod endpoints matching selector.';
      case TutorMode.review:
        return '### Architectural Review & Best Practices\n\n- **Security**: Run as non-root, drop `ALL` capabilities, add only needed ones.\n- **Reliability**: Configure `PodDisruptionBudgets` (PDB) to survive cluster upgrades.\n- **Observability**: Expose Prometheus `/metrics` and structured JSON logs.';
    }
  }

  void _scrollToBottom() {
    Future.delayed(const Duration(milliseconds: 100), () {
      if (_scrollController.hasClients) {
        _scrollController.animateTo(
          _scrollController.position.maxScrollExtent,
          duration: const Duration(milliseconds: 250),
          curve: Curves.easeOut,
        );
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      height: MediaQuery.of(context).size.height * 0.85,
      decoration: const BoxDecoration(
        color: Color(0xFF0A0E17),
        borderRadius: BorderRadius.vertical(top: Radius.circular(24)),
        border: Border(top: BorderSide(color: Color(0xFF06B6D4), width: 1.5)),
      ),
      child: Column(
        children: [
          // Header handle
          Container(
            margin: const EdgeInsets.only(top: 12),
            width: 40,
            height: 4,
            decoration: BoxDecoration(
              color: const Color(0xFF334155),
              borderRadius: BorderRadius.circular(2),
            ),
          ),

          // Title bar
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
            child: Row(
              children: [
                Container(
                  padding: const EdgeInsets.all(8),
                  decoration: BoxDecoration(
                    color: const Color(0x2606B6D4),
                    borderRadius: BorderRadius.circular(10),
                    border: Border.all(color: const Color(0x4D06B6D4)),
                  ),
                  child: const Icon(Icons.psychology, color: Color(0xFF06B6D4), size: 20),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text(
                        'AI SOCRATIC TUTOR',
                        style: TextStyle(
                          fontSize: 14,
                          fontWeight: FontWeight.bold,
                          letterSpacing: 1.1,
                          color: Colors.white,
                        ),
                      ),
                      Text(
                        widget.topicTitle,
                        style: const TextStyle(fontSize: 12, color: Color(0xFF94A3B8)),
                        overflow: TextOverflow.ellipsis,
                      ),
                    ],
                  ),
                ),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                  decoration: BoxDecoration(
                    color: _isBackendOnline ? const Color(0x2610B981) : const Color(0x26F59E0B),
                    borderRadius: BorderRadius.circular(6),
                    border: Border.all(
                      color: _isBackendOnline ? const Color(0xFF10B981) : const Color(0xFFF59E0B),
                    ),
                  ),
                  child: Text(
                    _isBackendOnline ? 'LIVE AI' : 'LOCAL TUTOR',
                    style: TextStyle(
                      fontSize: 10,
                      fontWeight: FontWeight.bold,
                      color: _isBackendOnline ? const Color(0xFF10B981) : const Color(0xFFF59E0B),
                    ),
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.close, color: Color(0xFF64748B)),
                  onPressed: () => Navigator.of(context).pop(),
                ),
              ],
            ),
          ),

          // Mode Selector
          SingleChildScrollView(
            scrollDirection: Axis.horizontal,
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 4),
            child: Row(
              children: TutorMode.values.map((mode) {
                final isSelected = _selectedMode == mode;
                return Padding(
                  padding: const EdgeInsets.only(right: 8),
                  child: ChoiceChip(
                    label: Text(
                      mode.name.toUpperCase(),
                      style: TextStyle(
                        fontSize: 11,
                        fontWeight: FontWeight.bold,
                        color: isSelected ? Colors.black : const Color(0xFF94A3B8),
                      ),
                    ),
                    selected: isSelected,
                    selectedColor: const Color(0xFF06B6D4),
                    backgroundColor: const Color(0xFF0F172A),
                    onSelected: (selected) {
                      if (selected) {
                        setState(() => _selectedMode = mode);
                      }
                    },
                  ),
                );
              }).toList(),
            ),
          ),

          const Divider(color: Color(0xFF1E293B), height: 16),

          // Chat Message List
          Expanded(
            child: ListView.builder(
              controller: _scrollController,
              padding: const EdgeInsets.all(16),
              itemCount: _messages.length,
              itemBuilder: (context, index) {
                final msg = _messages[index];
                final isUser = msg.sender == 'user';
                return Align(
                  alignment: isUser ? Alignment.centerRight : Alignment.centerLeft,
                  child: Container(
                    margin: const EdgeInsets.only(bottom: 12),
                    constraints: BoxConstraints(maxWidth: MediaQuery.of(context).size.width * 0.82),
                    padding: const EdgeInsets.all(14),
                    decoration: BoxDecoration(
                      color: isUser ? const Color(0xFF06B6D4) : const Color(0xFF0F172A),
                      borderRadius: BorderRadius.circular(14),
                      border: Border.all(
                        color: isUser ? const Color(0xFF06B6D4) : const Color(0xFF1E293B),
                      ),
                    ),
                    child: Text(
                      msg.text,
                      style: TextStyle(
                        color: isUser ? Colors.black : const Color(0xFFE2E8F0),
                        fontSize: 13,
                        height: 1.4,
                      ),
                    ),
                  ),
                );
              },
            ),
          ),

          if (_isLoading)
            const Padding(
              padding: EdgeInsets.all(8.0),
              child: SizedBox(
                width: 20,
                height: 20,
                child: CircularProgressIndicator(strokeWidth: 2, color: Color(0xFF06B6D4)),
              ),
            ),

          // Input Bar
          Container(
            padding: EdgeInsets.only(
              left: 16,
              right: 16,
              top: 10,
              bottom: MediaQuery.of(context).viewInsets.bottom + 16,
            ),
            decoration: const BoxDecoration(
              color: Color(0xFF0F172A),
              border: Border(top: BorderSide(color: Color(0xFF1E293B))),
            ),
            child: Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: _inputController,
                    style: const TextStyle(color: Colors.white, fontSize: 13),
                    decoration: InputDecoration(
                      hintText: 'Ask ${_selectedMode.name} question...',
                      hintStyle: const TextStyle(color: Color(0xFF64748B)),
                      filled: true,
                      fillColor: const Color(0xFF0A0E17),
                      contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                      border: OutlineInputBorder(
                        borderRadius: BorderRadius.circular(10),
                        borderSide: const BorderSide(color: Color(0xFF1E293B)),
                      ),
                      focusedBorder: OutlineInputBorder(
                        borderRadius: BorderRadius.circular(10),
                        borderSide: const BorderSide(color: Color(0xFF06B6D4)),
                      ),
                    ),
                    onSubmitted: _sendMessage,
                  ),
                ),
                const SizedBox(width: 8),
                IconButton(
                  icon: const Icon(Icons.send_rounded, color: Color(0xFF06B6D4)),
                  onPressed: () => _sendMessage(_inputController.text),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
