# Test Layers for macOS-First Development

This document defines the A/B/C/D test layers used by the multi-agent workflow.

## Goals

- Keep local feedback fast for daily development.
- Preserve reproducible milestone validation.
- Separate implementer completion from release readiness.

## Layer A — Fast Local Correctness

- **Purpose**: Catch obvious regressions quickly.
- **Environment**: local development machine (macOS is acceptable).
- **Typical checks**:
  - `cargo check --workspace`
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets`
  - lightweight doc consistency checks
- **Pass criteria**: all selected checks pass on touched scope.
- **Owner**: Implementer.

## Layer B — Local Integration/Demo Smoke

- **Purpose**: Verify milestone behavior works end-to-end at a basic level.
- **Environment**: local machine and/or containerized test harness.
- **Typical checks**:
  - focused unit tests for changed modules
  - integration smoke script(s)
  - minimal demo script proving expected behavior
- **Pass criteria**: smoke scenarios pass with stable, repeatable output.
- **Owner**: Implementer.

## Layer C — Reviewer Baseline Verification

- **Purpose**: Provide independent verification before review sign-off.
- **Environment**: milestone-defined baseline (for kernel milestones, usually local QEMU x86_64 + UEFI path).
- **Typical checks**:
  - boot/run script with deterministic output markers
  - milestone acceptance criteria mapped to explicit evidence
  - regression checks for known failure paths
- **Pass criteria**: reviewer can reproduce baseline run and validate criteria evidence.
- **Owner**: Reviewer (with coordinator visibility).

## Layer D — Release Validation

- **Purpose**: Gate milestone release branches.
- **Environment**: Linux x86_64 CI (preferred GitHub Actions via `workflow_dispatch`).
- **Typical checks**:
  - full milestone validation pipeline
  - artifact and log capture for teaching replay
  - release branch sanity checks
- **Pass criteria**: CI workflow green with retained logs/artifacts and no unresolved blockers.
- **Owner**: Coordinator + Reviewer.

## Required Gates

- Implementer done gate: **Layer A + Layer B pass**.
- Reviewer precondition: **Layer C pass**.
- Milestone release gate: **Layer C + Layer D pass**.

## Planned `cargo xtask` Mapping

The repository will expose unified task entrypoints through `cargo xtask` once implemented.

- Layer A: `cargo xtask layer-a`
- Layer B: `cargo xtask layer-b`
- Layer C: `cargo xtask layer-c`
- Layer D: `cargo xtask layer-d`
- QEMU demo flow: `cargo xtask qemu-run`
- QEMU deterministic smoke flow: `cargo xtask qemu-smoke`

Command contracts, exit codes, and CI mapping are defined in `docs/testing/xtask-plan.md`.

## Recommended Cadence (macOS-First)

- **Per commit**: Layer A.
- **Per implementation step / before PR update**: Layer B.
- **Before reviewer handoff**: Layer C.
- **Before creating/updating milestone release branch**: Layer D.

## Notes

- Docker is recommended for reproducible host-tooling checks in Layer B.
- QEMU x86_64 on Apple Silicon may be slower (TCG translation), so keep Layer C scenarios focused and deterministic.
- Treat Layer D as the final release-quality source of truth.
