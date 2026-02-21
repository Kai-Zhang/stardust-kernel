#![no_std]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DemoSyscall {
    Read { fd: u64, len: usize },
    Write { fd: u64, bytes: &'static [u8] },
    BrkSet { addr: u64 },
    MmapAnon { len: usize, flags: u64 },
    Munmap { addr: u64, len: usize },
    Uname,
    Getpid,
    Exit { code: i32 },
    ExitGroup { code: i32 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DemoUserProgram {
    pub name: &'static str,
    pub calls: &'static [DemoSyscall],
}

const DEMO_CALLS: [DemoSyscall; 5] = [
    DemoSyscall::Read { fd: 0, len: 8 },
    DemoSyscall::Write {
        fd: 1,
        bytes: b"hello from user payload\n",
    },
    DemoSyscall::Uname,
    DemoSyscall::Getpid,
    DemoSyscall::ExitGroup { code: 0 },
];

pub const DEMO_USER_PROGRAM: DemoUserProgram = DemoUserProgram {
    name: "demo-user-hello",
    calls: &DEMO_CALLS,
};
