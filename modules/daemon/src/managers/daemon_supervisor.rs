use chrono::{DateTime, Local};
use std::collections::HashMap;
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

use crate::{
    database::duckdb::DuckDBManager,
    types::{
        database::{BaseDatabaseManager, ProcessRecord},
        manager::{BaseManagerTrait, ConcreteManager},
    },
};

pub struct DaemonSupervisor {
    pub spawn_time: DateTime<Local>,
    pub instances: HashMap<String, ConcreteManager>,
    pub db: DuckDBManager,
}

impl DaemonSupervisor {
    pub fn new() -> Self {
        let mut supervisor: DaemonSupervisor = DaemonSupervisor {
            spawn_time: Local::now(),
            instances: HashMap::new(),
            db: DuckDBManager::initiate(),
        };
        // Initialize the database connection
        supervisor.db.initiate_schema();
        supervisor.db.connect();

        supervisor
    }
    fn spawn_process(&mut self, name: &String, command: &String, mgr_type: Option<&str>) {
        let mgr_type: &str = mgr_type.unwrap_or("standard");

        // Spawn the process
        if let Some(manager) = self.instances.get_mut(mgr_type) {
            let process = manager.spawn_process(command);
            match process {
                Ok((child_pid, child_user)) => {
                    let add_result = self.db.add_process(
                        &name,
                        Local::now().timestamp(),
                        child_pid.clone(),
                        child_user.clone(),
                    );
                    if let Err(err) = add_result {
                        eprintln!("{}", err);
                    }
                }
                Err(err) => {
                    eprintln!("{}", err)
                }
            }
        } else {
            eprintln!("Manager type '{}' not found", mgr_type);
        }
    }

    fn list_all_processes(&self) -> Result<Vec<ProcessRecord>, String> {
        // List all processes
        let processes = self.db.list_processes();
        match processes {
            Ok(processes) => Ok(processes),
            Err(err) => Err(err),
        }
    }

    pub async fn handle_stream(&self, mut stream: TcpStream) -> Result<(), ()> {
        let (reader, mut writer) = stream.split();
        let mut reader = BufReader::new(reader);
        let mut command = String::new();
        reader.read_line(&mut command).await?;

        match command.trim() {
            "status" => {
                let response = format!("Running commands: {}\n", state.commands.len());
                writer.write(response.as_bytes()).await?;
                writer.flush().await?;
            }
            cmd if cmd.starts_with("run ") => {
                let parts: Vec<&str> = cmd.split_whitespace().collect();
                if parts.len() >= 2 {
                    let cmd = parts[1];
                    let args = parts[2..].iter().map(|s| s.to_string()).collect();
                    let id = self.spawn_command(cmd, args).await;
                    let response = format!("Started command with ID: {}\n", id);
                    writer.write(response.as_bytes()).await?;
                    writer.flush().await?;
                } else {
                    writer.write(b"Invalid run command\n").await?;
                    writer.flush().await?;
                }
            }
            "shutdown" => {
                writer.write(b"Shutting down daemon...\n").await?;
                writer.flush().await?;
                std::process::exit(0);
            }
            _ => {
                writer.write(b"Unknown command\n").await?;
                writer.flush().await?;
            }
        }
        Ok(())
    }

    pub async fn monitor_managers(&mut self) {
        // Collect names of managers to remove
        let managers_to_remove: Vec<String> = self
            .instances
            .iter()
            .filter(|(_, manager)| !(manager.get_children_count() > 0))
            .map(|(name, _)| {
                println!(
                    "{} has no active child processes. Queued to be destroyed.",
                    name
                );
                name.clone()
            })
            .collect();

        // Remove managers in a separate step
        for name in managers_to_remove {
            self.instances.remove(&name);
        }
    }
}
