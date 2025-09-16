use crate::types::{
    child::{ChildProcess, ChildProcessStatus, ChildSignal},
    manager::{BaseManagerTrait, ConcreteManager, ProcessType},
};
use chrono::{DateTime, Local};
use tokio::task::JoinHandle;

pub struct StandardMacosManager {
    spawn_time: DateTime<Local>,
    watchdog_cycles: Vec<JoinHandle<()>>,
}

impl BaseManagerTrait for StandardMacosManager {
    fn create() -> Self {
        let manager: StandardMacosManager = StandardMacosManager {
            spawn_time: Local::now(),
            watchdog_cycles: vec![],
        };
        manager
    }
    fn spawn_process(&self, command: String) -> Result<ChildProcess, String> {
        todo!();
    }
    fn record_process_status(&self, pid: u32) {
        todo!();
    }
    fn terminate_process(&self, identifier: &mut ProcessType, _signal: ChildSignal) {
        todo!();
    }
    fn spawn_manager_cycle(&self, pid: u32) -> JoinHandle<()> {
        todo!();
    }
}
