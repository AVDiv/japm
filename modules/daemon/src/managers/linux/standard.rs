use crate::types::{
    child::{ChildProcess, ChildProcessStatus, ChildSignal},
    manager::{BaseManagerTrait, ProcessType},
};
use chrono::{DateTime, Local};
use std::process::Command;
use tokio::task::JoinHandle;

pub struct StandardLinuxManager {
    spawn_time: DateTime<Local>,
}

impl BaseManagerTrait for StandardLinuxManager {
    fn create() -> Self {
        let manager: StandardLinuxManager = StandardLinuxManager {
            spawn_time: Local::now(),
        };
        manager
    }
    fn spawn_process(&self, command: String) -> Result<ChildProcess, String> {
        // Spawn child process
        let child_instance = Command::new(&command).spawn();

        // Check for errors in the child process
        if child_instance.is_err() {
            todo!();
        } else if let Ok(child) = child_instance {
            let child: ChildProcess = ChildProcess {
                command: command,
                spawn_time: Local::now(),
                status: ChildProcessStatus::Pending,
                exit_code: None,
                pid: child.id(),
                user: "japm".to_string(),
                process_instance: Some(child),
            };
            Ok(child)
        } else {
            Err("Failed to spawn process".to_string())
        }
    }
    fn record_process_status(&self, pid: u32) {
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

    async fn spawn_manager_cycle(&self, pid: u32) -> JoinHandle<()> {
        todo!();
    }
}
