# Contributing

## Scope

This project is a teaching/demo kernel effort. Prefer clarity and correctness over cleverness.

## Getting Started

1. Read `README.md` and `docs/milestones/ROADMAP.md`.
2. Pick a milestone task with clear acceptance criteria.
3. Open a focused PR that addresses one problem.

## Development Expectations

- Write code and docs in English.
- Keep subsystem boundaries explicit.
- Avoid introducing unstable abstractions too early.
- Update ABI docs when syscall-visible behavior changes.

## Pull Request Checklist

- [ ] Changes are aligned with current roadmap milestone.
- [ ] `cargo check --workspace` passes.
- [ ] Relevant tests added/updated.
- [ ] Documentation updated (`README`, ABI, roadmap) if needed.
- [ ] Commit messages are clear and scoped.

## Code Review Priorities

- Behavioral correctness.
- Clarity for teaching use.
- Regression risk.
- Test coverage of changed behavior.
