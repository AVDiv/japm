use chrono::{DateTime, Local};
use std::collections::HashMap;

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
    pub fn spawn_process(&self, name: String, command: String, mgr_type: Option<&str>) {
        let mgr_type: &str = mgr_type.unwrap_or("standard");

        // Spawn the process
        let process = self.instances[mgr_type].spawn_process(command);
        match process {
            Ok(child) => {
                let add_result =
                    self.db
                        .add_process(name, Local::now().timestamp(), child.pid, child.user);
                if let Err(err) = add_result {
                    eprintln!("{}", err);
                }
            }
            Err(err) => {
                eprintln!("{}", err)
            }
        }
    }

    pub fn list_all_processes(&self) -> Result<Vec<ProcessRecord>, String> {
        // List all processes
        let processes = self.db.list_processes();
        match processes {
            Ok(processes) => Ok(processes),
            Err(err) => Err(err),
        }
    }
}
