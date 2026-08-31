# KubeLab Release Guide

## Versioning
KubeLab uses [Semantic Versioning](https://semver.org/): `MAJOR.MINOR.PATCH`

## Release Process

### 1. Ensure CI passes on main
All jobs in `ci.yml` must pass, including `production-validation`.

### 2. Update version
- `package.json` → `version`
- `Cargo.toml` → `workspace.package.version`
- `apps/mobile/pubspec.yaml` → `version`

### 3. Update CHANGELOG.md
Add a new version section with all changes since the last release.

### 4. Commit and tag
```bash
git add -A
git commit -m "release: v1.2.0"
git tag v1.2.0
git push origin main --tags
```

### 5. Automated release
`release.yml` triggers on the tag push and:
1. Downloads APK/AAB artifacts from the CI run for this commit
2. Creates a GitHub Release with auto-generated release notes
3. Attaches `kubelab-android-1.2.0.apk` and `.aab` to the release

### 6. Post-release
- Verify the GitHub Release page has correct artifacts
- Update documentation if needed
- Announce the release

## Artifact Publishing

| Artifact | Source | Destination |
|---|---|---|
| Android APK | `mobile-android` CI job | GitHub Release attachment |
| Android AAB | `mobile-android` CI job | GitHub Release attachment |
| Container images | `container-build` CI job | (Future: container registry) |
| iOS IPA | Requires Apple Developer credentials | (Future: TestFlight) |

## iOS Distribution
Signed IPA distribution requires:
- Apple Developer Program membership
- Provisioning profiles and certificates
- Configured in `apps/mobile/ios/` Xcode project

The CI pipeline builds an **unsigned simulator build** to prove source buildability. Signed distribution is a separate, explicitly configured capability.
