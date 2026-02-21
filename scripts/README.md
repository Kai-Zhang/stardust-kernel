# Scripts Directory

## M1 boot-to-rust scripts

- `scripts/build.sh`
  - Builds `kernel` as `x86_64-unknown-uefi`
  - Produces FAT image at `target/m1-boot.img` with `EFI/BOOT/BOOTX64.EFI`
- `scripts/run-qemu.sh`
  - Boots the image with QEMU + OVMF
  - Prints serial output in terminal

## Requirements

- Rust toolchain with `cargo` + `rustup`
- `mtools` (`mformat`, `mmd`, `mcopy`)
- `qemu-system-x86_64`
- OVMF firmware file (`OVMF_CODE.fd`), optionally via `OVMF_CODE=/path/to/OVMF_CODE.fd`

## Usage

```bash
scripts/build.sh
scripts/run-qemu.sh
```

Expected output includes:
- `stardust-kernel <version>`
- `memmap: descriptors=<n> total_pages=<n> conventional_pages=<n>`
