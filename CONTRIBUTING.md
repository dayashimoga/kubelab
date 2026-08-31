# Contributing to KubeLab

Thank you for your interest in contributing to KubeLab!

## Code of Conduct

All contributors must adhere to our standard Code of Conduct: be welcoming, respectful, and constructive.

## Development Setup

KubeLab requires **zero host development dependencies** other than `git` and `podman`.

```bash
# 1. Clone repository
git clone https://github.com/kubelab/kubelab.git
cd kubelab

# 2. Run system doctor
./scripts/doctor.ps1 # or ./scripts/doctor.sh

# 3. Start local development stack
./scripts/dev.ps1 # or ./scripts/dev.sh
```

## Pull Request Guidelines

1. Ensure all tests pass: `./scripts/test.ps1`
2. Run format & lint checks: `./scripts/validate.ps1`
3. Include tests for all new features or bug fixes.
4. Maintain >90% code coverage.
5. Provide clear PR descriptions linking relevant issues.
