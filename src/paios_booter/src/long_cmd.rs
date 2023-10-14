use async_std::task;
use async_std::io::prelude::*;
use async_std::process::{Command, Stdio};
use async_std::sync::{Arc, Mutex};
use std::time::Duration;
use std::collections::HashMap;
use lazy_static::lazy_static;

pub struct LongCommand {
    cmd: String,
    args: Vec<String>,
    collected_output: Arc<Mutex<Vec<u8>>>,
    last_printed_position: Arc<Mutex<usize>>,
    child_handle: Arc<Mutex<Option<async_std::process::Child>>>,
    reader_handle: Arc<Mutex<Option<task::JoinHandle<()>>>>,
    status: Arc<Mutex<Option<i32>>>, // Store the exit status here
}

impl LongCommand {
    pub fn new(cmd: &str, args: &[&str]) -> Self {
        LongCommand {
            cmd: cmd.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
            collected_output: Arc::new(Mutex::new(Vec::new())),
            last_printed_position: Arc::new(Mutex::new(0)),
            child_handle: Arc::new(Mutex::new(None)),
            reader_handle: Arc::new(Mutex::new(None)),
            status: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        
        let mut child = Command::new(&self.cmd)
            .args(&self.args)
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().expect("Failed to open stdout");
        *self.child_handle.lock().await = Some(child);
        let output_handle = self.collected_output.clone();

        let reader = task::spawn(async move {
            let mut buf = [0; 1024];
            let mut stdout = stdout;
            loop {
                let n = stdout.read(&mut buf).await.expect("Read failed");
                if n == 0 {
                    break;
                }
                output_handle.lock().await.extend(&buf[..n]);
            }
        });

        *self.reader_handle.lock().await = Some(reader);

        let status_handle = self.status.clone();
        let child = self.child_handle.lock().await.take();
        if let Some(mut child) = child {
            task::spawn(async move {
                if let Ok(exit_status) = child.status().await {
                    let code = exit_status.code().unwrap_or(-1);
                    *status_handle.lock().await = Some(code);
                }
            });
        }
        
        Ok(())
    }

    pub async fn get_new_output(&self) -> String {
        let output = self.collected_output.lock().await.clone();
        let mut last_position = self.last_printed_position.lock().await;
        let new_output = String::from_utf8_lossy(&output[*last_position..]).into_owned();
        *last_position = output.len();
        new_output
    }

    pub async fn is_completed(&self) -> bool {
        self.status.lock().await.is_some()
    }

    pub async fn kill(&self) -> std::io::Result<()> {
        if let Some(child) = &mut *self.child_handle.lock().await {
            child.kill();
        }
        Ok(())
    }

    pub async fn wait_for_completion(&self) {
        if let Some(handle) = &mut *self.reader_handle.lock().await {
            handle.await;
        }
    }
}

pub struct LongCommandManager {
    commands: Mutex<HashMap<String, Arc<LongCommand>>>,
}

impl LongCommandManager {
    pub fn new() -> Self {
        LongCommandManager {
            commands: Mutex::new(HashMap::new()),
        }
    }

    pub async fn add_command(&self, key: String, cmd: LongCommand) {
        let mut commands = self.commands.lock().await;
        commands.insert(key, Arc::new(cmd));
    }

    pub async fn get_command(&self, key: &str) -> Option<Arc<LongCommand>> {
        let commands = self.commands.lock().await;
        commands.get(key).cloned()
    }

    pub async fn remove_command(&self, key: &str) {
        let mut commands = self.commands.lock().await;
        commands.remove(key);
    }
}

lazy_static! {
    static ref LONG_COMMAND_MANAGER: LongCommandManager = LongCommandManager::new();
}

// Provide a global function to access the singleton
pub fn get_long_command_manager() -> &'static LongCommandManager {
    &LONG_COMMAND_MANAGER
}
