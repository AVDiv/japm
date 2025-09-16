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

    match &args.command {
        // `start` command
        SubcommandsType::Start {
            name,
            command,
            restart,
        } => {
            actions::commands::start_process(name, command, restart);
        }
        SubcommandsType::Stop { id } => {
            actions::commands::stop_process(id);
        }
        SubcommandsType::List => {
            actions::commands::list_processes();
        }
    }
}
