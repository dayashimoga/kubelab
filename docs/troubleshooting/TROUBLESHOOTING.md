# Troubleshooting & Diagnostic Guide

## Common Issues & Resolutions

### 1. Podman socket connection error
**Symptom:** `Cannot connect to Podman daemon`
**Fix:** Run `podman machine start` or verify Podman service is running.

### 2. Sandbox namespace quota exceeded
**Symptom:** `0/1 nodes available: insufficient cpu/memory`
**Fix:** Run `./scripts/clean.ps1` to purge orphaned lab containers.
