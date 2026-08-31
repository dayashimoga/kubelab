# Contributing to KubeLab

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork: `git clone https://github.com/<you>/kubelab.git`
3. Create a feature branch: `git checkout -b feat/my-feature`
4. Make changes and test locally
5. Push and open a Pull Request against `main`

## Code Standards

- **Rust**: `cargo fmt --all` + `cargo clippy -- -D warnings` must pass
- **TypeScript**: `pnpm lint` must pass
- **Flutter**: `flutter analyze --fatal-infos --fatal-warnings` must pass
- **Commits**: Use conventional commits (`feat:`, `fix:`, `docs:`, `ci:`, `refactor:`)

## Pull Request Checklist

- [ ] Code compiles without warnings
- [ ] All existing tests pass
- [ ] New functionality has tests
- [ ] Documentation updated if needed
- [ ] CHANGELOG.md updated for user-facing changes

## Development Setup

See [Developer Guide](DEVELOPER_GUIDE.md) for local setup instructions.

## License

By contributing, you agree that your contributions will be licensed under the Apache 2.0 License.
