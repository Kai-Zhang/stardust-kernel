# Layer D CI Workflow

This repository provides a GitHub Actions workflow for Layer D validation on Linux x86_64:

- Workflow file: `.github/workflows/layer-d.yml`
- Trigger methods:
  - `workflow_dispatch` (manual)
  - Push/PR on `main` and `release/**`

## Intended Role

Layer D is the release-quality gate for milestone branches. It should provide reproducible logs/artifacts and a clear pass/fail outcome.

## Current Behavior

The workflow runs:

1. Environment setup (QEMU + OVMF + Rust nightly)
2. Layer A checks (`fmt`, `clippy`, `check`)
3. Optional scripts if present:
   - `scripts/test-layer-b.sh`
   - `scripts/test-layer-c.sh`
   - `scripts/test-layer-d.sh`
4. Artifact upload from `artifacts/`, `logs/`, and `*.log`

## How to Trigger Manually

Using GitHub UI:

1. Open **Actions** → **Layer D Release Validation**
2. Click **Run workflow**
3. Optional: set `milestone` and `ref`

Using GitHub CLI:

```bash
gh workflow run "Layer D Release Validation" --ref main -f milestone=M1-boot-to-rust
```

## Next Step (Recommended)

As milestone scripts become available, implement deterministic checks in:

- `scripts/test-layer-b.sh`
- `scripts/test-layer-c.sh`
- `scripts/test-layer-d.sh`

Then make milestone release branch creation depend on a green Layer D run.
