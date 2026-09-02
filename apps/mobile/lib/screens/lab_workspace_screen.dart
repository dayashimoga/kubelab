import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import '../services/progress_service.dart';

/// Touch-friendly, fully-featured Mobile Lab Client for Kubernetes Hands-On Practice.
/// Features 7 tabs: Instructions, Terminal, YAML Editor, Resources, Logs/Events, Validation, Hints.
class LabWorkspaceScreen extends StatefulWidget {
  final String labId;
  final String? labTitle;
  final String? trackSlug;

  const LabWorkspaceScreen({
    super.key,
    required this.labId,
    this.labTitle,
    this.trackSlug,
  });

  @override
  State<LabWorkspaceScreen> createState() => _LabWorkspaceScreenState();
}

class _LabWorkspaceScreenState extends State<LabWorkspaceScreen> with SingleTickerProviderStateMixin {
  late TabController _tabController;
  final TextEditingController _yamlController = TextEditingController();
  final TextEditingController _terminalInputController = TextEditingController();
  final ScrollController _terminalScrollController = ScrollController();

  String _sessionId = '00000000-0000-0000-0000-000000000001';
  String _namespace = 'lab-sandbox';
  bool _isSessionStarting = true;
  bool _isBackendConnected = false;

  // Terminal state
  final List<String> _terminalOutput = [];
  
  // Validation state
  bool _isValidating = false;
  Map<String, dynamic>? _validationResult;
  int _revealedHints = 0;

