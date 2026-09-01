import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:shared_preferences/shared_preferences.dart';

class ApiService {
  static const String _defaultBaseUrl = 'http://10.0.2.2:8080'; // Android emulator to host
  static const String _tokenKey = 'kubelab_jwt_token';
  static const String _offlineQueueKey = 'kubelab_offline_queue';

  final http.Client _client;
  String _baseUrl;

  ApiService({http.Client? client, String baseUrl = _defaultBaseUrl})
      : _client = client ?? http.Client(),
        _baseUrl = baseUrl;

  String get baseUrl => _baseUrl;
  set baseUrl(String url) => _baseUrl = url;

  Future<String?> getToken() async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.getString(_tokenKey);
  }

  Future<void> setToken(String token) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_tokenKey, token);
  }

  Future<void> clearToken() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.remove(_tokenKey);
  }

  Map<String, String> _headers(String? token) {
    return {
      'Content-Type': 'application/json',
      'Accept': 'application/json',
      if (token != null) 'Authorization': 'Bearer $token',
    };
  }

  // 1. Auth: Login
  Future<Map<String, dynamic>> login(String email, String password) async {
    final url = Uri.parse('$_baseUrl/api/auth/login');
    final response = await _client.post(
      url,
      headers: _headers(null),
      body: jsonEncode({'email': email, 'password': password}),
    );

    if (response.statusCode == 200) {
      final data = jsonDecode(response.body) as Map<String, dynamic>;
      final token = data['token'] as String?;
      if (token != null) {
        await setToken(token);
      }
      return data;
    } else {
      throw Exception('Login failed: ${response.statusCode} - ${response.body}');
    }
  }

  // 2. Auth: Register
  Future<Map<String, dynamic>> register(String name, String email, String password) async {
    final url = Uri.parse('$_baseUrl/api/auth/register');
    final response = await _client.post(
      url,
      headers: _headers(null),
      body: jsonEncode({
        'name': name,
        'email': email,
        'password': password,
        'role': 'learner',
      }),
    );

    if (response.statusCode == 200 || response.statusCode == 201) {
      final data = jsonDecode(response.body) as Map<String, dynamic>;
      final token = data['token'] as String?;
      if (token != null) {
        await setToken(token);
      }
      return data;
    } else {
      throw Exception('Registration failed: ${response.statusCode} - ${response.body}');
    }
  }

  // 3. Curriculum: Fetch Tracks
  Future<List<dynamic>> getTracks() async {
    final token = await getToken();
    final url = Uri.parse('$_baseUrl/api/tracks');
    try {
      final response = await _client.get(url, headers: _headers(token));
      if (response.statusCode == 200) {
        return jsonDecode(response.body) as List<dynamic>;
      }
    } catch (_) {
      // Return cached/offline tracks if network fails
    }
    return [];
  }

  // 4. Progress: Sync XP and Badges
  Future<Map<String, dynamic>> getProgress() async {
    final token = await getToken();
    final url = Uri.parse('$_baseUrl/api/progress');
    final response = await _client.get(url, headers: _headers(token));
    if (response.statusCode == 200) {
      return jsonDecode(response.body) as Map<String, dynamic>;
    }
    throw Exception('Failed to fetch progress: ${response.statusCode}');
  }

  // 5. Offline Queue: Enqueue completion
  Future<void> queueOfflineAction(String actionType, Map<String, dynamic> payload) async {
    final prefs = await SharedPreferences.getInstance();
    final queue = prefs.getStringList(_offlineQueueKey) ?? [];
    final item = jsonEncode({
      'action': actionType,
      'payload': payload,
      'timestamp': DateTime.now().toIso8601String(),
    });
    queue.add(item);
    await prefs.setStringList(_offlineQueueKey, queue);
  }

  // 6. Offline Queue: Flush and sync
  Future<int> flushOfflineQueue() async {
    final prefs = await SharedPreferences.getInstance();
    final queue = prefs.getStringList(_offlineQueueKey) ?? [];
    if (queue.isEmpty) return 0;

    int syncedCount = 0;
    final List<String> remaining = [];

    for (final rawItem in queue) {
      try {
        final item = jsonDecode(rawItem) as Map<String, dynamic>;
        final action = item['action'] as String;
        final payload = item['payload'] as Map<String, dynamic>;

        if (action == 'quiz_submission') {
          final token = await getToken();
          final url = Uri.parse('$_baseUrl/api/assessment/grade');
          final resp = await _client.post(url, headers: _headers(token), body: jsonEncode(payload));
          if (resp.statusCode == 200) {
            syncedCount++;
            continue;
          }
        }
        remaining.add(rawItem);
      } catch (_) {
        remaining.add(rawItem);
      }
    }

    await prefs.setStringList(_offlineQueueKey, remaining);
    return syncedCount;
  }
}
