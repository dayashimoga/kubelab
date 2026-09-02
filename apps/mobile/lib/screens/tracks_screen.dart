import 'package:flutter/material.dart';
import '../data/curriculum_data.dart';
import '../services/progress_service.dart';
import 'modules_screen.dart';

class TracksScreen extends StatefulWidget {
  const TracksScreen({super.key});

  @override
  State<TracksScreen> createState() => _TracksScreenState();
}

class _TracksScreenState extends State<TracksScreen> {
  String _searchQuery = '';
  String _selectedDifficulty = 'all';
  final Map<String, double> _progressMap = {};

  @override
  void initState() {
    super.initState();
    _loadProgress();
  }

  Future<void> _loadProgress() async {
    for (final t in CurriculumRepository.tracks) {
      final p = await ProgressService.instance.getTrackProgressPercentage(t.slug);
      if (mounted) {
        setState(() {
          _progressMap[t.slug] = p;
        });
      }
    }
  }

  IconData _getIconData(String iconName) {
    switch (iconName.toLowerCase()) {
      case 'terminal':
        return Icons.terminal;
      case 'boxes':
        return Icons.view_in_ar;
      case 'database':
        return Icons.storage;
      case 'network':
        return Icons.hub;
      case 'package':
        return Icons.inventory_2;
      case 'server':
        return Icons.dns;
      case 'shieldcheck':
        return Icons.security;
      case 'gitbranch':
        return Icons.alt_route;
      case 'layers':
        return Icons.layers;
      case 'activity':
        return Icons.insights;
      case 'wrench':
        return Icons.build;
      case 'gauge':
        return Icons.speed;
      case 'cpu':
        return Icons.memory;
      case 'alerttriangle':
        return Icons.warning_amber_rounded;
      case 'award':
        return Icons.military_tech;
      default:
        return Icons.cloud_circle;
    }
  }

  Color _parseColor(String hex) {
    try {
      final clean = hex.replaceAll('#', '');
      return Color(int.parse('FF$clean', radix: 16));
    } catch (_) {
      return const Color(0xFF06B6D4);
    }
  }

