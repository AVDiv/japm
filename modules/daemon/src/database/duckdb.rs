use duckdb::{Connection, params};

use crate::types::database::{BaseDatabaseManager, ProcessRecord};

pub struct DuckDBManager {
    db_path: &'static str,
    connection: Option<Connection>,
}

impl BaseDatabaseManager for DuckDBManager {
    fn initiate() -> Self {
        let instance = DuckDBManager {
            db_path: "~/.config/japm/log.db3",
            connection: None,
        };
        instance.initiate_schema();
        instance
    }
    fn connect(&mut self) {
        let connection: Result<Connection, duckdb::Error> = Connection::open(&self.db_path);

        if self.connection.is_none()
            && let Ok(conn) = connection
        {
            self.connection = Some(conn);
        }
    }
    fn disconnect(self) {
        if let Some(conn) = self.connection {
            conn.close();
        } else {
            // Non-existent connection situation
            eprintln!("Attempting to close non-existent connection.");
        }
    }
    fn initiate_schema(&self) {
        // Creation of schema if it doesn't exist
        if let Some(conn) = &self.connection {
            conn.execute_batch(
                "BEGIN;
                CREATE TABLE IF NOT EXISTS process(id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, pid INTEGER, user TEXT, spawn_time INTEGER, is_live BOOLEAN);
                CREATE TABLE IF NOT EXISTS usage_metrics(id INTEGER, timestamp INTEGER, cpu_usage REAL, mem_usage REAL, status TEXT, thread_count INTEGER, PRIMARY KEY(id, timestamp));
                COMMIT;",
            );
        }
    }

    fn add_process(
        &self,
        name: String,
        timestamp: i64,
        pid: u32,
        user: String,
    ) -> Result<(), String> {
        // Add process to the database
        if let Some(conn) = &self.connection {
            let result = conn.execute(
                "INSERT INTO process(name, pid, user, spawn_time, is_live) VALUES (?, ?, ?, ?, ?)",
                params![name, pid, user, timestamp, true],
            );

            match result {
                Ok(rows) => Ok(()),
                Err(err) => Err(format!("Failed to add process: {}", err)),
            }
        } else {
            Err("No database connection available".to_string())
        }
    }

    fn remove_process(&self, id: i32) -> Result<(), String> {
        // Remove process from the database
        if let Some(conn) = &self.connection {
            let result = conn.execute("DELETE FROM process WHERE id = ?", params![id]);

            match result {
                Ok(_) => Ok(()),
                Err(err) => Err(format!("Failed to remove process: {}", err)),
            }
        } else {
            Err("No database connection available".to_string())
        }
    }

    fn list_processes(&self) -> Result<Vec<ProcessRecord>, String> {
        // List all processes
        if let Some(conn) = &self.connection {
            let stmt = conn
                .prepare("SELECT id, name, pid, user, spawn_time FROM process WHERE is_live = 1");
            match stmt {
                Ok(mut stmt) => {
                    let rows = stmt.query_map([], |row| {
                        let id = row.get(0)?;
                        let name = row.get(1)?;
                        let pid = row.get(2)?;
                        let user = row.get(3)?;
                        let spawn_time = row.get(4)?;
                        Ok(ProcessRecord {
                            id,
                            name,
                            pid,
                            user,
                            spawn_time,
                            is_live: true,
                        })
                    });

                    match rows {
                        Ok(mapped_rows) => {
                            let mut processes = Vec::new();
                            for row in mapped_rows {
                                match row {
                                    Ok(process) => processes.push(process),
                                    Err(err) => {
                                        return Err(format!("Failed to process row: {}", err));
                                    }
                                }
                            }
                            Ok(processes)
                        }
                        Err(err) => Err(format!("Failed to query rows: {}", err)),
                    }
                }
                Err(err) => Err(format!("Failed to prepare statement: {}", err)),
            }
        } else {
            Err("No database connection available".to_string())
        }
    }

    fn append_process_metrics(
        &self,
        id: i32,
        timestamp: i64,
        cpu_usage: f32,
        memory_usage: f32,
        status: String,
        thread_count: i32,
    ) -> Result<(), String> {
        // Append process usage metrics
        if let Some(conn) = &self.connection {
            let result = conn.execute(
                "INSERT INTO usage_metrics(process_id, timestamp, cpu_usage, mem_usage, status, thread_count) VALUES (?, ?, ?, ?, ?, ?)",
                params![id, timestamp, cpu_usage, memory_usage, status, thread_count],
            );

            match result {
                Ok(_) => Ok(()),
                Err(err) => Err(format!("Failed to append process metrics: {}", err)),
            }
        } else {
            Err("No database connection available".to_string())
        }
    }
}
