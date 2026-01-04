use crate::types::{
    child::{ChildProcess, ChildProcessStatus, ChildSignal},
    manager::{BaseManagerTrait, ProcessType},
};
use chrono::{DateTime, Local};
use std::{collections::HashMap, process::Command};
use tokio::task::JoinHandle;

pub struct StandardLinuxManager {
    pub children: HashMap<u32, ChildProcess>,
    spawn_time: DateTime<Local>,
    watchdog_cycles: Vec<JoinHandle<()>>,
}

impl BaseManagerTrait for StandardLinuxManager {
    fn create() -> Self {
        let manager: StandardLinuxManager = StandardLinuxManager {
            children: HashMap::new(),
            spawn_time: Local::now(),
            watchdog_cycles: vec![],
        };
        manager
    }

    fn get_children_count(&self) -> usize {
        self.children.len()
    }

    fn get_child_process_by_pid(&self, pid: u32) -> Option<&ChildProcess> {
        self.children.get(&pid)
    }

    fn get_manager_spawn_time(&self) -> DateTime<Local> {
        self.spawn_time
    }

    fn spawn_process(&mut self, command: &String) -> Result<(u32, String), String> {
        // Spawn child process
        let child_instance = Command::new(&command).spawn();

        // Check for errors in the child process
        if child_instance.is_err() {
            Err(format!(
                "Failed to spawn process: {}",
                child_instance.err().unwrap()
            ))
        } else if let Ok(child) = child_instance {
            let child: ChildProcess = ChildProcess {
                command: command.clone(),
                spawn_time: Local::now(),
                status: ChildProcessStatus::Pending,
                exit_code: None,
                pid: child.id(),
                user: "japm".to_string(),
                process_instance: Some(child),
            };
            let pid_clone = child.pid.clone();
            let user_clone = child.user.clone();
            self.children.insert(pid_clone, child);
            Ok((pid_clone, user_clone))
        } else {
            Err("Failed to spawn process".to_string())
        }
    }

    fn record_process_status(&self, pid: &u32) {
        todo!();
    }
    fn terminate_process(&self, identifier: &mut ProcessType, _signal: &ChildSignal) {
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

    fn spawn_manager_cycle(&self, pid: &u32) -> JoinHandle<()> {
        todo!();
    }
}
