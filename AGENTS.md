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

## 3) Branching & Release Policy

- `main`: ongoing development and integration.
- After each milestone is accepted, create a **milestone release branch**:
  - naming: `release/m{N}-{slug}`
  - must be demo-ready and test-passing
  - should remain stable for teaching/playground use

## 4) Commit Policy

- Use clear, readable commit messages.
- Architect-defined implementation steps should land as separate commits when feasible (one step, one commit).
- Prefer Conventional Commit prefixes: `feat:`, `fix:`, `docs:`, `test:`, `chore:`.

## 5) Code Quality Rules

- Prioritize readability and explicit control flow.
- Avoid meaningless utility layers and premature abstractions.
- Duplicate a small amount of code if it keeps concepts easier to teach.
- Refactor only when repeated patterns are proven and stable.

## 6) Documentation Rules

- All repository files are written in **English**.
- Keep milestone docs concise and machine-readable.
- If syscall behavior changes, update `docs/abi/linux-syscall-subset.md` in the same change.

## 7) Definition of Done (Per Milestone Task)

A task is done only when all are true:

- PRD and design docs exist and match implementation scope.
- Code builds for touched crates.
- Tests and demo scripts pass in the defined environment.
- Reviewer checklist is completed with unresolved issues explicitly tracked.
- Commit history is clear enough for teaching walkthrough.
