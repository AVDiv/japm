use serde::{Deserialize, Serialize};

pub type ProcId = String; // could be UUID later

#[derive(Deserialize, Serialize, Debug)]
pub enum Request {
    Start(ProcessSpec),
    Stop { id: ProcId },
    List,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Ok,
    Started { id: ProcId },
    List(Vec<ProcessInfo>),
    Err(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessSpec {
    pub name: String,
    pub cmd: String,
    pub args: Vec<String>,
    pub cwd: Option<String>,
    pub env: Vec<(String, String)>,
    pub restart: RestartPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestartPolicy {
    Never,
    OnFailure,
    Always,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub id: ProcId,
    pub name: String,
    pub pid: Option<u32>,
    pub status: Status,
    pub restarts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Starting,
    Running,
    Exited(i32),
    Failed(String),
}