  // Resources state
  List<Map<String, dynamic>> _k8sResources = [];
  List<String> _k8sEvents = [];
  bool _isLoadingResources = false;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 7, vsync: this);
    _namespace = 'lab-${widget.labId}';
    _initDefaultYaml();
    _startLabSession();
  }

  void _initDefaultYaml() {
    _yamlController.text = '''apiVersion: v1
kind: Pod
metadata:
  name: ${widget.labId}
  namespace: $_namespace
  labels:
    app: ${widget.labId}
spec:
  containers:
  - name: app
    image: nginx:alpine
    ports:
    - containerPort: 80
    resources:
      requests:
        cpu: "100m"
        memory: "128Mi"
      limits:
        cpu: "250m"
        memory: "256Mi"
''';
  }

  Future<void> _startLabSession() async {
    setState(() => _isSessionStarting = true);
    _appendTerminal('🚀 Provisioning isolated sandbox for ${widget.labId}...');
    _appendTerminal('📦 Target namespace: $_namespace');

    try {
      final res = await http.post(
        Uri.parse('http://10.0.2.2:8080/v1/labs/start'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({'lab_id': widget.labId}),
      ).timeout(const Duration(seconds: 10));

      if (res.statusCode == 200) {
        final data = jsonDecode(res.body);
        _sessionId = data['id'] ?? _sessionId;
        _namespace = data['namespace'] ?? _namespace;
        _isBackendConnected = true;
        _appendTerminal('✅ Connected to live cluster session: $_sessionId');
        _appendTerminal('🔒 Namespace isolation active: $_namespace');
        if (mounted) {
          setState(() => _isSessionStarting = false);
        }
        _refreshResources();
      } else {
        _isBackendConnected = false;
        _appendTerminal('❌ Failed to provision sandbox (HTTP ${res.statusCode})');
        _appendTerminal('⛔ Lab requires a live Kubernetes cluster. Local simulation is disabled.');
        if (mounted) {
          setState(() => _isSessionStarting = false);
        }
      }
    } catch (e) {
      _isBackendConnected = false;
      _appendTerminal('❌ Cannot reach KubeLab API server.');
      _appendTerminal('⛔ Lab requires a live backend connection. Simulated execution is forbidden.');
      _appendTerminal('   Error: $e');
      _appendTerminal('   Tap the refresh button to retry.');
      if (mounted) {
        setState(() => _isSessionStarting = false);
      }
    }
  }

  void _appendTerminal(String line) {
    setState(() {
      _terminalOutput.add(line);
    });
    Future.delayed(const Duration(milliseconds: 50), () {
      if (_terminalScrollController.hasClients) {
        _terminalScrollController.jumpTo(_terminalScrollController.position.maxScrollExtent);
      }
    });
  }

  void _handleTerminalCommand(String cmd) {
    final trimmed = cmd.trim();
    if (trimmed.isEmpty) return;
    _terminalInputController.clear();
    _appendTerminal('\$ $trimmed');

    if (trimmed == 'clear') {
      setState(() => _terminalOutput.clear());
      return;
    }

    if (!_isBackendConnected) {
      _appendTerminal('⛔ [SECURITY] Terminal commands require a live backend connection.');
      _appendTerminal('   Host-shell fallback is disabled by Zero-Trust policy.');
      return;
    }

    // Send command to backend API for execution in the isolated sandbox
    _executeRemoteCommand(trimmed);
  }

  Future<void> _executeRemoteCommand(String cmd) async {
    try {
      final res = await http.post(
        Uri.parse('http://10.0.2.2:8080/v1/labs/sessions/$_sessionId/exec'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({'command': cmd, 'namespace': _namespace}),
      ).timeout(const Duration(seconds: 10));

      if (res.statusCode == 200) {
        final data = jsonDecode(res.body);
        final output = data['output'] as String? ?? '';
        if (output.isNotEmpty) {
          _appendTerminal(output);
        }
        if (cmd.startsWith('kubectl apply')) {
          _refreshResources();
        }
      } else {
        _appendTerminal('Error (HTTP ${res.statusCode}): ${res.body}');
      }
    } catch (e) {
      _appendTerminal('⚠️ Command execution failed: $e');
    }
  }

  Future<void> _applyYaml() async {
    final yaml = _yamlController.text;
    _appendTerminal('\$ kubectl apply -f manifest.yaml');

    if (!_isBackendConnected) {
      _appendTerminal('⛔ [SECURITY] YAML apply requires a live backend connection.');
      _appendTerminal('   Simulated apply is disabled by Zero-Trust policy.');
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Cannot apply: no live cluster connection'),
          backgroundColor: Color(0xFFDC2626),
          duration: Duration(seconds: 3),
        ),
      );
      return;
    }

    try {
      final res = await http.post(
        Uri.parse('http://10.0.2.2:8080/v1/labs/sessions/$_sessionId/apply'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({'yaml_content': yaml}),
      ).timeout(const Duration(seconds: 10));

      if (res.statusCode == 200) {
        final data = jsonDecode(res.body);
        final applied = data['applied_resources'] as List<dynamic>? ?? [];
        _appendTerminal('✅ Server-side apply: ${applied.length} resource(s) applied to $_namespace');
        for (final r in applied) {
          _appendTerminal('   ${r['kind']}/${r['name']} ${r['action']}');
        }
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('${applied.length} resource(s) applied via server-side apply'),
            backgroundColor: const Color(0xFF10B981),
            duration: const Duration(seconds: 2),
          ),
        );
      } else {
        _appendTerminal('❌ Server-side apply failed (HTTP ${res.statusCode})');
        _appendTerminal('   ${res.body}');
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Apply failed: HTTP ${res.statusCode}'),
            backgroundColor: const Color(0xFFDC2626),
            duration: const Duration(seconds: 3),
          ),
        );
      }
    } catch (e) {
      _appendTerminal('⚠️ Apply request failed: $e');
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Apply request failed — check connection'),
          backgroundColor: Color(0xFFDC2626),
          duration: Duration(seconds: 3),
        ),
      );
    }
    _refreshResources();
  }

  Future<void> _refreshResources() async {
    if (!_isBackendConnected) {
      setState(() {
        _k8sResources = [];
        _k8sEvents = ['⛔ No live cluster connection — resource list unavailable'];
      });
      return;
    }

    setState(() => _isLoadingResources = true);

    try {
      // Fetch live resources from backend API
      final resResponse = await http.get(
        Uri.parse('http://10.0.2.2:8080/v1/labs/sessions/$_sessionId/resources'),
        headers: {'Content-Type': 'application/json'},
      ).timeout(const Duration(seconds: 5));

      if (resResponse.statusCode == 200) {
        final data = jsonDecode(resResponse.body);
        final resources = data['resources'] as List<dynamic>? ?? [];
        setState(() {
          _k8sResources = resources.map<Map<String, dynamic>>((r) => {
            'kind': r['kind']?.toString() ?? 'Unknown',
            'name': r['name']?.toString() ?? 'unnamed',
            'status': r['status']?.toString() ?? 'Unknown',
            'ready': r['details']?.toString() ?? '-',
            'age': r['age']?.toString() ?? '-',
            'ip': r['namespace']?.toString() ?? '-',
          }).toList();
        });
      } else {
        setState(() {
          _k8sResources = [];
        });
      }

      // Fetch live events
      final evResponse = await http.get(
        Uri.parse('http://10.0.2.2:8080/v1/labs/sessions/$_sessionId/events'),
        headers: {'Content-Type': 'application/json'},
      ).timeout(const Duration(seconds: 5));

      if (evResponse.statusCode == 200) {
        final evData = jsonDecode(evResponse.body);
        final events = evData['events'] as List<dynamic>? ?? [];
        setState(() {
          _k8sEvents = events.map<String>((e) => e.toString()).toList();
        });
      } else {
        setState(() {
          _k8sEvents = ['Failed to fetch events (HTTP ${evResponse.statusCode})'];
        });
      }
    } catch (e) {
      setState(() {
        _k8sResources = [];
        _k8sEvents = ['⚠️ Failed to fetch cluster state: $e'];
      });
    } finally {
      if (mounted) {
        setState(() => _isLoadingResources = false);
      }
    }
  }

  Future<void> _runValidation() async {
    if (!_isBackendConnected) {
      setState(() {
        _validationResult = {
          'passed': false,
          'score': 0,
          'message': '⛔ Validation requires a live backend connection. Simulated grading is disabled.',
        };
      });
      _appendTerminal('⛔ Cannot grade: no live cluster connection. Simulated scoring is forbidden.');
      return;
    }

    setState(() => _isValidating = true);
    _appendTerminal('🧪 Querying live Kubernetes cluster state for validation...');

    try {
      final res = await http.post(
        Uri.parse('http://10.0.2.2:8080/v1/labs/sessions/$_sessionId/validate'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({'task_id': 'all'}),
      ).timeout(const Duration(seconds: 10));

      if (res.statusCode == 200) {
        final data = jsonDecode(res.body);
        final passed = data['passed'] == true;
        final score = data['score'] ?? 0;
        final message = data['message'] ?? (passed ? 'All assertions passed against live cluster state.' : 'One or more assertions failed.');

        setState(() {
          _validationResult = {
            'passed': passed,
            'score': score,
            'message': message,
          };
        });

        if (passed) {
          await ProgressService.instance.markLessonCompleted(widget.labId, score as int);
          _appendTerminal('🎉 LAB PASSED! Score: $score (live cluster state verified)');
        } else {
          _appendTerminal('❌ Validation FAILED: $message');
          // Show assertion details if available
          final assertions = data['assertions'] as List<dynamic>?;
          if (assertions != null) {
            for (final a in assertions) {
              final aPassed = a['passed'] == true;
              final field = a['field'] ?? '';
              final error = a['error_message'] ?? '';
              _appendTerminal('   ${aPassed ? '✅' : '❌'} $field${aPassed ? '' : ': $error'}');
            }
          }
        }
      } else {
        setState(() {
          _validationResult = {
            'passed': false,
            'score': 0,
            'message': 'Validation request failed (HTTP ${res.statusCode})',
          };
        });
        _appendTerminal('❌ Validation request failed: HTTP ${res.statusCode}');
      }
    } catch (e) {
      setState(() {
        _validationResult = {
          'passed': false,
          'score': 0,
          'message': 'Validation request error: $e',
        };
      });
      _appendTerminal('⚠️ Validation request failed: $e');
    } finally {
      setState(() => _isValidating = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    final title = widget.labTitle ?? widget.labId;

    return Scaffold(
      backgroundColor: const Color(0xFF0A0E17),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0F172A),
        title: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              title,
              style: const TextStyle(fontSize: 14, fontWeight: FontWeight.bold, color: Colors.white),
              overflow: TextOverflow.ellipsis,
            ),
            Row(
              children: [
                Container(
                  width: 6,
                  height: 6,
                  decoration: BoxDecoration(
                    color: _isBackendConnected ? const Color(0xFF10B981) : const Color(0xFFF59E0B),
                    shape: BoxShape.circle,
                  ),
                ),
                const SizedBox(width: 6),
                Text(
                  _isBackendConnected ? 'LIVE CLUSTER • $_namespace' : 'DISCONNECTED • $_namespace',
                  style: TextStyle(fontSize: 10, color: _isBackendConnected ? const Color(0xFF94A3B8) : const Color(0xFFF87171)),
                ),
              ],
            ),
          ],
        ),
        bottom: TabBar(
          controller: _tabController,
          isScrollable: true,
          labelColor: const Color(0xFF06B6D4),
          unselectedLabelColor: const Color(0xFF94A3B8),
          indicatorColor: const Color(0xFF06B6D4),
          tabs: const [
            Tab(icon: Icon(Icons.assignment_outlined, size: 16), text: 'Steps'),
            Tab(icon: Icon(Icons.terminal, size: 16), text: 'Terminal'),
            Tab(icon: Icon(Icons.code, size: 16), text: 'YAML'),
            Tab(icon: Icon(Icons.inventory_2_outlined, size: 16), text: 'Resources'),
            Tab(icon: Icon(Icons.receipt_long, size: 16), text: 'Logs/Events'),
            Tab(icon: Icon(Icons.check_circle_outline, size: 16), text: 'Grade'),
            Tab(icon: Icon(Icons.lightbulb_outline, size: 16), text: 'Hints'),
          ],
        ),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh, color: Color(0xFF38BDF8)),
            tooltip: 'Restart Lab Session',
            onPressed: _startLabSession,
          ),
        ],
      ),
      body: SafeArea(
        child: Column(
          children: [
            if (_isSessionStarting)
              const LinearProgressIndicator(
                minHeight: 2,
                backgroundColor: Color(0xFF1E293B),
                valueColor: AlwaysStoppedAnimation<Color>(Color(0xFF06B6D4)),
              ),
            Expanded(
              child: TabBarView(
                controller: _tabController,
                children: [
                  _buildInstructionsTab(),
                  _buildTerminalTab(),
                  _buildYamlEditorTab(),
                  _buildResourcesTab(),
                  _buildLogsEventsTab(),
                  _buildGradeTab(),
                  _buildHintsTab(),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildInstructionsTab() {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Container(
            padding: const EdgeInsets.all(14),
            decoration: BoxDecoration(
              color: const Color(0x1A06B6D4),
              borderRadius: BorderRadius.circular(12),
              border: Border.all(color: const Color(0x3306B6D4)),
            ),
            child: Row(
              children: [
                const Icon(Icons.rocket_launch, color: Color(0xFF06B6D4), size: 20),
                const SizedBox(width: 10),
                Expanded(
                  child: Text(
                    'Lab Scenario: ${widget.labId}',
                    style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 13),
                  ),
                ),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                  decoration: BoxDecoration(
                    color: const Color(0xFF10B981),
                    borderRadius: BorderRadius.circular(6),
                  ),
                  child: const Text('100 PTS', style: TextStyle(color: Colors.black, fontWeight: FontWeight.bold, fontSize: 10)),
                ),
              ],
            ),
          ),
          const SizedBox(height: 16),
          const Text('OBJECTIVES & SUCCESS CRITERIA', style: TextStyle(color: Color(0xFF94A3B8), fontSize: 11, fontWeight: FontWeight.bold, letterSpacing: 1.1)),
          const SizedBox(height: 10),
          _buildObjectiveCard('1. Deploy Declarative Specification', 'Apply the valid YAML manifest targeting namespace $_namespace.', 50),
          _buildObjectiveCard('2. Verify Container Health & Readiness', 'Ensure all readiness probes succeed and pod reaches status Running.', 50),
          const SizedBox(height: 20),
          const Text('RECOMMENDED WORKFLOW', style: TextStyle(color: Color(0xFF94A3B8), fontSize: 11, fontWeight: FontWeight.bold, letterSpacing: 1.1)),
          const SizedBox(height: 10),
          const Text(
            '1. Switch to the YAML tab to inspect and edit the manifest.\n2. Tap "Apply to Cluster" to deploy changes.\n3. Switch to Terminal tab to inspect live status using kubectl.\n4. Open Grade tab and tap "Validate & Grade Lab" to record your score.',
            style: TextStyle(color: Color(0xFFCBD5E1), fontSize: 12, height: 1.6),
          ),
          const SizedBox(height: 20),
          ElevatedButton.icon(
            style: ElevatedButton.styleFrom(
              backgroundColor: const Color(0xFF06B6D4),
              foregroundColor: const Color(0xFF0A0E17),
              minimumSize: const Size.fromHeight(44),
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
            ),
            icon: const Icon(Icons.code),
            label: const Text('OPEN YAML EDITOR', style: TextStyle(fontWeight: FontWeight.bold)),
            onPressed: () => _tabController.animateTo(2),
          ),
        ],
      ),
    );
  }

  Widget _buildObjectiveCard(String title, String desc, int points) {
    return Container(
      margin: const EdgeInsets.only(bottom: 10),
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: const Color(0xFF0F172A),
        borderRadius: BorderRadius.circular(10),
        border: Border.all(color: const Color(0xFF1E293B)),
      ),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Icon(Icons.check_circle_outline, color: Color(0xFF10B981), size: 18),
          const SizedBox(width: 10),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(title, style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 12)),
                const SizedBox(height: 2),
                Text(desc, style: const TextStyle(color: Color(0xFF94A3B8), fontSize: 11)),
              ],
            ),
          ),
          Text('+$points pts', style: const TextStyle(color: Color(0xFF38BDF8), fontWeight: FontWeight.bold, fontSize: 11)),
        ],
      ),
    );
  }

  Widget _buildTerminalTab() {
    return Column(
      children: [
        // Virtual Keyboard Quick Bar
        Container(
          height: 38,
          color: const Color(0xFF1E293B),
          child: ListView(
            scrollDirection: Axis.horizontal,
            padding: const EdgeInsets.symmetric(horizontal: 8),
            children: [
              _buildKeyBtn('Ctrl+C', () => _handleTerminalCommand('^C')),
              _buildKeyBtn('Tab', () => _handleTerminalCommand('\t')),
              _buildKeyBtn('Clear', () => _handleTerminalCommand('clear')),
              _buildKeyBtn('kgp', () => _handleTerminalCommand('kubectl get pods -n $_namespace')),
              _buildKeyBtn('kgs', () => _handleTerminalCommand('kubectl get svc -n $_namespace')),
              _buildKeyBtn('kge', () => _handleTerminalCommand('kubectl get events -n $_namespace')),
              _buildKeyBtn('describe', () => _handleTerminalCommand('kubectl describe pod ${widget.labId} -n $_namespace')),
            ],
          ),
        ),

        // Terminal Output Screen
        Expanded(
          child: Container(
            color: const Color(0xFF050811),
            padding: const EdgeInsets.all(12),
            child: ListView.builder(
              controller: _terminalScrollController,
              itemCount: _terminalOutput.length,
              itemBuilder: (context, idx) {
                final line = _terminalOutput[idx];
                final isCmd = line.startsWith('\$');
                return SelectableText(
                  line,
                  style: TextStyle(
                    fontFamily: 'monospace',
                    fontSize: 12,
                    height: 1.4,
                    color: isCmd ? const Color(0xFF38BDF8) : (line.startsWith('✅') || line.startsWith('🎉') ? const Color(0xFF10B981) : const Color(0xFFE2E8F0)),
                    fontWeight: isCmd ? FontWeight.bold : FontWeight.normal,
                  ),
                );
              },
            ),
          ),
        ),

        // Command Input Bar
        Container(
          color: const Color(0xFF0F172A),
          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
          child: Row(
            children: [
              const Text('\$ ', style: TextStyle(color: Color(0xFF06B6D4), fontFamily: 'monospace', fontWeight: FontWeight.bold)),
              Expanded(
                child: TextField(
                  controller: _terminalInputController,
                  style: const TextStyle(color: Colors.white, fontFamily: 'monospace', fontSize: 13),
                  decoration: const InputDecoration(
                    hintText: 'Type kubectl command...',
                    hintStyle: TextStyle(color: Color(0xFF64748B), fontSize: 12),
                    border: InputBorder.none,
                    isDense: true,
                  ),
                  onSubmitted: _handleTerminalCommand,
                ),
              ),
              IconButton(
                icon: const Icon(Icons.send, color: Color(0xFF06B6D4), size: 18),
                onPressed: () => _handleTerminalCommand(_terminalInputController.text),
              ),
            ],
          ),
        ),
      ],
    );
  }

  Widget _buildKeyBtn(String label, VoidCallback onTap) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 3, vertical: 5),
      child: OutlinedButton(
        style: OutlinedButton.styleFrom(
          foregroundColor: const Color(0xFFCBD5E1),
          side: const BorderSide(color: Color(0xFF334155)),
          padding: const EdgeInsets.symmetric(horizontal: 8),
          minimumSize: Size.zero,
          tapTargetSize: MaterialTapTargetSize.shrinkWrap,
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(4)),
        ),
        onPressed: onTap,
        child: Text(label, style: const TextStyle(fontFamily: 'monospace', fontSize: 10, fontWeight: FontWeight.bold)),
      ),
    );
  }

  Widget _buildYamlEditorTab() {
    return Column(
      children: [
        // Editor Actions Toolbar
        Container(
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
          color: const Color(0xFF0F172A),
          child: Row(
            children: [
              const Icon(Icons.edit_note, color: Color(0xFF38BDF8), size: 18),
              const SizedBox(width: 8),
              const Text('manifest.yaml', style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 12)),
              const Spacer(),
              TextButton.icon(
                icon: const Icon(Icons.restart_alt, size: 14),
                label: const Text('Reset', style: TextStyle(fontSize: 11)),
                style: TextButton.styleFrom(foregroundColor: const Color(0xFF94A3B8)),
                onPressed: _initDefaultYaml,
              ),
              const SizedBox(width: 6),
              ElevatedButton.icon(
                icon: const Icon(Icons.play_arrow, size: 16),
                label: const Text('APPLY', style: TextStyle(fontSize: 11, fontWeight: FontWeight.bold)),
                style: ElevatedButton.styleFrom(
                  backgroundColor: const Color(0xFF10B981),
                  foregroundColor: Colors.black,
                  padding: const EdgeInsets.symmetric(horizontal: 10),
                  minimumSize: const Size(60, 32),
                ),
                onPressed: _applyYaml,
              ),
            ],
          ),
        ),

        // Code Editor
        Expanded(
          child: Container(
            color: const Color(0xFF050811),
            padding: const EdgeInsets.all(12),
            child: TextField(
              controller: _yamlController,
              maxLines: null,
              expands: true,
              style: const TextStyle(
                fontFamily: 'monospace',
                fontSize: 12,
                color: Color(0xFF67E8F9),
                height: 1.4,
              ),
              decoration: const InputDecoration(
                border: InputBorder.none,
                isDense: true,
              ),
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildResourcesTab() {
    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.all(12),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text('LIVE CLUSTER OBJECTS • $_namespace', style: const TextStyle(color: Color(0xFF94A3B8), fontSize: 11, fontWeight: FontWeight.bold)),
              IconButton(
                icon: const Icon(Icons.refresh, color: Color(0xFF38BDF8), size: 18),
                onPressed: _refreshResources,
              ),
            ],
          ),
        ),
        if (_isLoadingResources)
          const LinearProgressIndicator(color: Color(0xFF06B6D4)),
        Expanded(
          child: ListView.builder(
            padding: const EdgeInsets.symmetric(horizontal: 12),
            itemCount: _k8sResources.length,
            itemBuilder: (context, idx) {
              final r = _k8sResources[idx];
              return Container(
                margin: const EdgeInsets.only(bottom: 8),
                padding: const EdgeInsets.all(12),
                decoration: BoxDecoration(
                  color: const Color(0xFF0F172A),
                  borderRadius: BorderRadius.circular(10),
                  border: Border.all(color: const Color(0xFF1E293B)),
                ),
                child: Row(
                  children: [
                    Container(
                      padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                      decoration: BoxDecoration(
                        color: const Color(0x3338BDF8),
                        borderRadius: BorderRadius.circular(4),
                      ),
                      child: Text(r['kind']!, style: const TextStyle(color: Color(0xFF38BDF8), fontSize: 10, fontWeight: FontWeight.bold)),
                    ),
                    const SizedBox(width: 10),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(r['name']!, style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 12)),
                          Text('Ready: ${r['ready']} • Age: ${r['age']} • IP: ${r['ip']}', style: const TextStyle(color: Color(0xFF94A3B8), fontSize: 10)),
                        ],
                      ),
                    ),
                    Container(
                      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                      decoration: BoxDecoration(
                        color: const Color(0x2610B981),
                        borderRadius: BorderRadius.circular(6),
                      ),
                      child: Text(r['status']!, style: const TextStyle(color: Color(0xFF10B981), fontWeight: FontWeight.bold, fontSize: 10)),
                    ),
                  ],
                ),
              );
            },
          ),
        ),
      ],
    );
  }

  Widget _buildLogsEventsTab() {
    return ListView(
      padding: const EdgeInsets.all(12),
      children: [
        const Text('NAMESPACE EVENTS', style: TextStyle(color: Color(0xFF94A3B8), fontSize: 11, fontWeight: FontWeight.bold)),
        const SizedBox(height: 8),
        for (final ev in _k8sEvents)
          Container(
            margin: const EdgeInsets.only(bottom: 6),
            padding: const EdgeInsets.all(10),
            decoration: BoxDecoration(
              color: const Color(0xFF0F172A),
              borderRadius: BorderRadius.circular(8),
              border: Border.all(color: const Color(0xFF1E293B)),
            ),
            child: Text(ev, style: const TextStyle(color: Color(0xFFCBD5E1), fontSize: 11, fontFamily: 'monospace')),
          ),
      ],
    );
  }

  Widget _buildGradeTab() {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(20),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Container(
            padding: const EdgeInsets.all(20),
            decoration: BoxDecoration(
              color: const Color(0xFF0F172A),
              borderRadius: BorderRadius.circular(16),
              border: Border.all(color: const Color(0xFF1E293B)),
            ),
            child: Column(
              children: [
                const Icon(Icons.fact_check_outlined, color: Color(0xFF06B6D4), size: 40),
                const SizedBox(height: 12),
                const Text('Deterministic Lab Grading', style: TextStyle(color: Colors.white, fontSize: 16, fontWeight: FontWeight.bold)),
                const SizedBox(height: 6),
                const Text('Evaluates live Kubernetes objects against deterministic acceptance criteria.', style: TextStyle(color: Color(0xFF94A3B8), fontSize: 11), textAlign: TextAlign.center),
                const SizedBox(height: 20),
                ElevatedButton.icon(
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFF06B6D4),
                    foregroundColor: const Color(0xFF0A0E17),
                    minimumSize: const Size.fromHeight(48),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                  ),
                  icon: _isValidating ? const SizedBox(width: 16, height: 16, child: CircularProgressIndicator(strokeWidth: 2, color: Colors.black)) : const Icon(Icons.check_circle),
                  label: Text(_isValidating ? 'VALIDATING CLUSTER...' : 'VALIDATE & GRADE LAB', style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 13)),
                  onPressed: _isValidating ? null : _runValidation,
                ),
              ],
            ),
          ),
          if (_validationResult != null) ...[
            const SizedBox(height: 20),
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: const Color(0x1A10B981),
                borderRadius: BorderRadius.circular(14),
                border: Border.all(color: const Color(0xFF10B981)),
              ),
              child: Column(
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      const Icon(Icons.emoji_events, color: Color(0xFFFBBF24), size: 28),
                      const SizedBox(width: 10),
                      Text('SCORE: ${_validationResult!['score']} / 100', style: const TextStyle(color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold)),
                    ],
                  ),
                  const SizedBox(height: 8),
                  Text(_validationResult!['message']!, style: const TextStyle(color: Color(0xFF10B981), fontSize: 12), textAlign: TextAlign.center),
                ],
              ),
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildHintsTab() {
    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        const Text('PROGRESSIVE HINTS (10 PT PENALTY)', style: TextStyle(color: Color(0xFF94A3B8), fontSize: 11, fontWeight: FontWeight.bold)),
        const SizedBox(height: 12),
        _buildHintCard(1, 'Verify container status with `kubectl get pods -n $_namespace -o wide` and check node scheduling.'),
        _buildHintCard(2, 'Inspect failed events using `kubectl describe pod ${widget.labId} -n $_namespace`.'),
        _buildHintCard(3, 'Ensure `app: ${widget.labId}` label matches the Service selector in the YAML manifest.'),
      ],
    );
  }

  Widget _buildHintCard(int index, String text) {
    final isRevealed = _revealedHints >= index;

    return Container(
      margin: const EdgeInsets.only(bottom: 12),
      padding: const EdgeInsets.all(14),
      decoration: BoxDecoration(
        color: const Color(0xFF0F172A),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: isRevealed ? const Color(0xFF06B6D4) : const Color(0xFF1E293B)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(isRevealed ? Icons.lightbulb : Icons.lock_outline, color: isRevealed ? const Color(0xFFFBBF24) : const Color(0xFF64748B), size: 16),
              const SizedBox(width: 8),
              Text('Hint #$index', style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 12)),
              const Spacer(),
              if (!isRevealed)
                TextButton(
                  style: TextButton.styleFrom(foregroundColor: const Color(0xFF38BDF8), padding: EdgeInsets.zero, minimumSize: Size.zero),
                  child: const Text('Reveal (-10 pts)', style: TextStyle(fontSize: 11)),
                  onPressed: () {
                    setState(() => _revealedHints = index);
                  },
                ),
            ],
          ),
          if (isRevealed) ...[
            const SizedBox(height: 8),
            Text(text, style: const TextStyle(color: Color(0xFFCBD5E1), fontSize: 12, height: 1.5)),
          ],
        ],
      ),
    );
  }

  @override
  void dispose() {
    _tabController.dispose();
    _yamlController.dispose();
    _terminalInputController.dispose();
    _terminalScrollController.dispose();
    super.dispose();
  }
}
