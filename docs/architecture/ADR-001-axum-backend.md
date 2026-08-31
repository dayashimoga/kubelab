# ADR 001: Adoption of Rust & Axum for Core Platform Backend

## Status
Accepted

## Context
KubeLab requires high-concurrency WebSocket terminal multiplexing, low memory footprint when orchestrating container/cluster workloads, zero runtime garbage collection pauses during live exam grading, and strict memory safety.

## Decision
We adopt **Rust 2021 edition** with the **Axum / Tokio / Tower** asynchronous ecosystem for all backend services, validation engine, and lab orchestration controllers.

## Consequences
- **Pros:** Sub-millisecond latency, minimal memory overhead (< 25MB per service container), robust compile-time safety, seamless async concurrency with Tokio.
- **Cons:** Longer initial build times (mitigated by multi-stage OCI caching and workspace dependency sharing).
