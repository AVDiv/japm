#[cfg(target_os = "linux")]
pub mod linux_specific {
    use crate::types::cli::other::RestartPolicyCli;
    use chrono::{DateTime, Local};
    use japm_daemon::{
        managers::linux::standard::StandardLinuxManager, types::manager::BaseManager,
    };

    pub fn start_process(name: &String, command: &Vec<String>, restart: &RestartPolicyCli) {
        let manager: StandardLinuxManager = StandardLinuxManager::create();
        let combined_command = command.join(" ");
        manager.spawn_process(combined_command);
    }
    pub fn stop_process(id: &String) {}
    pub fn list_processes() {}
}

#[cfg(target_os = "windows")]
pub mod windows_specific {
    pub fn start_process(name: &String, command: &Vec<String>, restart: &RestartPolicyCli) {}
    pub fn stop_process(id: &String) {}
    pub fn list_processes() {}
}

#[cfg(target_os = "macos")]
pub mod macos_specific {
    pub fn start_process(name: &String, command: &Vec<String>, restart: &RestartPolicyCli) {}
    pub fn stop_process(id: &String) {}
    pub fn list_processes() {}
}
