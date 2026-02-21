# AGENTS.md

Repository-local operating guide for human contributors and coding agents.

## 1) Project Principles

- Build a **teaching/demo kernel** in pure Rust.
- Favor **clarity and traceability** over clever abstractions.
- Reuse existing standards/tools whenever practical:
  - Linux-compatible syscall ABI subset
  - ELF binaries
  - x86_64 + UEFI boot path
- Avoid overdesign and speculative architecture.

## 2) Multi-Agent Workflow (Required)

For any non-trivial milestone task, run four roles:

1. **Planner/PM Agent**
   - Produces: PRD + MVP + Non-Goals + Acceptance Criteria
2. **Architect Agent**
   - Produces: Design doc including interfaces/CRDs, state machines, failure modes, rollback, compatibility
3. **Implementer Agent (Codex)**
   - Produces: code diff + unit/integration tests and scripts only
4. **Reviewer Agent**
   - Produces: checklist review against acceptance criteria (no code edits)

The orchestrator (human or coordinator agent) must keep status visible and unblock decisions quickly.

## 3) Testing Layers (Required Quality Model)

Use the same layer model for local work, review handoff, and milestone release:

- **Layer A (Fast Local Correctness)**: quick local checks on touched crates and docs.
- **Layer B (Local Integration/Demo Smoke)**: local integration or demo scripts proving the milestone behavior.
- **Layer C (Reviewer Baseline Verification)**: full reviewer baseline run in the defined milestone environment.
- **Layer D (Release Validation)**: release-grade validation for milestone branch readiness.
  - Layer D may run via GitHub Actions on Linux x86_64 (for example, through `workflow_dispatch`).

Required gates:

- Implementer done gate: **Layer A + Layer B pass**.
- Reviewer precondition: **Layer C pass**.
- Milestone release gate: **Layer C + Layer D pass**.

Note: once `cargo xtask` is implemented, prefer unified task entrypoints over ad-hoc per-layer shell invocations.

## 4) Branching & Release Policy

- `main`: ongoing development and integration.
- After each milestone is accepted, create a **milestone release branch**:
  - naming: `release/m{N}-{slug}`
  - must be demo-ready and test-passing
  - should remain stable for teaching/playground use

### Release Gate Policy (Mandatory)

- A milestone cannot be marked complete unless both are available:
  - Layer C pass evidence (reviewer baseline)
  - Layer D pass evidence (GitHub Actions run URL)
- No pass URL, no milestone completion.
- After release gate passes, update `docs/milestones/ROADMAP.md` milestone status to `done` and push that status update to `main`.
- `release/*` branches are treated as frozen snapshots.
  - Allowed changes: `fix(release): ...` and `docs(release): ...` only.
  - Every release branch fix must re-run Layer D and attach the new passing run URL.

### Release Incident Handling (Bad Case Procedure)

When a release gate fails after branch creation:

1. Fix on `main` first.
2. Verify Layer D pass on `main` and capture run URL.
3. Cherry-pick minimal fix commit(s) onto the target `release/*` branch.
4. Re-run Layer D on the release branch and capture run URL.
5. Update milestone review evidence with both run URLs (`main` + `release`).

## 5) Commit Policy

- Use clear, readable commit messages.
- Architect-defined implementation steps should land as separate commits when feasible (one step, one commit).
- Prefer Conventional Commit prefixes: `feat:`, `fix:`, `docs:`, `test:`, `chore:`.

## 6) Code Quality Rules

- Prioritize readability and explicit control flow.
- Avoid meaningless utility layers and premature abstractions.
- Duplicate a small amount of code if it keeps concepts easier to teach.
- Refactor only when repeated patterns are proven and stable.

## 7) Documentation Rules

- All repository files are written in **English**.
- Keep milestone docs concise and machine-readable.
- Each milestone must include a fundamental theory note at `docs/fundamentals/M{N}-{slug}.md`, aligned with PRD/design scope, and indexed in `docs/fundamentals/README.md`.
- If syscall behavior changes, update `docs/abi/linux-syscall-subset.md` in the same change.

## 8) Definition of Done (Per Milestone Task)

A task is done only when all are true:

- PRD and design docs exist and match implementation scope.
- Fundamental theory note exists for the milestone and reflects actual runtime behavior.
- Code builds for touched crates.
- Implementer gate passed: Layer A + Layer B.
- Reviewer precondition passed: Layer C.
- Milestone release gate passed: Layer C + Layer D.
- Reviewer checklist is completed with unresolved issues explicitly tracked.
- Commit history is clear enough for teaching walkthrough.
