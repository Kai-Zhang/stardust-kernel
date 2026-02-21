# Milestone Theory Note Template

Use this template for each milestone to provide a teaching-oriented theory companion to PRD/design/review docs.

Recommended file name:

- `docs/fundamentals/M{N}-{slug}.md`

## 0) Header

- Title: `M{N} Theory Note: <Short Milestone Name>`
- Related milestone id: ``M{N}-...``
- Links:
  - `docs/specs/<milestone>/prd.md`
  - `docs/specs/<milestone>/design.md`
  - `docs/reviews/<milestone>/review.md` (if available)

## 1) What This Milestone Proves

Explain in 4-8 lines:

- The runtime capability demonstrated.
- What "done" looks like in logs/behavior.
- Why it matters before the next milestone.

## 2) Required Concepts (Minimal Theory)

Teach only concepts required to understand this milestone.

For each concept, include:

1. Plain-language explanation (2-5 lines).
2. Where it appears in our repository (file path).
3. Why it matters in this milestone.

## 3) Execution Path (Step by Step)

Provide a numbered control-flow path from command/start condition to milestone outcome.

Requirements:

- Use concrete file/script names.
- Keep steps chronological.
- Include the exact handoff point into Rust/kernel code.

## 4) Scope Boundaries (In Scope vs Deferred)

Document:

1. What this milestone intentionally includes.
2. What it explicitly defers.
3. Why deferral is useful for learning and risk control.

Link deferred work to roadmap entries in `docs/milestones/ROADMAP.md`.

## 5) Observability and Validation

Show learners how to verify behavior locally.

Include:

- Commands to run.
- Expected success markers.
- Common failure points and first files to inspect.

## 6) Mapping Table: Concept -> File

Add a compact table:

| Concept | File(s) |
| --- | --- |
| `<concept>` | `<path>` |

This is the quickest way for new contributors to move from theory to code.

## 7) Keep-It-Practical Checklist

Before finalizing, verify:

- English only.
- Friendly step-by-step tone.
- No unnecessary academic background.
- Every major concept links to concrete repo files.
- Content matches current implementation, not aspirational behavior.
