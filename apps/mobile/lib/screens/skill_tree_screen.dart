import 'package:flutter/material.dart';

class SkillTreeScreen extends StatelessWidget {
  const SkillTreeScreen({super.key});

  final List<Map<String, dynamic>> skills = const [
    {'name': 'Linux & CLI', 'level': 3, 'category': 'Foundations'},
    {'name': 'OCI Containers', 'level': 2, 'category': 'Foundations'},
    {'name': 'Kubernetes Workloads', 'level': 4, 'category': 'Core'},
    {'name': 'Networking & CNI', 'level': 3, 'category': 'Networking'},
    {'name': 'GitOps & Argo CD', 'level': 3, 'category': 'GitOps'},
    {'name': 'Istio Service Mesh', 'level': 2, 'category': 'Service Mesh'},
    {'name': 'OpenTelemetry', 'level': 3, 'category': 'Observability'},
    {'name': 'Incident Response', 'level': 2, 'category': 'SRE'},
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('SKILL TREE DAG'),
        backgroundColor: const Color(0xFF0F172A),
      ),
      body: ListView.builder(
        padding: const EdgeInsets.all(16),
        itemCount: skills.length,
        itemBuilder: (context, index) {
          final s = skills[index];
          final level = s['level'] as int;
          return Card(
            color: const Color(0xFF0F172A),
            margin: const EdgeInsets.only(bottom: 12),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(12),
              side: const BorderSide(color: Color(0xFF1E293B)),
            ),
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        s['name'] as String,
                        style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 14),
                      ),
                      Text(
                        s['category'] as String,
                        style: const TextStyle(color: Color(0xFF94A3B8), fontSize: 11),
                      ),
                    ],
                  ),
                  Row(
                    children: List.generate(
                      5,
                      (i) => Container(
                        width: 8,
                        height: 8,
                        margin: const EdgeInsets.only(left: 4),
                        decoration: BoxDecoration(
                          color: i < level ? const Color(0xFF06B6D4) : const Color(0xFF334155),
                          shape: BoxShape.circle,
                        ),
                      ),
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
