#!/usr/bin/env python3
"""
Comprehensive Topic Definition Generator for all 154 KubeLab Scenarios
Generates individual, domain-specific architectures, Mermaid diagrams, manifests, gotchas,
and production guidance for all 15 tracks.
"""

def populate_all_topics(reg):
    # Track 1: Linux & Containers (8 labs)
    reg("linux-03-namespaces-isolation", 
        "Linux Namespaces & Container Process Isolation",
        "Explore PID, NET, MNT, IPC, UTS, and USER kernel namespaces using unshare and nsenter.",
        """graph LR
    Host["Linux Host Kernel"] --> NS_PID["PID Namespace (Isolated PID 1)"]
    Host --> NS_NET["NET Namespace (veth & loopback)"]
    Host --> NS_MNT["MNT Namespace (pivot_root)"]
    Host --> NS_USER["USER Namespace (subuid mapping)"]
    Container["Container Workload"] --> NS_PID
    Container --> NS_NET
    Container --> NS_MNT
    Container --> NS_USER""",
        """# Linux Namespaces & Container Process Isolation

Linux namespaces are the fundamental kernel mechanism providing virtualized isolation for running container processes.

## Architectural Overview & Kernel Subsystems

When a container runtime creates a container, it invokes `clone(2)` with flags corresponding to specific namespace subsystems:

```mermaid
graph LR
    Host["Linux Host Kernel"] --> NS_PID["PID Namespace (Isolated PID 1)"]
    Host --> NS_NET["NET Namespace (veth & loopback)"]
    Host --> NS_MNT["MNT Namespace (pivot_root)"]
    Host --> NS_USER["USER Namespace (subuid mapping)"]
    Container["Container Workload"] --> NS_PID
    Container --> NS_NET
    Container --> NS_MNT
    Container --> NS_USER
```

## Key Operational Commands

```bash
# Inspect all namespaces active on the host
lsns -t pid,net,mnt

# Execute a command inside a running container's network namespace
nsenter --target <PID> --net ip addr show

# Spawn an isolated bash shell with new UTS, PID, and Mount namespaces
unshare --uts --pid --mount --fork /bin/bash
```

## Practical Manifest Implementation

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: isolated-workload
  labels:
    tier: compute
spec:
  hostNetwork: false
  hostPID: false
  hostIPC: false
  containers:
  - name: worker
    image: alpine:3.20
    command: ["sh", "-c", "sleep 3600"]
```

## Common Production Gotchas & Anti-Patterns

1. **Inadvertent Host Namespace Sharing**: Enabling `hostPID: true` or `hostNetwork: true` allows containers to inspect host processes or bind privileged host ports.
2. **Zombie Process Accumulation in PID Namespaces**: Failing to reap child processes when PID 1 inside the namespace ignores `SIGCHLD`.
3. **Mount Leakage**: Mounting host paths without `shared` or `private` propagation flags leading to unmounted filesystem traps.

## Security & Reliability Best Practices

- **Block Host Namespaces**: Restrict `hostNetwork`, `hostPID`, and `hostIPC` using Pod Security Standards `Baseline` or `Restricted`.
- **Adopt User Namespaces**: Utilize Kubernetes User Namespaces (`hostUsers: false`) to map container root (UID 0) to unprivileged host UIDs (e.g. UID 100000).""",
        ["Enabling hostNetwork: true, exposing node-level network interfaces to container escape vulnerabilities.",
         "Neglecting PID 1 zombie process reaping in custom container entrypoints.",
         "Unsharing mount namespace without marking existing mounts as private."],
        "Enforce Pod Security Standards 'Restricted' cluster-wide to reject any pods requesting host namespaces.")

    reg("linux-04-cgroupsv2-limits",
        "Control Groups v2 (cgroups v2) CPU & Memory Throttling",
        "Inspect and configure unified cgroups v2 hierarchy for memory.max, cpu.max, and OOM killer protection.",
        """graph TD
    ParentCgroup["/sys/fs/cgroup/kubepods.slice"] --> PodCgroup["kubepods-burstable.slice/pod-1234"]
    PodCgroup --> MemoryMax["memory.max: 256M"]
    PodCgroup --> CpuMax["cpu.max: 50000 100000 (0.5 CPU)"]
    PodCgroup --> MemoryCurrent["memory.current: 180M (Healthy)"]
    MemoryCurrent -->|Exceeds 256M| OOM["Kernel OOM-Killer Invoked (Exit 137)"]""",
        """# Control Groups v2 (cgroups v2) CPU & Memory Throttling

Control Groups v2 provides a unified single-hierarchy resource control system in the Linux kernel for managing CPU, memory, I/O, and PIDs.

## Architectural Overview & Single Hierarchy

```mermaid
graph TD
    ParentCgroup["/sys/fs/cgroup/kubepods.slice"] --> PodCgroup["kubepods-burstable.slice/pod-1234"]
    PodCgroup --> MemoryMax["memory.max: 256M"]
    PodCgroup --> CpuMax["cpu.max: 50000 100000 (0.5 CPU)"]
    PodCgroup --> MemoryCurrent["memory.current: 180M (Healthy)"]
    MemoryCurrent -->|Exceeds 256M| OOM["Kernel OOM-Killer Invoked (Exit 137)"]
```

## Key Operational Commands

```bash
# Verify system is running unified cgroups v2
stat -fc %T /sys/fs/cgroup/

# Inspect memory limits on container slice
cat /sys/fs/cgroup/kubepods.slice/memory.max

# Check current CPU throttling statistics
cat /sys/fs/cgroup/kubepods.slice/cpu.stat
```

## Practical Manifest Implementation

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: cgroup-bounded-pod
spec:
  containers:
  - name: app
    image: alpine:3.20
    command: ["sh", "-c", "sleep 3600"]
    resources:
      requests:
        cpu: "250m"
        memory: "128Mi"
      limits:
        cpu: "500m"
        memory: "256Mi"
```

## Common Production Gotchas & Anti-Patterns

1. **Unset Memory Limits**: Omitting memory limits causes container memory leaks to consume node-level RAM, triggering node-wide OOM panics.
2. **Aggressive CPU CFS Throttling**: Setting CPU limits too tight on multi-threaded runtimes (e.g. JVM/Go) causes latency spikes even when node CPU is idle.
3. **Ignoring Pressure Stall Information (PSI)**: Not monitoring `/proc/pressure/memory` before OOM events occur.

## Security & Reliability Best Practices

- **Set Memory Requests Equal to Limits**: For mission-critical workloads, configure Guaranteed QoS by matching requests and limits.
- **Utilize CPU Requests for Scheduling**: Tune CPU requests for capacity planning while avoiding unnecessarily restrictive CPU limits unless multi-tenant noisy-neighbor isolation is required.""",
        ["Assuming CPU throttling kills the container (only memory.max triggers SIGKILL OOM 137; CPU throttling only pauses CFS cycles).",
         "Setting memory limits lower than application baseline runtime footprint.",
         "Confusing cgroups v1 multi-hierarchy paths with v2 unified paths."],
        "Monitor memory pressure metrics (container_memory_working_set_bytes) and avoid setting overly rigid CPU limits on bursty web workloads.")

    # Populate remaining 150 labs programmatically with tailored domain knowledge
    _populate_remaining_tracks(reg)

def _populate_remaining_tracks(reg):
    from kb_tracks_bulk import register_bulk_curriculum
    register_bulk_curriculum(reg)
