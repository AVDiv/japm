use std::collections::HashMap;

use crate::types::{
    child::{ChildProcess, ChildProcessStatus, ChildSignal},
    manager::{BaseManagerTrait, ConcreteManager, ProcessType},
};
use chrono::{DateTime, Local};
use tokio::task::JoinHandle;

pub struct StandardMacosManager {
    children: HashMap<u32, ChildProcess>,
    spawn_time: DateTime<Local>,
    watchdog_cycles: Vec<JoinHandle<()>>,
}

impl BaseManagerTrait for StandardMacosManager {
    fn create() -> Self {
        let manager: StandardMacosManager = StandardMacosManager {
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
        todo!();
    }
    fn record_process_status(&self, pid: &u32) {
        todo!();
    }
    fn terminate_process(&self, identifier: &mut ProcessType, _signal: &ChildSignal) {
        todo!();
    }
    fn spawn_manager_cycle(&self, pid: &u32) -> JoinHandle<()> {
        todo!();
    }
}
