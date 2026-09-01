import 'package:flutter/material.dart';
import '../services/api_service.dart';
import 'login_screen.dart';

class SettingsScreen extends StatefulWidget {
  final ApiService? apiService;

  const SettingsScreen({super.key, this.apiService});

  @override
  State<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends State<SettingsScreen> {
  late final ApiService _apiService;
  final _serverUrlController = TextEditingController();
  bool _notificationsEnabled = true;
  bool _offlineCacheEnabled = true;
  bool _hapticFeedbackEnabled = true;
  bool _isLoggedIn = false;

  @override
  void initState() {
    super.initState();
    _apiService = widget.apiService ?? ApiService();
    _serverUrlController.text = _apiService.baseUrl;
    _checkAuth();
  }

  Future<void> _checkAuth() async {
    final token = await _apiService.getToken();
    if (mounted) {
      setState(() {
        _isLoggedIn = token != null && token.isNotEmpty;
      });
    }
  }

  Future<void> _handleLogout() async {
    await _apiService.clearToken();
    if (mounted) {
      setState(() {
        _isLoggedIn = false;
      });
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Logged out successfully')),
      );
    }
  }

  @override
  void dispose() {
    _serverUrlController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0A0E17),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0F172A),
        title: const Text(
          'SETTINGS & PREFERENCES',
          style: TextStyle(
            fontSize: 15,
            fontWeight: FontWeight.bold,
            letterSpacing: 1.1,
            color: Color(0xFF06B6D4),
          ),
        ),
      ),
      body: ListView(
        padding: const EdgeInsets.all(20.0),
        children: [
          const Text(
            'ACCOUNT & AUTHENTICATION',
            style: TextStyle(color: Color(0xFF64748B), fontSize: 12, fontWeight: FontWeight.bold, letterSpacing: 1.1),
          ),
          const SizedBox(height: 12),
          Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: const Color(0xFF0F172A),
              borderRadius: BorderRadius.circular(8),
              border: Border.all(color: const Color(0xFF1E293B)),
            ),
            child: Row(
              children: [
                Icon(
                  _isLoggedIn ? Icons.verified_user : Icons.person_outline,
                  color: _isLoggedIn ? const Color(0xFF10B981) : const Color(0xFF94A3B8),
                  size: 28,
                ),
                const SizedBox(width: 14),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        _isLoggedIn ? 'Signed In as Learner' : 'Not Signed In',
                        style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 14),
                      ),
                      Text(
                        _isLoggedIn ? 'Cloud Sync & Certifications Active' : 'Sign in to sync your progress',
                        style: const TextStyle(color: Color(0xFF64748B), fontSize: 12),
                      ),
                    ],
                  ),
                ),
                ElevatedButton(
                  onPressed: _isLoggedIn
                      ? _handleLogout
                      : () async {
                          await Navigator.of(context).push(
                            MaterialPageRoute(builder: (_) => LoginScreen(apiService: _apiService)),
                          );
                          _checkAuth();
                        },
                  style: ElevatedButton.styleFrom(
                    backgroundColor: _isLoggedIn ? const Color(0xFF1E293B) : const Color(0xFF06B6D4),
                    foregroundColor: _isLoggedIn ? const Color(0xFFEF4444) : const Color(0xFF0A0E17),
                    padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 8),
                  ),
                  child: Text(_isLoggedIn ? 'Logout' : 'Sign In'),
                ),
              ],
            ),
          ),
          const SizedBox(height: 28),
          const Text(
            'CONNECTIVITY & API GATEWAY',
            style: TextStyle(color: Color(0xFF64748B), fontSize: 12, fontWeight: FontWeight.bold, letterSpacing: 1.1),
          ),
          const SizedBox(height: 12),
          Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: const Color(0xFF0F172A),
              borderRadius: BorderRadius.circular(8),
              border: Border.all(color: const Color(0xFF1E293B)),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('Gateway Base URL', style: TextStyle(color: Colors.white, fontSize: 13, fontWeight: FontWeight.bold)),
                const SizedBox(height: 8),
                TextField(
                  controller: _serverUrlController,
                  style: const TextStyle(color: Colors.white, fontSize: 13),
                  decoration: InputDecoration(
                    hintText: 'http://localhost:8080',
                    hintStyle: const TextStyle(color: Color(0xFF64748B)),
                    filled: true,
                    fillColor: const Color(0xFF0A0E17),
                    border: OutlineInputBorder(borderRadius: BorderRadius.circular(6), borderSide: const BorderSide(color: Color(0xFF1E293B))),
                  ),
                ),
                const SizedBox(height: 10),
                ElevatedButton(
                  onPressed: () {
                    _apiService.baseUrl = _serverUrlController.text.trim();
                    ScaffoldMessenger.of(context).showSnackBar(
                      SnackBar(content: Text('API Gateway URL updated to ${_apiService.baseUrl}')),
                    );
                  },
                  style: ElevatedButton.styleFrom(backgroundColor: const Color(0xFF06B6D4), foregroundColor: Colors.black),
                  child: const Text('Save Server URL'),
                ),
              ],
            ),
          ),
          const SizedBox(height: 28),
          const Text(
            'PREFERENCES',
            style: TextStyle(color: Color(0xFF64748B), fontSize: 12, fontWeight: FontWeight.bold, letterSpacing: 1.1),
          ),
          const SizedBox(height: 12),
          Container(
            decoration: BoxDecoration(
              color: const Color(0xFF0F172A),
              borderRadius: BorderRadius.circular(8),
              border: Border.all(color: const Color(0xFF1E293B)),
            ),
            child: Column(
              children: [
                SwitchListTile(
                  title: const Text('Push Notifications', style: TextStyle(color: Colors.white, fontSize: 14)),
                  subtitle: const Text('Daily practice reminders & incident alerts', style: TextStyle(color: Color(0xFF64748B), fontSize: 12)),
                  value: _notificationsEnabled,
                  activeColor: const Color(0xFF06B6D4),
                  onChanged: (v) => setState(() => _notificationsEnabled = v),
                ),
                const Divider(color: Color(0xFF1E293B), height: 1),
                SwitchListTile(
                  title: const Text('Offline Track Caching', style: TextStyle(color: Colors.white, fontSize: 14)),
                  subtitle: const Text('Pre-fetch lesson theory and quizzes for offline study', style: TextStyle(color: Color(0xFF64748B), fontSize: 12)),
                  value: _offlineCacheEnabled,
                  activeColor: const Color(0xFF06B6D4),
                  onChanged: (v) => setState(() => _offlineCacheEnabled = v),
                ),
                const Divider(color: Color(0xFF1E293B), height: 1),
                SwitchListTile(
                  title: const Text('Haptic Feedback', style: TextStyle(color: Colors.white, fontSize: 14)),
                  subtitle: const Text('Vibrate on quiz submission and XP milestones', style: TextStyle(color: Color(0xFF64748B), fontSize: 12)),
                  value: _hapticFeedbackEnabled,
                  activeColor: const Color(0xFF06B6D4),
                  onChanged: (v) => setState(() => _hapticFeedbackEnabled = v),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
