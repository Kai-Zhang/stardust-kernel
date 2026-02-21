# Full Repository Review — 2026-02-21

## Executive Summary

Review scope covered kernel/user Rust code, build/test scripts, CI workflow, ABI docs, milestone specs/reviews, and roadmap/release policy.

Local verification performed:
- `cargo fmt --all -- --check` (pass)
- `cargo clippy --workspace --all-targets -- -D warnings` (pass)
- `cargo check --workspace` (pass)
- `cargo test -p stardust-kernel --lib` (pass)
- `scripts/test-layer-b.sh` (pass)
- `scripts/test-layer-c.sh` (pass)
- `scripts/test-layer-d.sh` (pass)

Findings summary:
- High: 2
- Medium: 4
- Low: 3

Primary risks are an ABI correctness bug in synthetic `mmap` allocation, release-evidence policy drift for milestones marked done, and CI/reproducibility fragility (nightly drift + hard-coded short QEMU timeout).

## High Severity Findings

### FR-001
ID: FR-001  
Severity: High  
Location: `kernel/src/userspace.rs:21`, `kernel/src/userspace.rs:95`, `kernel/src/userspace.rs:99`, `kernel/src/userspace.rs:317`  
Evidence: `next_mmap` is always advanced by fixed `MMAP_STRIDE` (`0x10_000`) regardless of requested mapping length. Because `alloc_mapping(len)` stores arbitrary `len`, two successful mappings can overlap when `len > MMAP_STRIDE` (for example: first `len=0x20000` at `0x40000000`, next at `0x40010000`). This violates non-overlap expectations for `mmap` results. (Inference from code path.)  
Recommendation: Advance `next_mmap` by page-aligned requested length (plus guard gap if desired), validate overflow, and add explicit overlap-prevention tests (including large-length cases).

### FR-002
ID: FR-002  
Severity: High  
Location: `docs/milestones/ROADMAP.md:14`, `docs/milestones/ROADMAP.md:39`, `docs/milestones/ROADMAP.md:54`, `docs/milestones/ROADMAP.md:69`, `docs/reviews/M1-boot-to-rust/review.md:1`, `docs/reviews/M2-memory-interrupt-foundation/review.md:1`, `docs/reviews/M2b-timer-irq-controller-integration/review.md:1`  
Evidence: Roadmap policy states milestones can be `done` only with Layer C evidence plus Layer D pass URLs for `main` and `release/*`. M1/M2/M2b are marked `done`, but their review files do not contain Layer D URL evidence sections, while M3/M4 do.  
Recommendation: Either backfill required Layer D evidence for M1/M2/M2b or change status from `done` until evidence is attached. Add an automated docs gate that checks required URL fields before allowing milestone `done` state.

## Medium Severity Findings

### FR-003
ID: FR-003  
Severity: Medium  
Location: `rust-toolchain.toml:2`, `.github/workflows/layer-d.yml:51`  
Evidence: Toolchain is set to rolling `nightly` with no date pin in both local and CI flows. This introduces non-deterministic breakage risk for release validation over time.  
Recommendation: Pin a dated nightly (for example `nightly-YYYY-MM-DD`), then update on a controlled cadence with explicit PRs.

### FR-004
ID: FR-004  
Severity: Medium  
Location: `scripts/test-layer-c.sh:14`, `scripts/test-layer-c.sh:15`, `scripts/test-layer-c.sh:19`  
Evidence: Layer C uses a hardcoded 8-second timeout and swallows timeout exceptions, then relies on grep markers. Slow hosts (especially TCG-heavy or loaded CI machines) can fail spuriously despite correct behavior.  
Recommendation: Make timeout configurable (env/flag), raise default, and log whether timeout occurred to distinguish real boot failures from slow-start flakes.

### FR-005
ID: FR-005  
Severity: Medium  
Location: `.github/workflows/layer-d.yml:62`, `scripts/test-layer-b.sh:4`, `kernel/src/main.rs:18`  
Evidence: Layer A/B primarily lint/test the library path; the UEFI entry binary path is built but not strongly linted in normal Layer A checks. This can miss target-specific quality regressions in `efi_main`-path logic.  
Recommendation: Add explicit target/bin lint checks into Layer A (for example `cargo clippy -p stardust-kernel --target x86_64-unknown-uefi --bin stardust-kernel -- -D warnings`) and keep it in CI.

