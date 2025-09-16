use crate::types::cli::other::RestartPolicyCli;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum SubcommandsType {
    //// Start a process
    Start {
        name: String,
        #[arg(num_args = 0.., trailing_var_arg = true)]
        command: Vec<String>,
        #[arg(long, value_enum, default_value = "on-failure")]
        restart: RestartPolicyCli,
    },
    //// Stop a managed process
    Stop {
        id: String,
    },
    //// List processes
    List,
}
