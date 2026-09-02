#!/usr/bin/env python3
"""
Bulk Topic Definitions for All Tracks (Tracks 1 through 15)
Every entry contains custom Mermaid diagrams, realistic YAML, commands, gotchas, and production guidance.
"""

def register_bulk_curriculum(reg):
    # Track 1: Linux Remaining
    reg("linux-05-rootless-containers",
        "Rootless Container Architecture & User Namespace Mapping",
        "Run Podman containers in unprivileged user mode with subuid/subgid mapping and slirp4netns.",
        """graph TD
    HostUser["Host Non-Root User (UID: 1000)"] -->|newuidmap| SubUID["subuid Range: 100000-165535"]
    SubUID --> ContainerRoot["Container UID 0 (Mapped to Host UID 1000)"]
    SubUID --> ContainerUser["Container UID 1000 (Mapped to Host UID 101000)"]
    Slirp["slirp4netns / pasta"] --> Net["Unprivileged Network Stack (Tap Device)"]""",
        """# Rootless Container Architecture & User Namespace Mapping

Rootless containers allow unprivileged users to build, run, and manage OCI containers without requiring root access or sudo privileges.

## Architectural Overview & UID Mapping

```mermaid
graph TD
    HostUser["Host Non-Root User (UID: 1000)"] -->|newuidmap| SubUID["subuid Range: 100000-165535"]
    SubUID --> ContainerRoot["Container UID 0 (Mapped to Host UID 1000)"]
    SubUID --> ContainerUser["Container UID 1000 (Mapped to Host UID 101000)"]
    Slirp["slirp4netns / pasta"] --> Net["Unprivileged Network Stack (Tap Device)"]
```

## Key Operational Commands

```bash
# Verify subuid and subgid allocations
grep $(whoami) /etc/subuid /etc/subgid

# Run a rootless container with Podman
podman run -d --name web -p 8080:80 nginx:alpine

# Inspect container user namespace mapping
podman top web user huser
```

## Practical Manifest Implementation

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: rootless-pod
spec:
  securityContext:
    runAsNonRoot: true
    runAsUser: 10001
  containers:
  - name: app
    image: alpine:3.20
    command: ["sh", "-c", "sleep 3600"]
```

## Common Production Gotchas & Anti-Patterns

1. **Missing SubUID Configuration**: Failing to allocate `/etc/subuid` entries prevents non-root users from executing `podman` or `unshare`.
2. **Privileged Port Binding**: Attempting to bind ports `<1024` without configuring `net.ipv4.ip_unprivileged_port_start=80`.
3. **Storage Driver Mismatch**: Using `vfs` instead of `overlay` in rootless mode, causing massive disk consumption.

## Security & Reliability Best Practices

- **Enforce Rootless in CI**: Execute build pipelines and test containers exclusively in rootless Podman environments.
- **Enable Pasta for High Performance**: Use `pasta` network driver over `slirp4netns` for enhanced rootless network throughput.""",
        ["Attempting to bind container ports below 1024 without sysctl unprivileged port tuning.",
         "Missing subuid entries in /etc/subuid preventing user namespace creation.",
         "Trying to access host block devices directly from unprivileged rootless namespaces."],
        "Adopt rootless Podman across CI/CD runners to prevent container breakout privilege escalations to host root.")

    reg("linux-06-multistage-containerfile",
        "Multi-Stage Containerfile Optimization & Distroless Base",
        "Minimize attack surface and image size using multi-stage builds, non-root users, and scratch/distroless bases.",
        """graph LR
    BuildStage["Stage 1: Builder (golang:1.23-alpine)"] -->|Compile Static Binary| Artifact["/app/server (ELF Executable)"]
    Artifact --> FinalStage["Stage 2: Distroless Runtime (gcr.io/distroless/static)"]
    FinalStage --> NonRootUser["USER nonroot:nonroot (UID 65532)"]""",
        """# Multi-Stage Containerfile Optimization & Distroless Base

Multi-stage builds decouple build tools, compilers, and source files from the final production runtime container image.

## Architectural Overview & Image Layering

```mermaid
graph LR
    BuildStage["Stage 1: Builder (golang:1.23-alpine)"] -->|Compile Static Binary| Artifact["/app/server (ELF Executable)"]
    Artifact --> FinalStage["Stage 2: Distroless Runtime (gcr.io/distroless/static)"]
    FinalStage --> NonRootUser["USER nonroot:nonroot (UID 65532)"]
```

## Key Operational Commands

```bash
# Build multi-stage image with Podman
podman build -t myapp:latest -f Containerfile .

# Analyze container image layers and size
podman history myapp:latest

# Scan image for CVE vulnerabilities with Trivy
trivy image --severity HIGH,CRITICAL myapp:latest
```

## Practical Manifest Implementation

```dockerfile
# Multi-stage Containerfile
FROM golang:1.23-alpine AS builder
WORKDIR /src
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -ldflags="-w -s" -o /bin/server .

FROM gcr.io/distroless/static:nonroot
WORKDIR /
COPY --from=builder /bin/server /server
USER 65532:65532
ENTRYPOINT ["/server"]
```

## Common Production Gotchas & Anti-Patterns

1. **Leaving Package Managers in Final Image**: Retaining `apk`, `apt`, or `yum` in production images introduces high CVE surfaces and supply chain risks.
2. **Running as Default Root**: Failing to declare `USER nonroot` before `ENTRYPOINT`.
3. **Improper Layer Caching**: Copying all source code before `go.mod` / `package.json`, busting build layer caching on every commit.

## Security & Reliability Best Practices

- **Use Distroless or Scratch**: Eliminate shells (`/bin/sh`, `/bin/bash`) to prevent attacker shell breakout post-exploitation.
- **Pin Digest Hashes**: Pin base images by cryptographic SHA256 digest (`FROM golang:1.23@sha256:...`).""",
        ["Copying full source tree before dependency files, invalidating layer caching.",
         "Running container as default root UID 0 instead of USER nonroot.",
         "Including build-time secrets or private keys in intermediate image layers without BuildKit secret mounts."],
        "Use multi-stage builds with static distroless bases and non-root users for zero-CVE production deployments.")

    reg("linux-07-systemd-service-quadlet",
        "Systemd Quadlet Container Services & Auto-Restart",
        "Manage declarative container lifecycles via native Systemd Quadlet unit files and journald logging.",
        """graph TD
    Systemd["systemd (init PID 1)"] --> Quadlet["/etc/containers/systemd/web.container"]
    Quadlet --> Generator["quadlet generator -> web.service"]
    Generator --> Podman["podman run lifecycle"]
    Podman --> Journald["systemd-journald (Centralized Logs)"]""",
        """# Systemd Quadlet Container Services & Auto-Restart

Systemd Quadlet is the cloud-native bridge that enables declarative `.container` and `.kube` unit files to be managed by systemd as native system services.

## Architectural Overview

```mermaid
graph TD
    Systemd["systemd (init PID 1)"] --> Quadlet["/etc/containers/systemd/web.container"]
    Quadlet --> Generator["quadlet generator -> web.service"]
    Generator --> Podman["podman run lifecycle"]
    Podman --> Journald["systemd-journald (Centralized Logs)"]
```

## Key Operational Commands

```bash
# Reload systemd daemon to process quadlet unit changes
systemctl daemon-reload

# Start and enable the generated container service
systemctl enable --now web.service

# Inspect live container service logs via journalctl
journalctl -u web.service -f
```

## Practical Manifest Implementation

```ini
# /etc/containers/systemd/api-service.container
[Unit]
Description=Production API Microservice
After=network-online.target

[Container]
Image=quay.io/libpod/banner:latest
PublishPort=8080:80
AutoUpdate=registry
Network=host
Restart=always

[Install]
WantedBy=multi-user.target default.target
```

## Common Production Gotchas & Anti-Patterns

1. **Forgetting daemon-reload**: Making changes to `.container` files without running `systemctl daemon-reload`.
2. **Port Collisions**: Binding container ports that conflict with host daemon ports when using `Network=host`.
3. **Unmanaged Restarts**: Adding custom restart loops in scripts instead of using systemd's built-in `Restart=on-failure`.

## Security & Reliability Best Practices

- **AutoUpdate Registry**: Leverage `AutoUpdate=registry` combined with `podman-auto-update.timer` for automated zero-downtime base patch rolling.
- **Journald Log Ingestion**: Stream container stdout/stderr directly into `systemd-journald` for unified host audit trails.""",
        ["Modifying Quadlet files without executing systemctl daemon-reload.",
         "Confusing Podman Quadlet .container syntax with standard .service syntax.",
         "Not specifying multi-user.target in WantedBy, causing container to fail on boot."],
        "Deploy standalone edge Kubernetes node agents and container services using Quadlet unit files for automatic systemd restart.")

    reg("linux-08-oci-spec-runc",
        "OCI Runtime Specification & Direct runc Container Execution",
        "Inspect OCI config.json, configure namespaces, mounts, capabilities, and execute containers directly with runc.",
        """graph TD
    OCI_Spec["config.json (OCI Runtime Spec)"] --> Runc["runc (Low-Level OCI Runtime)"]
    RootFS["rootfs/ (Root Filesystem Bundle)"] --> Runc
    Runc --> PivotRoot["pivot_root() & chroot()"]
    PivotRoot --> ContainerProcess["Isolated Container Process"]""",
        """# OCI Runtime Specification & Direct runc Container Execution

The Open Container Initiative (OCI) runtime specification defines the exact filesystem bundle layout and JSON configuration schema required to spawn isolated containers.

## Architectural Overview

```mermaid
graph TD
    OCI_Spec["config.json (OCI Runtime Spec)"] --> Runc["runc (Low-Level OCI Runtime)"]
    RootFS["rootfs/ (Root Filesystem Bundle)"] --> Runc
    Runc --> PivotRoot["pivot_root() & chroot()"]
    PivotRoot --> ContainerProcess["Isolated Container Process"]
```

## Key Operational Commands

```bash
# Generate default OCI runtime specification
runc spec

# Inspect OCI configuration capabilities and namespaces
jq .process.capabilities config.json

# Execute container directly from filesystem bundle
runc run mycontainer
```

## Practical Manifest Implementation

```json
{
  "ociVersion": "1.0.2-dev",
  "process": {
    "terminal": false,
    "user": { "uid": 10001, "gid": 10001 },
    "args": [ "/bin/app" ],
    "env": [ "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin" ],
    "cwd": "/",
    "capabilities": {
      "bounding": [ "CAP_NET_BIND_SERVICE" ],
      "effective": [ "CAP_NET_BIND_SERVICE" ],
      "permitted": [ "CAP_NET_BIND_SERVICE" ]
    }
  },
  "root": { "path": "rootfs", "readonly": true }
}
```

## Common Production Gotchas & Anti-Patterns

1. **CAP_SYS_ADMIN Inclusion**: Granting `CAP_SYS_ADMIN` in capabilities, effectively granting root host capabilities.
2. **Writable rootfs**: Setting `readonly: false` in OCI spec, allowing attackers to overwrite runtime binaries.
3. **Mount Leakage**: Omitting `noexec`, `nosuid`, or `nodev` on `/tmp` and ephemeral volume mounts.

## Security & Reliability Best Practices

- **Drop All Capabilities by Default**: Only add explicit required capabilities such as `CAP_NET_BIND_SERVICE`.
- **Set readonly rootfs**: Ensure the root filesystem is mounted read-only (`"readonly": true`).""",
        ["Retaining CAP_SYS_ADMIN or CAP_DAC_OVERRIDE in bounding capabilities.",
         "Missing rootfs directory structure when executing runc run.",
         "Setting readonly: false on rootfs in multi-tenant environments."],
        "Audit low-level CRI-O and containerd configurations to ensure default OCI runtime specs drop all capabilities and enforce read-only rootfilesystems.")

    # Now load and register all 14 remaining tracks with full deep definitions
    from kb_tracks_k8s import register_k8s_track
    from kb_tracks_storage_net import register_storage_and_networking
    from kb_tracks_helm_admin import register_helm_and_admin
    from kb_tracks_sec_gitops import register_sec_and_gitops
    from kb_tracks_mesh_obs import register_mesh_and_obs
    from kb_tracks_trouble_sre import register_trouble_and_sre
    from kb_tracks_platform_incidents import register_platform_and_incidents
    from kb_tracks_cert import register_cert_track

    register_k8s_track(reg)
    register_storage_and_networking(reg)
    register_helm_and_admin(reg)
    register_sec_and_gitops(reg)
    register_mesh_and_obs(reg)
    register_trouble_and_sre(reg)
    register_platform_and_incidents(reg)
    register_cert_track(reg)
