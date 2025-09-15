// use std::io::Empty;
// use duckdb::Connection;

// Connection properties for Database adapter
// enum DatabaseConnectionInstanceType {
//     DuckDB(Connection),
//     Null(Empty),
// }

// struct DatabaseConnectionParametersType {
//     protocol: String,
//     host: String,
//     port: i16,
//     username: String,
//     password: String,
//     dbname: String,
// }

// pub struct DatabaseConnectionProps {
//     instance: DatabaseConnectionInstanceType,
//     params: DatabaseConnectionParametersType,
// }

// Database Data Record Types
pub struct ProcessRecord {
    pub id: i32,
    pub name: String,
    pub pid: u32,
    pub user: String,
    pub spawn_time: i64,
    pub is_live: bool,
}

pub struct ProcessMetricsRecord {
    pub id: i32,
    pub timestamp: i64,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub status: String,
    pub thread_count: i32,
}

// Base Class of Database adapters
pub trait BaseDatabaseManager {
    fn initiate() -> Self; // Initiate a Database Manager instance
    fn initiate_schema(&self); // Create schema dedicated for JAPM
    fn connect(&mut self); // Connect to the Database
    fn disconnect(self); // Disconnect from the Database
    fn add_process(
        &self,
        name: String,
        timestamp: i64,
        pid: u32,
        user: String,
    ) -> Result<(), String>; // Add a process to the database
    fn remove_process(&self, id: i32) -> Result<(), String>; // Remove a process from the database
    fn list_processes(&self) -> Result<Vec<ProcessRecord>, String>; // List all processes in the database
    fn append_process_metrics(
        &self,
        id: i32,
        timestamp: i64,
        cpu_usage: f32,
        memory_usage: f32,
        status: String,
        thread_count: i32,
    ) -> Result<(), String>; // Append process metrics to the database
}
