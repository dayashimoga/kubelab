import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'data/curriculum_data.dart';
import 'screens/home_screen.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(const KubeLabApp());
}

class KubeLabApp extends StatelessWidget {
  const KubeLabApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'KubeLab Mobile',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        useMaterial3: true,
        brightness: Brightness.dark,
        scaffoldBackgroundColor: const Color(0xFF0A0E17),
        primaryColor: const Color(0xFF06B6D4),
        colorScheme: const ColorScheme.dark(
          primary: Color(0xFF06B6D4),
          secondary: Color(0xFF6366F1),
          surface: Color(0xFF0F172A),
        ),
        fontFamily: 'Roboto',
      ),
      home: const CurriculumBootstrap(),
    );
  }
}

/// Bootstrap widget that loads the curriculum JSON asset before showing the app.
/// This is the SINGLE point of truth for curriculum initialization.
/// It handles loading, error, and retry states explicitly.
class CurriculumBootstrap extends StatefulWidget {
  const CurriculumBootstrap({super.key});

  @override
  State<CurriculumBootstrap> createState() => _CurriculumBootstrapState();
}

class _CurriculumBootstrapState extends State<CurriculumBootstrap> {
  bool _isLoading = true;
  String? _errorMessage;

  @override
  void initState() {
    super.initState();
    _loadCurriculum();
  }

  Future<void> _loadCurriculum() async {
    setState(() {
      _isLoading = true;
      _errorMessage = null;
    });

    try {
      final rawJson = await rootBundle.loadString('assets/data/curriculum.json');

      if (rawJson.isEmpty) {
        throw Exception('curriculum.json asset is empty (0 bytes)');
      }

      CurriculumRepository.initializeFromJson(rawJson);

      final trackCount = CurriculumRepository.tracks.length;
      final lessonCount = CurriculumRepository.tracks
          .expand((t) => t.modules)
          .expand((m) => m.lessons)
          .length;
      final quizCount = CurriculumRepository.quizzes.length;

      // Assert authoritative counts — fail loudly on data corruption
      if (trackCount == 0) {
        throw Exception(
            'Curriculum deserialization produced 0 tracks. '
            'Expected 15 tracks from curriculum.json. '
            'Check JSON structure: data.tracks[] array.');
      }

      debugPrint(
          '[CurriculumBootstrap] Loaded: '
          'TRACKS=$trackCount LESSONS=$lessonCount QUIZZES=$quizCount');

      if (mounted) {
        setState(() {
          _isLoading = false;
        });
      }
    } catch (e, stack) {
      debugPrint('[CurriculumBootstrap] FATAL: Failed to load curriculum: $e');
      debugPrint('$stack');
      if (mounted) {
        setState(() {
          _isLoading = false;
          _errorMessage = e.toString();
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return const Scaffold(
        backgroundColor: Color(0xFF0A0E17),
        body: Center(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              CircularProgressIndicator(color: Color(0xFF06B6D4)),
              SizedBox(height: 20),
              Text(
                'Loading Curriculum...',
                style: TextStyle(
                  color: Color(0xFF94A3B8),
                  fontSize: 14,
                  fontWeight: FontWeight.w500,
                ),
              ),
            ],
          ),
        ),
      );
    }

    if (_errorMessage != null) {
      return Scaffold(
        backgroundColor: const Color(0xFF0A0E17),
        body: Center(
          child: Padding(
            padding: const EdgeInsets.all(32),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                const Icon(Icons.error_outline, color: Color(0xFFEF4444), size: 56),
                const SizedBox(height: 16),
                const Text(
                  'Failed to Load Curriculum',
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 12),
                Text(
                  _errorMessage!,
                  textAlign: TextAlign.center,
                  style: const TextStyle(
                    color: Color(0xFFFCA5A5),
                    fontSize: 12,
                    fontFamily: 'monospace',
                  ),
                ),
                const SizedBox(height: 24),
                ElevatedButton.icon(
                  onPressed: _loadCurriculum,
                  icon: const Icon(Icons.refresh),
                  label: const Text('RETRY'),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFF06B6D4),
                    foregroundColor: Colors.black,
                    padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 14),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(12),
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
      );
    }

    // Data loaded successfully — assert non-empty before proceeding
    assert(CurriculumRepository.tracks.isNotEmpty,
        'CurriculumRepository.tracks is empty after successful load');

    return const HomeScreen();
  }
}
