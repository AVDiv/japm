use std::{collections::btree_set::Union, process::Child};

pub enum ChildProcessStatus {
    Pending,
    Sleeping,
    Running,
    Exited,
}

pub struct ChildProcess {
    pub command: String,
    pub spawn_time: DateTime<Local>,
    pub status: Status,
    pub exit_code: Option<i16>,
    pub process_instance: Union<Child>,
}

impl ChildProcess {}
