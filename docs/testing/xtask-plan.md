# Xtask Plan (Design Only)

This document defines a minimal, teaching-friendly `cargo xtask` command surface.

Status: design only. No `xtask` crate or Rust implementation exists yet.

## Goals

- Provide one predictable entrypoint for test layers and QEMU demo runs.
- Keep behavior close to current script and CI flow.
- Make local and CI output easy to read and parse.

## Planned Command Surface

All commands are invoked as `cargo xtask <command> [options]`.

### Layer Commands

- `cargo xtask layer-a`
  - Runs fast local correctness checks (fmt, clippy, check).
- `cargo xtask layer-b`
  - Runs Layer B script wrapper (`scripts/test-layer-b.sh`) when present.
- `cargo xtask layer-c`
  - Runs Layer C script wrapper (`scripts/test-layer-c.sh`) when present.
- `cargo xtask layer-d`
  - Runs Layer D script wrapper (`scripts/test-layer-d.sh`) when present.
  - If script is missing, uses baseline Layer D policy from A+B+C result model.

### QEMU Run Commands

- `cargo xtask qemu-run`
  - Starts the default local QEMU run flow for milestone demo/manual checks.
- `cargo xtask qemu-smoke`
  - Runs deterministic QEMU smoke flow used by Layer C scripts.

## Expected Behavior

### Common

- Print a short start banner: layer/command name and key options.
- Stream subprocess output live.
- Print a final single-line result summary: `PASS`, `FAIL`, or `SKIP`.
- Return non-zero on contract violations or failed checks.

### Script Wrapping Contract (`scripts/`)

- Wrapper targets:
  - `scripts/test-layer-b.sh`
  - `scripts/test-layer-c.sh`
  - `scripts/test-layer-d.sh`
- If executable, run directly.
- If file exists but is not executable, run with `bash`.
- If missing:
  - default: print `SKIP` and exit success for `layer-b` and `layer-c`
  - `layer-d`: print baseline-policy note and continue with A+B+C semantics
  - optional strict mode (future): fail on missing script

### QEMU Contract

- Resolve required host tools (`qemu-system-x86_64`, OVMF path when needed) before launch.
- Emit clear timeout/marker messages for smoke mode.
- Preserve logs in `logs/` when enabled by flags or CI mode.

## Input Contract

Minimal initial options:

- `--ci`
  - CI-safe output (stable summaries, no interactive prompts).
- `--timeout-sec <N>`
  - Applies to QEMU-related commands and any script run that supports timeout.
- `--strict`
  - Treat missing optional scripts as failure.
- `--help`
  - Show usage and exit success.

## Output Contract

- Human-readable progress lines.
- Final machine-friendly summary line format:
  - `XTASK RESULT command=<name> status=<PASS|FAIL|SKIP> code=<n>`
- Optional logs/artifacts placement:
  - `logs/`
  - `artifacts/`

## Exit/Failure Codes

- `0`: success.
- `2`: invalid CLI usage or unknown command.
- `10`: required external tool missing (for example QEMU/OVMF).
- `20`: layer check failed (`fmt`/`clippy`/`check` or script returned non-zero).
- `30`: expected script missing in strict mode.
- `40`: QEMU run failed or timed out.

## Examples

```bash
# Layer checks
cargo xtask layer-a
cargo xtask layer-b
cargo xtask layer-c --strict
cargo xtask layer-d --ci

# QEMU flows
cargo xtask qemu-run
cargo xtask qemu-smoke --timeout-sec 90
```

## CI Mapping (`.github/workflows/layer-d.yml`)

Planned replacement of direct shell steps:

- Current: Layer A inline cargo commands
  - Planned: `cargo xtask layer-a --ci`
- Current: optional `scripts/test-layer-b.sh`
  - Planned: `cargo xtask layer-b --ci`
- Current: optional `scripts/test-layer-c.sh`
  - Planned: `cargo xtask layer-c --ci`
- Current: optional `scripts/test-layer-d.sh`
  - Planned: `cargo xtask layer-d --ci`

This keeps CI behavior aligned with local entrypoints while still using `scripts/` internally.
