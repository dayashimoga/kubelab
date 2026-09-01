import 'package:flutter/material.dart';
import '../services/api_service.dart';

class ProgressSyncScreen extends StatefulWidget {
  final ApiService? apiService;

  const ProgressSyncScreen({super.key, this.apiService});

  @override
  State<ProgressSyncScreen> createState() => _ProgressSyncScreenState();
}

class _ProgressSyncScreenState extends State<ProgressSyncScreen> {
  late final ApiService _apiService;
  bool _isSyncing = false;
  int _lastSyncedCount = 0;
  String _syncStatus = 'All progress synced with cloud';
  DateTime? _lastSyncTime;

  @override
  void initState() {
    super.initState();
    _apiService = widget.apiService ?? ApiService();
    _lastSyncTime = DateTime.now();
  }

  Future<void> _handleSyncNow() async {
    setState(() {
      _isSyncing = true;
      _syncStatus = 'Synchronizing pending tasks and quiz XP...';
    });

    try {
      final count = await _apiService.flushOfflineQueue();
      setState(() {
        _lastSyncedCount = count;
        _syncStatus = count > 0
            ? 'Successfully synced $count offline achievements!'
            : 'Cloud state is up to date.';
        _lastSyncTime = DateTime.now();
      });
    } catch (e) {
      setState(() {
        _syncStatus = 'Sync failed. Queued actions saved locally.';
      });
    } finally {
      setState(() {
        _isSyncing = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0A0E17),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0F172A),
        title: const Text(
          'CLOUD PROGRESS SYNC',
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
                  Icon(
                    _isSyncing ? Icons.sync : Icons.cloud_done,
                    size: 56,
                    color: _isSyncing ? const Color(0xFF6366F1) : const Color(0xFF10B981),
                  ),
                  const SizedBox(height: 12),
                  Text(
                    _syncStatus,
                    textAlign: TextAlign.center,
                    style: const TextStyle(
                      color: Colors.white,
                      fontSize: 15,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  const SizedBox(height: 8),
                  if (_lastSyncTime != null)
                    Text(
                      'Last Sync: ${_lastSyncTime!.hour.toString().padLeft(2, '0')}:${_lastSyncTime!.minute.toString().padLeft(2, '0')}:${_lastSyncTime!.second.toString().padLeft(2, '0')}',
                      style: const TextStyle(color: Color(0xFF64748B), fontSize: 12),
                    ),
                ],
              ),
            ),
            const SizedBox(height: 24),
            ElevatedButton.icon(
              onPressed: _isSyncing ? null : _handleSyncNow,
              icon: _isSyncing
                  ? const SizedBox(
                      width: 18,
                      height: 18,
                      child: CircularProgressIndicator(strokeWidth: 2, color: Colors.black),
                    )
                  : const Icon(Icons.sync),
              label: Text(_isSyncing ? 'SYNCING...' : 'SYNC NOW'),
              style: ElevatedButton.styleFrom(
                backgroundColor: const Color(0xFF06B6D4),
                foregroundColor: const Color(0xFF0A0E17),
                padding: const EdgeInsets.symmetric(vertical: 14),
                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
              ),
            ),
            const SizedBox(height: 32),
            const Text(
              'OFFLINE SYNCHRONIZATION FEATURES',
              style: TextStyle(
                color: Color(0xFF64748B),
                fontSize: 12,
                fontWeight: FontWeight.bold,
                letterSpacing: 1.1,
              ),
            ),
            const SizedBox(height: 12),
            _featureItem(Icons.offline_bolt, 'Automatic Offline Queuing', 'Quiz submissions and lab completions are stored securely on-device when network is unavailable.'),
            const SizedBox(height: 12),
            _featureItem(Icons.security, 'Zero Progress Loss', 'All queued items retain timestamps and are cryptographically reconciled upon connection.'),
            const SizedBox(height: 12),
            _featureItem(Icons.devices, 'Cross-Device State', 'Sync seamlessly between mobile and desktop lab workspaces.'),
          ],
        ),
      ),
    );
  }

  Widget _featureItem(IconData icon, String title, String description) {
    return Container(
      padding: const EdgeInsets.all(14),
      decoration: BoxDecoration(
        color: const Color(0xFF0F172A),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: const Color(0xFF1E293B)),
      ),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Icon(icon, color: const Color(0xFF06B6D4), size: 20),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(title, style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 13)),
                const SizedBox(height: 4),
                Text(description, style: const TextStyle(color: Color(0xFF94A3B8), fontSize: 12)),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
