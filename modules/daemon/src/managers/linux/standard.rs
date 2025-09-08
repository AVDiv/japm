use crate::types::{
    child::{ChildProcess, ChildProcessStatus},
    manager::BaseManager,
};
use chrono::{DateTime, Local};
use std::process::{Command, Stdio};

pub struct StandardLinuxManager {
    spawn_time: DateTime<Local>,
}

impl BaseManager for StandardLinuxManager {
    fn create() -> Self {
        let manager: StandardLinuxManager = StandardLinuxManager {
            spawn_time: Local::now(),
        };
        manager
    }
    fn spawn_process(&self, command: String) -> ChildProcess {
        // Spawn child process
        let mut child_instance = Command::new(command).spawn();

        // Check for errors in the child process
        if (child_instance.is_err()) {}

        let child: ChildProcess = ChildProcess {
            command: command,
            spawn_time: Local::now(),
            status: ChildProcessStatus::Pending,
            exit_code: None,
            process_instance: child_instance,
        };
        child
    }
    fn terminate_process(&self, process: Union<ChildProcess, i32>, signal: i16) {}
}
