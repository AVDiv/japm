use chrono::{DateTime, Local};
use std::env;
use thiserror::Error;
use tokio::task::JoinHandle;

use crate::{
    managers::{linux::standard::StandardLinuxManager, macos::standard::StandardMacosManager},
    types::child::{ChildProcess, ChildSignal},
};

pub enum ProcessType {
    ProcessInstance(ChildProcess),
    ProcessID(i32),
}

pub trait BaseManagerTrait {
    fn create() -> Self; // For initiating the manager instance
    // Getters
    fn get_children_count(&self) -> usize; // For getting the number of child processes
    fn get_child_process_by_pid(&self, pid: u32) -> Option<&ChildProcess>; // For getting a child process by PID
    fn get_manager_spawn_time(&self) -> DateTime<Local>; // For getting the spawn time of the manager
    // Other utilities
    fn spawn_process(&mut self, command: &String) -> Result<(u32, String), String>; // For spawning processes on a command
    fn record_process_status(&self, pid: &u32);
    fn terminate_process(&self, identifier: &mut ProcessType, signal: &ChildSignal);
    fn spawn_manager_cycle(&self, pid: &u32) -> JoinHandle<()>;
}

pub enum ConcreteManager {
    LinuxStandard(StandardLinuxManager),
    MacosStandard(StandardMacosManager),
}

#[derive(PartialEq)]
pub enum ManagerInstanceTypes {
    LinuxStandard,
    MacosStandard,
}

impl BaseManagerTrait for ConcreteManager {
    fn create() -> Self {
        panic!("Use create_manager(selection) to construct ConcreteManager");
    }

    fn get_children_count(&self) -> usize {
        match self {
            ConcreteManager::LinuxStandard(mgr) => mgr.get_children_count(),
            ConcreteManager::MacosStandard(mgr) => mgr.get_children_count(),
        }
    }

    fn get_child_process_by_pid(&self, pid: u32) -> Option<&ChildProcess> {
        match self {
            ConcreteManager::LinuxStandard(mgr) => mgr.get_child_process_by_pid(pid),
            ConcreteManager::MacosStandard(mgr) => mgr.get_child_process_by_pid(pid),
        }
    }

    fn get_manager_spawn_time(&self) -> DateTime<Local> {
        match self {
            ConcreteManager::LinuxStandard(mgr) => mgr.get_manager_spawn_time(),
            ConcreteManager::MacosStandard(mgr) => mgr.get_manager_spawn_time(),
        }
    }

    fn spawn_process(&mut self, command: &String) -> Result<(u32, String), String> {
        match self {
            ConcreteManager::LinuxStandard(mgr) => mgr.spawn_process(command),
            ConcreteManager::MacosStandard(mgr) => mgr.spawn_process(command),
        }
    }

    fn record_process_status(&self, pid: &u32) {
        match self {
            ConcreteManager::LinuxStandard(mgr) => mgr.record_process_status(pid),
            ConcreteManager::MacosStandard(mgr) => mgr.record_process_status(pid),
        }
    }

    fn terminate_process(&self, identifier: &mut ProcessType, signal: &ChildSignal) {
        match self {
            ConcreteManager::LinuxStandard(mgr) => mgr.terminate_process(identifier, signal),
            ConcreteManager::MacosStandard(mgr) => mgr.terminate_process(identifier, signal),
        }
    }

    fn spawn_manager_cycle(&self, pid: &u32) -> JoinHandle<()> {
        match self {
            ConcreteManager::LinuxStandard(mgr) => mgr.spawn_manager_cycle(pid),
            ConcreteManager::MacosStandard(mgr) => mgr.spawn_manager_cycle(pid),
        }
    }
}

// Error type for factory failures
#[derive(Error, Debug)]
pub enum ManagerError {
    #[error("Unsupported manager type: {0}. Supported: linux, macos")]
    UnsupportedType(String),
    #[error(
        "Incompatible environment: Requested {requested}-based manager, but running on {actual}"
    )]
    IncompatibleEnv { requested: String, actual: String },
}

pub fn create_manager(mgr_type: ManagerInstanceTypes) -> Result<ConcreteManager, ManagerError> {
    let actual_os = env::consts::OS.to_string();

    // Check if selection is supported
    let manager = match mgr_type {
        ManagerInstanceTypes::LinuxStandard => {
            ConcreteManager::LinuxStandard(StandardLinuxManager::create())
        }
        ManagerInstanceTypes::MacosStandard => {
            ConcreteManager::MacosStandard(StandardMacosManager::create())
        }
    };

    // Compatibility check (e.g., OS match)
    if actual_os != "linux" && (mgr_type == ManagerInstanceTypes::LinuxStandard) {
        return Err(ManagerError::IncompatibleEnv {
            requested: String::from("linux"),
            actual: actual_os,
        });
    } else if actual_os != "macos" && (mgr_type == ManagerInstanceTypes::MacosStandard) {
        return Err(ManagerError::IncompatibleEnv {
            requested: String::from("macos"),
            actual: actual_os,
        });
    }

    Ok(manager)
}
