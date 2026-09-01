import 'package:flutter/material.dart';

class PlatformNotification {
  final String id;
  final String title;
  final String message;
  final String type; // 'incident', 'achievement', 'curriculum', 'system'
  final DateTime timestamp;
  bool isRead;

  PlatformNotification({
    required this.id,
    required this.title,
    required this.message,
    required this.type,
    required this.timestamp,
    this.isRead = false,
  });
}

class NotificationsScreen extends StatefulWidget {
  const NotificationsScreen({super.key});

  @override
  State<NotificationsScreen> createState() => _NotificationsScreenState();
}

class _NotificationsScreenState extends State<NotificationsScreen> {
  final List<PlatformNotification> _notifications = [
    PlatformNotification(
      id: 'n1',
      title: '🚨 CRITICAL INCIDENT SIMULATION',
      message: 'CoreDNS is failing in sandbox environment. Investigate and repair ConfigMap.',
      type: 'incident',
      timestamp: DateTime.now().subtract(const Duration(minutes: 15)),
    ),
    PlatformNotification(
      id: 'n2',
      title: '🏆 Achievement Unlocked: Zero-Trust Master',
      message: 'You completed all 13 Security track declarative labs with 100% grade!',
      type: 'achievement',
      timestamp: DateTime.now().subtract(const Duration(hours: 2)),
    ),
    PlatformNotification(
      id: 'n3',
      title: '📚 New Lab Track Available: Service Mesh',
      message: '11 new Istio labs covering STRICT mTLS, canary 90/10, and circuit breaking.',
      type: 'curriculum',
      timestamp: DateTime.now().subtract(const Duration(days: 1)),
      isRead: true,
    ),
    PlatformNotification(
      id: 'n4',
      title: '⚡ Daily Practice Streak: Day 5',
      message: 'Keep your streak alive! Complete today\'s quiz to earn +200 bonus XP.',
      type: 'system',
      timestamp: DateTime.now().subtract(const Duration(days: 2)),
      isRead: true,
    ),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0A0E17),
      appBar: AppBar(
        backgroundColor: const Color(0xFF0F172A),
        title: const Text(
          'NOTIFICATIONS & ALERTS',
          style: TextStyle(
            fontSize: 15,
            fontWeight: FontWeight.bold,
            letterSpacing: 1.1,
            color: Color(0xFF06B6D4),
          ),
        ),
        actions: [
          IconButton(
            icon: const Icon(Icons.done_all, color: Color(0xFF06B6D4)),
            tooltip: 'Mark all as read',
            onPressed: () {
              setState(() {
                for (var n in _notifications) {
                  n.isRead = true;
                }
              });
            },
          ),
        ],
      ),
      body: _notifications.isEmpty
          ? const Center(
              child: Text('No notifications', style: TextStyle(color: Color(0xFF64748B))),
            )
          : ListView.separated(
              padding: const EdgeInsets.all(16),
              itemCount: _notifications.length,
              separatorBuilder: (_, __) => const SizedBox(height: 12),
              itemBuilder: (context, index) {
                final n = _notifications[index];
                Color badgeColor;
                IconData icon;

                switch (n.type) {
                  case 'incident':
                    badgeColor = const Color(0xFFEF4444);
                    icon = Icons.warning_amber_rounded;
                    break;
                  case 'achievement':
                    badgeColor = const Color(0xFF10B981);
                    icon = Icons.emoji_events;
                    break;
                  case 'curriculum':
                    badgeColor = const Color(0xFF6366F1);
                    icon = Icons.menu_book;
                    break;
                  default:
                    badgeColor = const Color(0xFF06B6D4);
                    icon = Icons.notifications;
                }

                return InkWell(
                  onTap: () {
                    setState(() {
                      n.isRead = true;
                    });
                  },
                  child: Container(
                    padding: const EdgeInsets.all(14),
                    decoration: BoxDecoration(
                      color: n.isRead ? const Color(0xFF0F172A) : const Color(0x991E293B),
                      borderRadius: BorderRadius.circular(8),
                      border: Border.all(
                        color: n.isRead ? const Color(0xFF1E293B) : badgeColor.withValues(alpha: 0.5),
                        width: n.isRead ? 1 : 1.5,
                      ),
                    ),
                    child: Row(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Container(
                          padding: const EdgeInsets.all(8),
                          decoration: BoxDecoration(
                            color: badgeColor.withValues(alpha: 0.15),
                            shape: BoxShape.circle,
                          ),
                          child: Icon(icon, color: badgeColor, size: 20),
                        ),
                        const SizedBox(width: 12),
                        Expanded(
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Row(
                                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                                children: [
                                  Expanded(
                                    child: Text(
                                      n.title,
                                      style: TextStyle(
                                        color: Colors.white,
                                        fontWeight: n.isRead ? FontWeight.normal : FontWeight.bold,
                                        fontSize: 13,
                                      ),
                                    ),
                                  ),
                                  if (!n.isRead)
                                    Container(
                                      width: 8,
                                      height: 8,
                                      decoration: BoxDecoration(
                                        color: badgeColor,
                                        shape: BoxShape.circle,
                                      ),
                                    ),
                                ],
                              ),
                              const SizedBox(height: 6),
                              Text(
                                n.message,
                                style: const TextStyle(color: Color(0xFF94A3B8), fontSize: 12),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                  ),
                );
              },
            ),
    );
  }
}
