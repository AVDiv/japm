mod actions;
mod types;

use clap::Parser;
use japm_common::RestartPolicy;
use types::cli::{other::RestartPolicyCli, subcommands::SubcommandsType};

#[derive(Debug, Parser)]
#[command(
    name = "JAPM",
    display_name = "JAPM",
    bin_name = "japm",
    version,
    about = "JAPM — Just Another Process Manager",
    long_about = "JAPM is a rust-based process manager for manipulating & monitoring processes effectively while not consuming least of resources for just running in the background.",
    author = "Avin Divakara, divakaraavin@gmail.com"
)]
struct Args {
    #[command(subcommand)]
    command: SubcommandsType,
}

impl From<RestartPolicyCli> for RestartPolicy {
    fn from(v: RestartPolicyCli) -> Self {
        match v {
            RestartPolicyCli::Never => RestartPolicy::Never,
            RestartPolicyCli::OnFailure => RestartPolicy::OnFailure,
            RestartPolicyCli::Always => RestartPolicy::Always,
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Platform-specific division
    #[cfg(target_os = "linux")]
    {
        match &args.command {
            // `start` command
            SubcommandsType::Start {
                name,
                command,
                restart,
            } => {
                actions::commands::linux_specific::start_process(name, command, restart);
            }
            SubcommandsType::Stop { id } => {
                actions::commands::linux_specific::stop_process(id);
            }
            SubcommandsType::List => {
                actions::commands::linux_specific::list_processes();
            }
        }
    }
    #[cfg(target_os = "windows")]
    {
        match &args.command {
            // `start` command
            SubcommandsType::Start {
                name,
                command,
                restart,
            } => {
                actions::commands::windows_specific::start_process(name, command, restart);
            }
            SubcommandsType::Stop { id } => {
                actions::commands::windows_specific::stop_process(id);
            }
            SubcommandsType::List => {
                actions::commands::windows_specific::list_processes();
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        match &args.command {
            // `start` command
            SubcommandsType::Start {
                name,
                command,
                restart,
            } => {
                actions::commands::macos_specific::start_process(name, command, restart);
            }
            SubcommandsType::Stop { id } => {
                actions::commands::macos_specific::stop_process(id);
            }
            SubcommandsType::List => {
                actions::commands::macos_specific::list_processes();
            }
        }
    }
}
