mod managers;
mod types;

use managers::linux::standard::BaseManager

use japm_common::{ProcessInfo, ProcessSpec, Request, Response, Status};
use std::{collections::HashMap, path::PathBuf};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, Command},
    sync::Mutex,
};
use tracing::{error, info};

#[cfg(unix)]
use tokio::net::{UnixListener, UnixStream};

type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Default)]
struct State {
    procs: HashMap<String, ManagedProc>,
}

struct ManagedProc {
    spec: ProcessSpec,
    child: Option<Child>,
    restarts: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    // logging with daily rotation
    let file_appender = tracing_appender::rolling::daily(log_dir()?, "daemon.log");
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_writer(file_appender)
        .init();

    let state = tokio::sync::Arc::new(Mutex::new(State::default()));

    #[cfg(unix)]
    {
        let sock = socket_path()?;
        // remove stale socket
        let _ = tokio::fs::remove_file(&sock).await;
        let listener = UnixListener::bind(&sock)?;
        info!(path=?sock, "daemon listening");

        loop {
            let (stream, _) = listener.accept().await?;
            let st = state.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_client(st, stream).await {
                    error!(?e, "client handler error");
                }
            });
        }
    }

    #[cfg(windows)]
    compile_error!("Windows transport not implemented yet: switch to TCP or Named Pipes");
}

#[cfg(unix)]
async fn handle_client(
    state: tokio::sync::Arc<Mutex<State>>,
    mut stream: UnixStream,
) -> Result<()> {
    let (r, mut w) = stream.split();
    let mut reader = BufReader::new(r);
    let mut line = String::new();

    while reader.read_line(&mut line).await? > 0 {
        let req: Request = serde_json::from_str(&line.trim())?;
        line.clear();
        let resp = dispatch(state.clone(), req).await;
        let json = serde_json::to_string(&resp)? + "\n";
        w.write_all(json.as_bytes()).await?;
    }
    Ok(())
}

async fn dispatch(state: tokio::sync::Arc<Mutex<State>>, req: Request) -> Response {
    match req {
        Request::Start(spec) => match start_proc(state, spec).await {
            Ok(id) => Response::Started { id },
            Err(e) => Response::Err(e.to_string()),
        },
        Request::Stop { id } => match stop_proc(state, &id).await {
            Ok(_) => Response::Ok,
            Err(e) => Response::Err(e.to_string()),
        },
        Request::List => {
            let s = state.lock().await;
            let list = s
                .procs
                .iter()
                .map(|(id, m)| ProcessInfo {
                    id: id.clone(),
                    name: m.spec.name.clone(),
                    pid: m.child.as_ref().and_then(|c| c.id()),
                    status: if let Some(ch) = &m.child {
                        if ch.id().is_some() {
                            Status::Running
                        } else {
                            Status::Starting
                        }
                    } else {
                        Status::Exited(0)
                    },
                    restarts: m.restarts,
                })
                .collect();
            Response::List(list)
        }
    }
}
fn log_dir() -> Result<PathBuf> {
    Ok(dirs::data_local_dir()
        .ok_or_else(|| anyhow::anyhow!("no data dir"))?
        .join("rusty-pm"))
}
