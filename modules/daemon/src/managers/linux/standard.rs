use crate::types::{
    child::{ChildProcess, ChildProcessStatus, ChildSignal},
    manager::{BaseManager, ProcessType},
};
use chrono::{DateTime, Local};
use std::process::Command;

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
        let child_instance = Command::new(&command).spawn();

        // Check for errors in the child process
        if child_instance.is_err() {
            todo!();
        }

        let child: ChildProcess = ChildProcess {
            command: command,
            spawn_time: Local::now(),
            status: ChildProcessStatus::Pending,
            exit_code: None,
            process_instance: Some(child_instance.unwrap()),
        };
        child
    }
    fn record_process_status(&self, pid: i32) {
        todo!();
    }
    fn terminate_process(&self, identifier: &mut ProcessType, _signal: ChildSignal) {
        match identifier {
            // For Process Instance
            ProcessType::ProcessInstance(value) => {
                if let Some(child) = value.process_instance.as_mut() {
                    child
                        .kill()
                        .expect(&format!("Failed attempt to kill process {}.", child.id()));
                }
            }
            ProcessType::ProcessID(_value) => {
                todo!("Not implemented yet! Use Process Instance method for the moment.")
            }
        }
    }
}
