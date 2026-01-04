use chrono::{DateTime, Local};
use std::process::Child;

#[derive(Clone)]
pub enum ChildProcessStatus {
    Pending,
    Sleeping,
    Running,
    Exited,
}

pub enum ChildSignal {
    SIGHUP = 1,
    SIGINT = 2,
    SIGQUIT = 3,
    SIGILL = 4,
    SIGTRAP = 5,
    SIGABRT = 6,
    SIGBUS = 7,
    SIGFPE = 8,
    SIGKILL = 9,
    SIGUSR1 = 10,
    SIGSEGV = 11,
    SIGUSR2 = 12,
    SIGPIPE = 13,
    SIGALRM = 14,
    SIGTERM = 15,
    // SIGTRAP = 16,
    SIGCHLD = 17,
    SIGCONT = 18,
    SIGSTOP = 19,
    SIGTSTP = 20,
    SIGTTIN = 21,
    SIGTTOU = 22,
    SIGURG = 23,
    SIGXCPU = 24,
    SIGXFSZ = 25,
    SIGVTALRM = 26,
    SIGPROF = 27,
    SIGWINCH = 28,
    SIGIO = 29,
    SIGPWR = 30,
    SIGSYS = 31,
}

pub struct ChildProcess {
    pub command: String,
    pub pid: u32,
    pub user: String,
    pub spawn_time: DateTime<Local>,
    pub status: ChildProcessStatus,
    pub exit_code: Option<i16>,
    pub process_instance: Option<Child>,
}
