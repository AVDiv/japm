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

// Base Class of Database adapters
pub trait BaseDatabaseManager {
    fn initiate() -> Self; // Initiate a Database Manager instance
    fn create_or_get_schema(&self); // Create/Get a Database dedicated for JAPM
    fn connect(&mut self); // Connect to the Database
    fn disconnect(self); // Disconnect from the Database
}