  @override
  Widget build(BuildContext context) {
    final allTracks = CurriculumRepository.tracks;

    // CRITICAL: If the repository is empty, show explicit error state
    if (allTracks.isEmpty) {
      return Scaffold(
        backgroundColor: const Color(0xFF0A0E17),
        body: Center(
          child: Padding(
            padding: const EdgeInsets.all(32),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                const Icon(Icons.warning_amber_rounded, color: Color(0xFFF59E0B), size: 56),
                const SizedBox(height: 16),
                const Text(
                  'No Tracks Available',
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 8),
                const Text(
                  'CurriculumRepository.tracks is empty.\n'
                  'The curriculum data may not have loaded correctly.\n'
                  'Restart the app to retry.',
                  textAlign: TextAlign.center,
                  style: TextStyle(
                    color: Color(0xFFFCA5A5),
                    fontSize: 12,
                  ),
                ),
              ],
            ),
          ),
        ),
      );
    }

    final filteredTracks = allTracks.where((t) {
      final matchesSearch = _searchQuery.isEmpty ||
          t.title.toLowerCase().contains(_searchQuery.toLowerCase()) ||
          t.description.toLowerCase().contains(_searchQuery.toLowerCase());
      final matchesDiff = _selectedDifficulty == 'all' ||
          t.difficulty.toLowerCase() == _selectedDifficulty.toLowerCase();
      return matchesSearch && matchesDiff;
    }).toList();

    final totalLessons = allTracks.expand((t) => t.modules).expand((m) => m.lessons).length;
    final totalModules = allTracks.expand((t) => t.modules).length;

    return Scaffold(
      backgroundColor: const Color(0xFF0A0E17),
      body: Column(
        children: [
          // Header with live counts
          Container(
            padding: const EdgeInsets.fromLTRB(16, 12, 16, 0),
            color: const Color(0xFF0F172A),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    const Text(
                      '15 Engineering Tracks',
                      style: TextStyle(
                        fontWeight: FontWeight.w900,
                        letterSpacing: 0.8,
                        fontSize: 17,
                        color: Colors.white,
                      ),
                    ),
                    Container(
                      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                      decoration: BoxDecoration(
                        color: const Color(0xFF06B6D4).withValues(alpha: 0.15),
                        borderRadius: BorderRadius.circular(6),
                        border: Border.all(color: const Color(0xFF06B6D4).withValues(alpha: 0.3)),
                      ),
                      child: Text(
                        '${allTracks.length}T · ${totalModules}M · ${totalLessons}L',
                        style: const TextStyle(
                          fontSize: 10,
                          fontWeight: FontWeight.bold,
                          fontFamily: 'monospace',
                          color: Color(0xFF06B6D4),
                        ),
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 12),
              ],
            ),
          ),

          // Search & Filter Bar
          Container(
            padding: const EdgeInsets.fromLTRB(16, 8, 16, 12),
            color: const Color(0xFF0F172A),
            child: Column(
              children: [
                TextField(
                  onChanged: (val) => setState(() => _searchQuery = val),
                  style: const TextStyle(color: Colors.white, fontSize: 13),
                  decoration: InputDecoration(
                    hintText: 'Search tracks, technologies...',
                    hintStyle: const TextStyle(color: Color(0xFF64748B), fontSize: 13),
                    prefixIcon: const Icon(Icons.search, color: Color(0xFF06B6D4), size: 20),
                    filled: true,
                    fillColor: const Color(0xFF0A0E17),
                    contentPadding: const EdgeInsets.symmetric(vertical: 10),
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(12),
                      borderSide: const BorderSide(color: Color(0xFF1E293B)),
                    ),
                    focusedBorder: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(12),
                      borderSide: const BorderSide(color: Color(0xFF06B6D4)),
                    ),
                  ),
                ),
                const SizedBox(height: 10),
                SizedBox(
                  height: 36,
                  child: ListView(
                    scrollDirection: Axis.horizontal,
                    children: ['all', 'beginner', 'intermediate', 'advanced', 'expert'].map((diff) {
                      final isSelected = _selectedDifficulty == diff;
                      return Padding(
                        padding: const EdgeInsets.only(right: 8),
                        child: ChoiceChip(
                          label: Text(
                            diff.toUpperCase(),
                            style: TextStyle(
                              fontSize: 10,
                              fontWeight: FontWeight.bold,
                              color: isSelected ? Colors.black : const Color(0xFF94A3B8),
                            ),
                          ),
                          selected: isSelected,
                          selectedColor: const Color(0xFF06B6D4),
                          backgroundColor: const Color(0xFF0A0E17),
                          side: BorderSide(
                            color: isSelected
                                ? const Color(0xFF06B6D4)
                                : const Color(0xFF1E293B),
                          ),
                          visualDensity: VisualDensity.compact,
                          materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
                          onSelected: (selected) {
                            if (selected) setState(() => _selectedDifficulty = diff);
                          },
                        ),
                      );
                    }).toList(),
                  ),
                ),
              ],
            ),
          ),

          // Track List or Empty Filter State
          Expanded(
            child: filteredTracks.isEmpty
                ? Center(
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        const Icon(Icons.filter_list_off, color: Color(0xFF64748B), size: 48),
                        const SizedBox(height: 12),
                        Text(
                          _searchQuery.isNotEmpty
                              ? 'No tracks match "$_searchQuery"'
                              : 'No tracks match "$_selectedDifficulty" filter',
                          style: const TextStyle(color: Color(0xFF94A3B8), fontSize: 14),
                          textAlign: TextAlign.center,
                        ),
                        const SizedBox(height: 16),
                        TextButton.icon(
                          onPressed: () {
                            setState(() {
                              _searchQuery = '';
                              _selectedDifficulty = 'all';
                            });
                          },
                          icon: const Icon(Icons.clear_all, size: 18),
                          label: const Text('Clear Filters'),
                          style: TextButton.styleFrom(
                            foregroundColor: const Color(0xFF06B6D4),
                          ),
                        ),
                      ],
                    ),
                  )
                : ListView.builder(
                    padding: const EdgeInsets.all(16),
                    itemCount: filteredTracks.length,
                    itemBuilder: (context, index) {
                      final track = filteredTracks[index];
                      final trackColor = _parseColor(track.colorHex);
                      final progress = _progressMap[track.slug] ?? 0.0;
                      final lessonCount = track.modules.expand((m) => m.lessons).length;

                      return Card(
                        color: const Color(0xFF0F172A),
                        margin: const EdgeInsets.only(bottom: 14),
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(16),
                          side: const BorderSide(color: Color(0xFF1E293B)),
                        ),
                        child: InkWell(
                          borderRadius: BorderRadius.circular(16),
                          onTap: () {
                            Navigator.push(
                              context,
                              MaterialPageRoute(
                                builder: (_) => ModulesScreen(track: track),
                              ),
                            ).then((_) => _loadProgress());
                          },
                          child: Padding(
                            padding: const EdgeInsets.all(18),
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Row(
                                  children: [
                                    Container(
                                      width: 48,
                                      height: 48,
                                      decoration: BoxDecoration(
                                        color: trackColor.withValues(alpha: 0.15),
                                        borderRadius: BorderRadius.circular(12),
                                        border: Border.all(color: trackColor.withValues(alpha: 0.3)),
                                      ),
                                      child: Icon(_getIconData(track.icon), color: trackColor, size: 24),
                                    ),
                                    const SizedBox(width: 14),
                                    Expanded(
                                      child: Column(
                                        crossAxisAlignment: CrossAxisAlignment.start,
                                        children: [
                                          Text(
                                            track.title,
                                            style: const TextStyle(
                                              fontSize: 15,
                                              fontWeight: FontWeight.bold,
                                              color: Colors.white,
                                            ),
                                          ),
                                          const SizedBox(height: 4),
                                          Wrap(
                                            spacing: 6,
                                            runSpacing: 4,
                                            children: [
                                              Container(
                                                padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                                                decoration: BoxDecoration(
                                                  color: const Color(0xFF1E293B),
                                                  borderRadius: BorderRadius.circular(4),
                                                ),
                                                child: Text(
                                                  track.difficulty.toUpperCase(),
                                                  style: const TextStyle(
                                                    fontSize: 9,
                                                    fontWeight: FontWeight.bold,
                                                    color: Color(0xFF94A3B8),
                                                    fontFamily: 'monospace',
                                                  ),
                                                ),
                                              ),
                                              Text(
                                                '$lessonCount Lessons • ${track.totalXp} XP',
                                                style: const TextStyle(fontSize: 11, color: Color(0xFF64748B)),
                                              ),
                                            ],
                                          ),
                                        ],
                                      ),
                                    ),
                                    const Icon(Icons.chevron_right, color: Color(0xFF64748B)),
                                  ],
                                ),
                                const SizedBox(height: 12),
                                Text(
                                  track.description,
                                  style: const TextStyle(fontSize: 12, color: Color(0xFF94A3B8), height: 1.4),
                                  maxLines: 2,
                                  overflow: TextOverflow.ellipsis,
                                ),
                                const SizedBox(height: 12),
                                // Progress bar
                                Row(
                                  children: [
                                    Expanded(
                                      child: ClipRRect(
                                        borderRadius: BorderRadius.circular(4),
                                        child: LinearProgressIndicator(
                                          value: progress,
                                          backgroundColor: const Color(0xFF1E293B),
                                          valueColor: AlwaysStoppedAnimation(trackColor),
                                          minHeight: 4,
                                        ),
                                      ),
                                    ),
                                    const SizedBox(width: 10),
                                    Text(
                                      '${(progress * 100).toInt()}%',
                                      style: TextStyle(
                                        fontSize: 10,
                                        fontWeight: FontWeight.bold,
                                        color: progress > 0 ? trackColor : const Color(0xFF64748B),
                                        fontFamily: 'monospace',
                                      ),
                                    ),
                                  ],
                                ),
                              ],
                            ),
                          ),
                        ),
                      );
                    },
                  ),
          ),
        ],
      ),
    );
  }
}
