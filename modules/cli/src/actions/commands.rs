use crate::types::cli::other::RestartPolicyCli;
use japm_daemon::managers::daemon_supervisor::DaemonSupervisor;

pub fn start_process(
    dsup: &DaemonSupervisor,
    name: &String,
    command: &Vec<String>,
    restart: &RestartPolicyCli,
) {
    let combined_command = command.join(" ");
    dsup.spawn_process(&name, &combined_command, Some("standard"));
}

pub fn stop_process(dsup: &DaemonSupervisor, id: &String) {
    // dsup.stop_process(id);
    todo!("Stay tuned, Pls! :)")
}
pub fn list_processes(dsup: &DaemonSupervisor) {
    let result = dsup.list_all_processes();
    if let Ok(processes) = result {
        for process in processes {
            println!("{}: {}", process.id, process.name);
        }
    }
}
