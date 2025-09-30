use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum RestartPolicyCli {
    Never,
    OnFailure,
    Always,
}
