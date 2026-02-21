# Stardust Kernel

[![Layer D Release Validation](https://github.com/Kai-Zhang/stardust-kernel/actions/workflows/layer-d.yml/badge.svg)](https://github.com/Kai-Zhang/stardust-kernel/actions/workflows/layer-d.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/Rust-nightly-orange)](https://www.rust-lang.org/)

A modern, milestone-driven educational kernel in Rust.

Stardust Kernel is built for people who want to learn kernel engineering through **real implementation steps**, not toy snippets. The project emphasizes clear architecture, reproducible workflows, and release-quality checkpoints for every milestone.

---

## ✨ Highlights

- Pure Rust kernel path
- UEFI boot on x86_64 (QEMU + OVMF first, hardware later)
- Linux syscall ABI subset strategy
- Milestone-oriented delivery with acceptance gates
- Theory + implementation paired documentation

---

## 🎯 Goals

- Build a readable, incremental kernel for learning and demos
- Reuse proven standards (UEFI, ELF, Linux ABI conventions)
- Keep development and testing repeatable
- Ship each milestone as a stable release branch

## 🚫 Non-goals (early phases)

- Full Linux compatibility
- Production hardening from day one
- Multi-architecture support in initial milestones
- Over-abstracted architecture that harms readability

---

## 🧭 Technical Direction

- **Language**: Rust
- **Boot model**: UEFI
- **ISA**: x86_64
- **Virtualization**: QEMU + OVMF
- **ABI direction**: Linux syscall subset
- **Milestone structure**: PRD → Design → Implement → Review

---

## 📁 Repository Layout

```text
kernel/                 # Kernel crate(s) and runtime path
user/                   # User-space ABI/demo components
scripts/                # Build/run/test scripts
docs/                   # Specs, roadmap, reviews, fundamentals, testing
  specs/                # PRD + design per milestone
  reviews/              # Reviewer reports
  milestones/           # Milestone plan and state
  fundamentals/         # Milestone theory notes
.github/workflows/      # CI pipelines (including Layer D)
```

---

## 🚀 Quick Start (local)

> Prerequisites: Rust nightly, QEMU, OVMF, mtools

```bash
# Build M1 image
scripts/build.sh

# Run in QEMU
scripts/run-qemu.sh
```

Expected boot markers include:

- `stardust-kernel <version>`
- `memmap: descriptors=... total_pages=... conventional_pages=...`

---

## ✅ Quality Gates

Testing uses four layers:

- **Layer A**: fast local correctness
- **Layer B**: local integration/demo smoke
- **Layer C**: reviewer baseline verification
- **Layer D**: release validation (Linux x86_64 CI)

Details:

- `docs/testing/test-layers.md`
- `docs/testing/layer-d-ci.md`
- `docs/testing/xtask-plan.md`

---

## 🗺️ Milestones & Release Branches

- `main`: active development
- `release/m{N}-{slug}`: stable milestone snapshots

Roadmap:

- `docs/milestones/ROADMAP.md`

---

## 📚 Learning Path (recommended)

1. `docs/milestones/ROADMAP.md`
2. `docs/fundamentals/README.md`
3. `docs/fundamentals/M1-boot-to-rust.md`
4. `docs/testing/test-layers.md`

This sequence explains both **why** and **how** each milestone works.

---

## 🤝 Contributing

Contributions are welcome in implementation, testing, review tooling, and docs quality.

Please read first:

- `CONTRIBUTING.md`
- `AGENTS.md`

---

## 📄 License

MIT. See [`LICENSE`](./LICENSE).
