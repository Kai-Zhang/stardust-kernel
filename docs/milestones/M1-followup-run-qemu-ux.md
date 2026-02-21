# Follow-up Task — M1 run-qemu UX (Non-blocking)

- Task id: `M1-followup-run-qemu-ux`
- Related milestone: `M1-boot-to-rust`
- Priority: Low (non-blocking)
- Status: Planned

## Goal

Improve `scripts/run-qemu.sh` so it supports both:

1. non-interactive smoke mode with deterministic auto-exit behavior, and
2. interactive demo mode with clear operator control.

## Scope

- Add explicit mode flags (example: `--smoke`, `--interactive`) or env switches.
- Define stop condition in smoke mode (timeout or expected log marker + clean quit path).
- Keep current default behavior backward compatible unless explicitly changed and documented.
- Update `scripts/README.md` with usage examples.

## Acceptance Criteria

- AC1: Smoke mode exits automatically without manual kill in local CI-like runs.
- AC2: Interactive mode still allows manual exploration without premature exit.
- AC3: Script returns non-zero on boot failure and zero on successful smoke completion.
- AC4: README documents both modes and expected outputs.

## Risks

- False positives if stop condition matches too early.
- Host-specific QEMU behavior differences for quit/monitor handling.
- Over-tuning for one platform may reduce portability.

## Out of Scope

- Any kernel functional change for M2.
- Reworking broader build pipeline beyond run script UX.
