pub mod database;
pub mod managers;
pub mod types;

use daemonize::Daemonize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::process::Command as TokioCommand;

use crate::managers::daemon_supervisor::DaemonSupervisor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: &'static str = "127.0.0.1:9371";

    // Check if daemon is already running by attempting to bind
    let listener: TcpListener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(_) => {
            eprintln!("Daemon already running on {}.", addr);
            return Ok(());
        }
    };

    // Daemonize
    let stdout = File::create("/tmp/japm-daemon.out")?;
    let stderr = File::create("/tmp/japm-daemon.err")?;
    let pid_file = "/tmp/japm-daemon.pid";
    let daemonize = Daemonize::new()
        .pid_file(pid_file)
        .stdout(stdout)
        .stderr(stderr);

    daemonize.start()?;

    // Initialize the daemon supervisor
    let mut daemon_supervisor = DaemonSupervisor::new();

    // Start monitoring managers
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            daemon_supervisor.monitor_managers().await;
        }
    });

    // Accept CLI connections over TCP
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {}
            Err(err) => {
                println!("Daemon Module is offline, Running now...");
            }
        }
    }

    Ok(())
}
