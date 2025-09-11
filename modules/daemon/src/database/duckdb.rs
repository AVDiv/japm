use crate::types::database::BaseDatabaseManager;
use duckdb::Connection;

struct DuckDBManager {
    connection: Connection,
}

impl BaseDatabaseManager for DuckDBManager {
    fn initiate() -> Self {
        let path = "/var/local/japm/log.db3";

        let connection = Connection::open(&path);
        let instance = DuckDBManager {
            connection: connection.unwrap(),
        };
        instance
    }
    fn connect(&mut self) {}
    fn disconnect(self) {
        self.connection.close();
    }
    fn create_or_get_schema(&self) {
        self.connection.execute_batch(
            "BEGIN;
            CREATE TABLE process(name TEXT);
            CREATE TABLE cpu_usage(y TEXT);
            COMMIT;",
        );
    }
}
