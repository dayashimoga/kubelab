#!/usr/bin/env python3
"""
Authoritative Topics Catalog for all 154 KubeLab scenarios.
Contains custom architecture diagrams, markdown, gotchas, and guidance for all 15 tracks.
"""

TOPIC_CATALOG = {}

def register_topic(lab_id, title, summary, diagram, markdown, mistakes, guidance, duration=15, xp=150):
    TOPIC_CATALOG[lab_id] = {
        "title": title,
        "summary": summary,
        "duration_minutes": duration,
        "xp": xp,
        "mermaid_diagram": diagram,
        "content_markdown": markdown,
        "common_mistakes": mistakes,
        "production_guidance": guidance,
        "concepts": [lab_id.split('-')[0], lab_id, "kubernetes", "cloud-native"],
        "prerequisites": []
    }

# Load the comprehensive catalog generator
from kb_catalog_data import populate_all_topics
populate_all_topics(register_topic)
