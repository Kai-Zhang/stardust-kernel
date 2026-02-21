# Milestone Orchestration Workflow

## Purpose
Provide a deterministic 4-agent flow for each milestone.

## Steps

1. Planner/PM
   - Read roadmap milestone entry.
   - Produce `docs/specs/<milestone-id>/prd.md`.
   - Gate: PRD accepted by coordinator.

2. Architect
   - Produce `docs/specs/<milestone-id>/design.md`.
   - Include commit-sized step plan.
   - Gate: design accepted by coordinator.

3. Implementer
   - Implement design steps with one commit per step where feasible.
   - Add tests/scripts proving acceptance criteria.
   - Gate: CI/tests/demo script pass.

4. Reviewer
   - Produce `docs/reviews/<milestone-id>/review.md`.
   - Compare implementation to acceptance checklist.
   - Gate: no unresolved blockers.

5. Milestone Release Branch
   - Create `release/m{N}-{slug}` from accepted state.
   - Tag optional demo point.

## Coordinator Responsibilities
- Track agent status and blockers.
- Escalate unanswered questions immediately.
- Keep progress visible with short status updates.
