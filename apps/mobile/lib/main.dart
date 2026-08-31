import 'package:flutter/material.dart';
import 'screens/home_screen.dart';

void main() {
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
        brightness: Brightness.dark,
        scaffoldBackgroundColor: const Color(0xFF0A0E17),
        primaryColor: const Color(0xFF06B6D4),
        colorScheme: const ColorScheme.dark(
          primary: Color(0xFF06B6D4),
          secondary: Color(0xFF6366F1),
          surface: Color(0xFF0F172A),
          background: Color(0xFF0A0E17),
        ),
        fontFamily: 'Roboto',
      ),
      home: const HomeScreen(),
    );
  }
}
