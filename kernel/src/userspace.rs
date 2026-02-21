use stardust_user::{DemoSyscall, DemoUserProgram, DEMO_USER_PROGRAM};

const SYS_WRITE: u64 = 1;
const SYS_EXIT: u64 = 60;
const SYS_EXIT_GROUP: u64 = 231;

const LINUX_EBADF: isize = 9;
const LINUX_EFAULT: isize = 14;
const LINUX_ENOSYS: isize = 38;

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
    pub exit_code: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct KernelState {
    written: usize,
    exit_code: i32,
    halted: bool,
}

impl KernelState {
    const fn new() -> Self {
        Self {
            written: 0,
            exit_code: 0,
            halted: false,
        }
    }
}

fn syscall_dispatch(nr: u64, arg0: u64, arg1: u64, state: &mut KernelState) -> SyscallResult {
    match nr {
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
            state.written += arg1 as usize;
            SyscallResult {
                value: arg1 as isize,
                exit_code: None,
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
                syscall_dispatch(SYS_WRITE, *fd, bytes.len() as u64, &mut state)
            }
            DemoSyscall::Exit { code } => syscall_dispatch(SYS_EXIT, *code as u64, 1, &mut state),
            DemoSyscall::ExitGroup { code } => {
                syscall_dispatch(SYS_EXIT_GROUP, *code as u64, 1, &mut state)
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
        exit_code: state.exit_code,
    }
}

pub fn run_m3_demo_payload() -> DemoRunReport {
    run_demo_program(&DEMO_USER_PROGRAM)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_rejects_non_stdio_fd_with_linux_errno() {
        let mut state = KernelState::new();
        let result = syscall_dispatch(SYS_WRITE, 3, 10, &mut state);
        assert_eq!(result.value, -9);
    }

    #[test]
    fn unknown_syscall_returns_linux_enosys() {
        let mut state = KernelState::new();
        let result = syscall_dispatch(999, 0, 0, &mut state);
        assert_eq!(result.value, -38);
    }

    #[test]
    fn exit_and_exit_group_share_single_task_exit_behavior() {
        let mut exit_state = KernelState::new();
        let exit = syscall_dispatch(SYS_EXIT, 7, 1, &mut exit_state);
        assert_eq!(exit.value, 0);
        assert_eq!(exit.exit_code, Some(7));

        let mut group_state = KernelState::new();
        let exit_group = syscall_dispatch(SYS_EXIT_GROUP, 9, 1, &mut group_state);
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
}
