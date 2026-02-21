# Codex Multi-Agent Configuration

This directory contains role prompts, templates, and workflow specs for the 4-agent execution model:

- Planner/PM
- Architect
- Implementer
- Reviewer

## Usage Pattern

1. Planner creates PRD package using `templates/prd-template.md`.
2. Architect creates design package using `templates/design-template.md`.
3. Implementer produces code + tests only.
4. Reviewer validates against acceptance criteria using `templates/review-checklist.md`.

All outputs must stay concise, actionable, and aligned to milestone acceptance criteria.
