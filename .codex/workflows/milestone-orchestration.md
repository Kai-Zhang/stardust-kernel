# Milestone Orchestration Workflow

## Purpose
Provide a deterministic 4-agent flow for each milestone.

## Testing Layer Model

- **Layer A**: fast local correctness checks on touched crates/docs.
- **Layer B**: local integration/demo smoke checks for milestone behavior.
- **Layer C**: reviewer baseline verification in the defined milestone environment.
- **Layer D**: release validation for milestone branch readiness (can run in GitHub Actions on Linux x86_64).

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
   - Required gate: **Layer A + Layer B pass**.

4. Reviewer
   - Produce `docs/reviews/<milestone-id>/review.md`.
   - Compare implementation to acceptance checklist.
   - Required precondition: **Layer C pass**.
   - Gate: no unresolved blockers in review report.

5. Milestone Release Branch
   - Create `release/m{N}-{slug}` from accepted state.
   - Required gate: **Layer C + Layer D pass**.
   - Tag optional demo point.

## Layer D Execution Paths

- Preferred: GitHub Actions on Linux x86_64 via `workflow_dispatch`.
- Alternative: equivalent Linux x86_64 CI runner with recorded logs/artifacts.

## Coordinator Responsibilities
- Track agent status and blockers.
- Escalate unanswered questions immediately.
- Keep progress visible with short status updates.
- Ensure evidence exists for Layer A/B/C/D gate outcomes.
