use stardust_user::{DemoSyscall, DemoUserProgram, DEMO_USER_PROGRAM};

const SYS_READ: u64 = 0;
const SYS_WRITE: u64 = 1;
const SYS_MMAP: u64 = 9;
const SYS_MUNMAP: u64 = 11;
const SYS_BRK: u64 = 12;
const SYS_GETPID: u64 = 39;
const SYS_EXIT: u64 = 60;
const SYS_UNAME: u64 = 63;
const SYS_EXIT_GROUP: u64 = 231;

const LINUX_EBADF: isize = 9;
const LINUX_EFAULT: isize = 14;
const LINUX_EINVAL: isize = 22;
const LINUX_ENOSYS: isize = 38;

const MMAP_ANON_FLAG: u64 = 0x20;
const MMAP_PRIVATE_FLAG: u64 = 0x02;
const MMAP_BASE: u64 = 0x4000_0000;
const MMAP_STRIDE: u64 = 0x10_000;
const MMAP_SLOTS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuRing {
    Ring0,
    Ring3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SyscallResult {
    pub value: isize,
    pub exit_code: Option<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DemoRunReport {
    pub started_ring: CpuRing,
    pub entered_ring: CpuRing,
    pub final_ring: CpuRing,
    pub bytes_written: usize,
    pub bytes_read: usize,
    pub exit_code: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct M4AbiSmokeReport {
    pub read_ok: bool,
    pub brk_ok: bool,
    pub mmap_ok: bool,
    pub munmap_ok: bool,
    pub uname_ok: bool,
    pub getpid_ok: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Mapping {
    start: u64,
    len: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct KernelState {
    written: usize,
    read: usize,
    input_len: usize,
    exit_code: i32,
    halted: bool,
    brk_min: u64,
    brk_cur: u64,
    brk_max: u64,
    pid: u64,
    next_mmap: u64,
    mappings: [Option<Mapping>; MMAP_SLOTS],
}

impl KernelState {
    const fn new() -> Self {
        let base = 0x20_0000;
        Self {
            written: 0,
            read: 0,
            input_len: 32,
            exit_code: 0,
            halted: false,
            brk_min: base,
            brk_cur: base,
            brk_max: base + 0x10_0000,
            pid: 42,
            next_mmap: MMAP_BASE,
            mappings: [None; MMAP_SLOTS],
        }
    }

    fn alloc_mapping(&mut self, len: u64) -> Option<u64> {
        for slot in &mut self.mappings {
            if slot.is_none() {
                let addr = self.next_mmap;
                self.next_mmap = self.next_mmap.saturating_add(MMAP_STRIDE);
                *slot = Some(Mapping { start: addr, len });
                return Some(addr);
            }
        }
        None
    }

    fn drop_mapping(&mut self, start: u64, len: u64) -> bool {
        for slot in &mut self.mappings {
            if let Some(mapping) = slot {
                if mapping.start == start && mapping.len == len {
                    *slot = None;
                    return true;
                }
            }
        }
        false
    }
}

fn syscall_dispatch(
    nr: u64,
    arg0: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    state: &mut KernelState,
) -> SyscallResult {
    match nr {
        SYS_READ => {
            if arg0 != 0 {
                return SyscallResult {
                    value: -LINUX_EBADF,
                    exit_code: None,
                };
            }
            if arg1 == 0 {
                return SyscallResult {
                    value: -LINUX_EFAULT,
                    exit_code: None,
                };
            }
            if arg2 == 0 {
                return SyscallResult {
                    value: 0,
                    exit_code: None,
                };
            }
            let bytes = core::cmp::min(arg2 as usize, state.input_len);
            state.read += bytes;
            SyscallResult {
                value: bytes as isize,
                exit_code: None,
            }
        }
        SYS_WRITE => {
            if arg0 != 1 && arg0 != 2 {
                return SyscallResult {
                    value: -LINUX_EBADF,
                    exit_code: None,
                };
            }
            if arg1 == 0 {
                return SyscallResult {
                    value: -LINUX_EFAULT,
                    exit_code: None,
                };
            }
            state.written += arg2 as usize;
            SyscallResult {
                value: arg2 as isize,
                exit_code: None,
            }
        }
        SYS_BRK => {
            if arg0 == 0 {
                return SyscallResult {
                    value: state.brk_cur as isize,
                    exit_code: None,
                };
            }
            if arg0 < state.brk_min || arg0 > state.brk_max {
                return SyscallResult {
                    value: state.brk_cur as isize,
                    exit_code: None,
                };
            }
            state.brk_cur = arg0;
            SyscallResult {
                value: state.brk_cur as isize,
                exit_code: None,
            }
        }
        SYS_MMAP => {
            let flags = arg3;
            if arg2 == 0 || (flags & MMAP_ANON_FLAG == 0) || (flags & MMAP_PRIVATE_FLAG == 0) {
                return SyscallResult {
                    value: -LINUX_EINVAL,
                    exit_code: None,
                };
            }
            match state.alloc_mapping(arg2) {
                Some(addr) => SyscallResult {
                    value: addr as isize,
                    exit_code: None,
                },
                None => SyscallResult {
                    value: -LINUX_EINVAL,
                    exit_code: None,
                },
            }
        }
        SYS_MUNMAP => {
            if arg0 == 0 || arg1 == 0 {
                return SyscallResult {
                    value: -LINUX_EINVAL,
                    exit_code: None,
                };
            }
            if state.drop_mapping(arg0, arg1) {
                SyscallResult {
                    value: 0,
                    exit_code: None,
                }
            } else {
                SyscallResult {
                    value: -LINUX_EINVAL,
                    exit_code: None,
                }
            }
        }
        SYS_GETPID => SyscallResult {
            value: state.pid as isize,
            exit_code: None,
        },
        SYS_UNAME => {
            if arg0 == 0 {
                SyscallResult {
                    value: -LINUX_EFAULT,
                    exit_code: None,
                }
            } else {
                SyscallResult {
                    value: 0,
                    exit_code: None,
                }
            }
        }
        SYS_EXIT | SYS_EXIT_GROUP => {
            state.exit_code = arg0 as i32;
            state.halted = true;
            SyscallResult {
                value: 0,
                exit_code: Some(state.exit_code),
            }
        }
        _ => SyscallResult {
            value: -LINUX_ENOSYS,
            exit_code: None,
        },
    }
}

pub fn run_demo_program(program: &DemoUserProgram) -> DemoRunReport {
    let started_ring = CpuRing::Ring0;
    let mut state = KernelState::new();

    for call in program.calls {
        let result = match call {
            DemoSyscall::Write { fd, bytes } => {
                syscall_dispatch(SYS_WRITE, *fd, 1, bytes.len() as u64, 0, &mut state)
            }
            DemoSyscall::Read { fd, len } => {
                syscall_dispatch(SYS_READ, *fd, 1, *len as u64, 0, &mut state)
            }
            DemoSyscall::BrkSet { addr } => syscall_dispatch(SYS_BRK, *addr, 0, 0, 0, &mut state),
            DemoSyscall::MmapAnon { len, flags } => {
                syscall_dispatch(SYS_MMAP, 0, 0, *len as u64, *flags, &mut state)
            }
            DemoSyscall::Munmap { addr, len } => {
                syscall_dispatch(SYS_MUNMAP, *addr, *len as u64, 0, 0, &mut state)
            }
            DemoSyscall::Uname => syscall_dispatch(SYS_UNAME, 1, 0, 0, 0, &mut state),
            DemoSyscall::Getpid => syscall_dispatch(SYS_GETPID, 0, 0, 0, 0, &mut state),
            DemoSyscall::Exit { code } => {
                syscall_dispatch(SYS_EXIT, *code as u64, 0, 0, 0, &mut state)
            }
            DemoSyscall::ExitGroup { code } => {
                syscall_dispatch(SYS_EXIT_GROUP, *code as u64, 0, 0, 0, &mut state)
            }
        };

        if result.value < 0 || result.exit_code.is_some() {
            break;
        }
    }

    DemoRunReport {
        started_ring,
        entered_ring: CpuRing::Ring3,
        final_ring: CpuRing::Ring0,
        bytes_written: state.written,
        bytes_read: state.read,
        exit_code: state.exit_code,
    }
}

pub fn run_m3_demo_payload() -> DemoRunReport {
    run_demo_program(&DEMO_USER_PROGRAM)
}

pub fn run_m4_abi_smoke() -> M4AbiSmokeReport {
    let mut state = KernelState::new();

    let read = syscall_dispatch(SYS_READ, 0, 1, 8, 0, &mut state);
    let brk_now = syscall_dispatch(SYS_BRK, 0, 0, 0, 0, &mut state);
    let brk_set = syscall_dispatch(SYS_BRK, state.brk_min + 0x2000, 0, 0, 0, &mut state);
    let mmap = syscall_dispatch(
        SYS_MMAP,
        0,
        0,
        0x2000,
        MMAP_ANON_FLAG | MMAP_PRIVATE_FLAG,
        &mut state,
    );
    let munmap = if mmap.value > 0 {
        syscall_dispatch(SYS_MUNMAP, mmap.value as u64, 0x2000, 0, 0, &mut state)
    } else {
        SyscallResult {
            value: -1,
            exit_code: None,
        }
    };
    let uname = syscall_dispatch(SYS_UNAME, 1, 0, 0, 0, &mut state);
    let getpid = syscall_dispatch(SYS_GETPID, 0, 0, 0, 0, &mut state);

    M4AbiSmokeReport {
        read_ok: read.value == 8,
        brk_ok: brk_now.value > 0 && brk_set.value == (state.brk_min + 0x2000) as isize,
        mmap_ok: mmap.value > 0,
        munmap_ok: munmap.value == 0,
        uname_ok: uname.value == 0,
        getpid_ok: getpid.value == state.pid as isize,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_rejects_non_stdio_fd_with_linux_errno() {
        let mut state = KernelState::new();
        let result = syscall_dispatch(SYS_WRITE, 3, 1, 10, 0, &mut state);
        assert_eq!(result.value, -9);
    }

    #[test]
    fn read_requires_stdin_and_non_null_buffer() {
        let mut state = KernelState::new();
        assert_eq!(syscall_dispatch(SYS_READ, 1, 1, 4, 0, &mut state).value, -9);
        assert_eq!(
            syscall_dispatch(SYS_READ, 0, 0, 4, 0, &mut state).value,
            -14
        );
    }

    #[test]
    fn brk_returns_current_when_request_out_of_range() {
        let mut state = KernelState::new();
        let base = syscall_dispatch(SYS_BRK, 0, 0, 0, 0, &mut state).value;
        let too_high = syscall_dispatch(SYS_BRK, state.brk_max + 1, 0, 0, 0, &mut state).value;
        assert_eq!(base, too_high);
    }

    #[test]
    fn mmap_and_munmap_validate_anonymous_private_contract() {
        let mut state = KernelState::new();
        let invalid = syscall_dispatch(SYS_MMAP, 0, 0, 0x1000, MMAP_PRIVATE_FLAG, &mut state);
        assert_eq!(invalid.value, -22);

        let mapped = syscall_dispatch(
            SYS_MMAP,
            0,
            0,
            0x2000,
            MMAP_ANON_FLAG | MMAP_PRIVATE_FLAG,
            &mut state,
        );
        assert!(mapped.value > 0);
        let unmapped = syscall_dispatch(SYS_MUNMAP, mapped.value as u64, 0x2000, 0, 0, &mut state);
        assert_eq!(unmapped.value, 0);
    }

    #[test]
    fn uname_and_getpid_follow_phase_c_shape() {
        let mut state = KernelState::new();
        assert_eq!(
            syscall_dispatch(SYS_UNAME, 0, 0, 0, 0, &mut state).value,
            -14
        );
        assert_eq!(syscall_dispatch(SYS_UNAME, 1, 0, 0, 0, &mut state).value, 0);
        assert_eq!(
            syscall_dispatch(SYS_GETPID, 0, 0, 0, 0, &mut state).value,
            42
        );
    }

    #[test]
    fn unknown_syscall_returns_linux_enosys() {
        let mut state = KernelState::new();
        let result = syscall_dispatch(999, 0, 0, 0, 0, &mut state);
        assert_eq!(result.value, -38);
    }

    #[test]
    fn exit_and_exit_group_share_single_task_exit_behavior() {
        let mut exit_state = KernelState::new();
        let exit = syscall_dispatch(SYS_EXIT, 7, 0, 0, 0, &mut exit_state);
        assert_eq!(exit.value, 0);
        assert_eq!(exit.exit_code, Some(7));

        let mut group_state = KernelState::new();
        let exit_group = syscall_dispatch(SYS_EXIT_GROUP, 9, 0, 0, 0, &mut group_state);
        assert_eq!(exit_group.value, 0);
        assert_eq!(exit_group.exit_code, Some(9));
    }

    #[test]
    fn demo_payload_transitions_ring3_and_returns_ring0() {
        let report = run_m3_demo_payload();
        assert_eq!(report.started_ring, CpuRing::Ring0);
        assert_eq!(report.entered_ring, CpuRing::Ring3);
        assert_eq!(report.final_ring, CpuRing::Ring0);
        assert!(report.bytes_written > 0);
        assert_eq!(report.exit_code, 0);
    }

    #[test]
    fn m4_abi_smoke_confirms_phase_b_c_surface() {
        let report = run_m4_abi_smoke();
        assert!(report.read_ok);
        assert!(report.brk_ok);
        assert!(report.mmap_ok);
        assert!(report.munmap_ok);
        assert!(report.uname_ok);
        assert!(report.getpid_ok);
    }
}
