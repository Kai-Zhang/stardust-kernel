#![no_std]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DemoSyscall {
    Write { fd: u64, bytes: &'static [u8] },
    Exit { code: i32 },
    ExitGroup { code: i32 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DemoUserProgram {
    pub name: &'static str,
    pub calls: &'static [DemoSyscall],
}

const DEMO_CALLS: [DemoSyscall; 2] = [
    DemoSyscall::Write {
        fd: 1,
        bytes: b"hello from user payload\n",
    },
    DemoSyscall::ExitGroup { code: 0 },
];

pub const DEMO_USER_PROGRAM: DemoUserProgram = DemoUserProgram {
    name: "demo-user-hello",
    calls: &DEMO_CALLS,
};
