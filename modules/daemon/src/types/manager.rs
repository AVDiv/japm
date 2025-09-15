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
    fn spawn_process(&self, command: String) -> Result<ChildProcess, String>; // For spawning processes on a command
    fn record_process_status(&self, pid: u32);
    fn terminate_process(&self, identifier: &mut ProcessType, signal: ChildSignal);
    async fn spawn_manager_cycle(&self, pid: u32) -> JoinHandle<()>;
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
    fn spawn_process(&self, command: String) -> Result<ChildProcess, String> {
        match self {
            ConcreteManager::LinuxStandard(mgr) => mgr.spawn_process(command),
            ConcreteManager::MacosStandard(mgr) => mgr.spawn_process(command),
        }
    }

    fn record_process_status(&self, pid: u32) {
        match self {
            ConcreteManager::LinuxStandard(mgr) => mgr.record_process_status(pid),
            ConcreteManager::MacosStandard(mgr) => mgr.record_process_status(pid),
        }
    }

    fn terminate_process(&self, identifier: &mut ProcessType, signal: ChildSignal) {
        match self {
            ConcreteManager::LinuxStandard(mgr) => mgr.terminate_process(identifier, signal),
            ConcreteManager::MacosStandard(mgr) => mgr.terminate_process(identifier, signal),
        }
    }

    async fn spawn_manager_cycle(&self, pid: u32) -> JoinHandle<()> {
        match self {
            ConcreteManager::LinuxStandard(mgr) => mgr.spawn_manager_cycle(pid).await,
            ConcreteManager::MacosStandard(mgr) => mgr.spawn_manager_cycle(pid).await,
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