### FR-006
ID: FR-006  
Severity: Medium  
Location: `docs/specs/M2-memory-interrupt-foundation/prd.md:63`, `docs/specs/M4-syscalls-b-and-doc-hardening/design.md:77`, `kernel/src/main.rs:74`, `scripts/test-layer-c.sh:19`  
Evidence: M2 acceptance/docs still describe `m2:*` readiness markers and later docs claim preserving earlier markers, but current runtime output from `scripts/test-layer-c.sh` run shows `m2b:*`, `m3:*`, `m4:*` markers only. This is a doc/runtime contract drift and can break historical acceptance reproducibility.  
Recommendation: Either restore legacy `m2:*` marker emission or formally deprecate/update the acceptance/docs/review narrative and checks to match current outputs.

## Low Severity Findings

### FR-007
ID: FR-007  
Severity: Low  
Location: `docs/README.md:6`, `docs/README.md:7`, `docs/README.md:8`  
Evidence: Documentation index references `teaching/*` paths that do not exist in current tree; actual location is `docs/fundamentals/*`.  
Recommendation: Update links to current paths to avoid onboarding confusion and broken navigation.

### FR-008
ID: FR-008  
Severity: Low  
Location: `docs/specs/M1-boot-to-rust/prd.md:4`, `docs/specs/M2-memory-interrupt-foundation/prd.md:4`, `docs/specs/M3-userspace-and-syscalls-a/prd.md:4`, `docs/specs/M4-syscalls-b-and-doc-hardening/prd.md:4`  
Evidence: Milestones are marked final/done in roadmap, but PRD/design files remain `Status: Draft`.  
Recommendation: Mark accepted specs as `Final` (or equivalent) with last-reviewed date to reduce process ambiguity.

### FR-009
ID: FR-009  
Severity: Low  
Location: `docs/specs/M3-userspace-and-syscalls-a/prd.md:44`, `docs/abi/linux-syscall-subset.md:35`, `kernel/src/userspace.rs:162`  
Evidence: M3 PRD says invalid write buffer length (`len==0`) returns `-EFAULT`, while ABI doc and implementation model `-EFAULT` for null buffer. This is a spec inconsistency.  
Recommendation: Reconcile PRD wording with ABI+implementation contract and keep one canonical syscall error definition.

## Test Coverage Gaps

1. No tests for `mmap` non-overlap guarantees with large lengths or stride-boundary cases (`kernel/src/userspace.rs:95`).
2. No tests for mapping-table exhaustion edge behavior and address monotonicity invariants in one scenario suite (`kernel/src/userspace.rs:95`).
3. No tests for frame allocator handling of duplicate/overlapping memory regions or documented allocation-order contract (`kernel/src/memory.rs:34`, `docs/specs/M2-memory-interrupt-foundation/design.md:33`).
4. No unit tests in user crate despite being ABI contract carrier (`user/src/lib.rs:1`).
5. Layer C script behavior lacks dedicated tests for timeout/no-marker diagnostics and failure-mode clarity (`scripts/test-layer-c.sh:14`).

## Release/CI Risks

1. Rolling nightly (unpinned) can fail previously green milestones without code changes (`rust-toolchain.toml:2`, `.github/workflows/layer-d.yml:51`).
2. Milestone status policy is not enforced automatically; historical milestones can remain `done` without complete Layer D URL evidence (`docs/milestones/ROADMAP.md:14`).
3. Layer C timeout is currently brittle for slower QEMU environments and may introduce false negatives (`scripts/test-layer-c.sh:14`).
4. Layer D script currently re-runs B+C but adds little unique validation signal (`scripts/test-layer-d.sh:4`).

## Suggested Next Actions (prioritized)

1. Fix FR-001 first: harden synthetic `mmap` allocator to prevent overlapping returned ranges; add targeted tests.
2. Resolve FR-002: backfill or correct milestone evidence/status for M1/M2/M2b to match release policy.
3. Pin toolchain versions (FR-003) in both repo and CI; introduce periodic update workflow.
4. Stabilize Layer C runtime (FR-004) with configurable timeout and explicit timeout diagnostics.
5. Add target-specific lint gate for UEFI bin path (FR-005) and keep it mandatory in CI.
6. Reconcile doc/runtime marker and syscall-spec drift (FR-006/FR-009), then refresh affected review docs.

## Remediation Status (updated)

- Status: Completed on `main`
- Scope: FR-001 through FR-009 addressed in this remediation series.

### Validation snapshot

- `scripts/test-layer-d.sh` => PASS (includes Layer B/C pass)
