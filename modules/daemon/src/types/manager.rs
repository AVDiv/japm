use crate::types::child::ChildProcess;

pub trait BaseManager {
    fn create() -> Self; // For initiating the manager instance
    fn spawn_process(&self, command: String) -> ChildProcess; // For spawning processes on a command
    fn record_process_status(&self, pid: i32);
    fn terminate_process(&self, process: ChildProcess, signal: i16);
}
