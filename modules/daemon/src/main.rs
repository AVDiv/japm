pub mod database;
pub mod managers;
pub mod types;

use std::time::Duration;
use tokio::task::JoinHandle;

use crate::{
    managers::daemon_supervisor::DaemonSupervisor,
    types::manager::{BaseManagerTrait, ConcreteManager},
};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Initialize setup
//     tracing_subscriber::fmt::init(); // Logger
//     let ds = DaemonSupervisor::new(); // Deamon Supervisor

//     // Spawn the background worker task
//     let worker_handle = tokio::spawn(async move {
//         let mut interval = tokio::time::interval(Duration::from_secs(1));
//         loop {
//             interval.tick().await;
//             if !(ds.instances.len() > 0) {
//                 break;
//             }
//         }
//     });

//     // Await for till the daemon is shutdown
//     worker_handle.await?;
//     Ok(())
// }
