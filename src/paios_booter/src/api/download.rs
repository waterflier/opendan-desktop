use async_recursion::async_recursion;
use async_std::fs::File;
use async_std::io::prelude::*;
use async_std::task;
use log::{error, info};
use std::sync::{Arc, Mutex};
use surf;
use tide::Request; // 引入额外的 trait 以便使用 async read

// use crate::api::def::*;

// 下载状态枚举
#[derive(Clone, Debug)]
pub enum DownloadState {
    NotStarted,
    InProgress,
    Completed,
}

// 全局状态
#[derive(Clone)]
pub struct AppState {
    download_state: Arc<Mutex<DownloadState>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            download_state: Arc::new(Mutex::new(DownloadState::NotStarted)),
        }
    }
}

// 启动下载的处理函数
pub async fn start_download(req: Request<AppState>) -> tide::Result {
    let mut state_lock = req.state().download_state.lock().unwrap();

    match *state_lock {
        DownloadState::NotStarted | DownloadState::Completed => {
            // 如果下载未开始或已完成，启动下载任务
            *state_lock = DownloadState::InProgress;

            // 异步下载文件
            let state = req.state().clone();
            task::spawn(async move {
                let url = "https://desktop.docker.com/win/stable/Docker Desktop Installer.exe";
                if let Err(e) = download_file(state, url.to_string()).await {
                    error!("Download error: {}", e);
                }
            });
            success_response!("Download started")
        }
        DownloadState::InProgress => {
            info!("target file downloading");
            // 如果下载正在进行中，返回正在下载
            success_response!("Download is in progress")
        }
    }
}

// 查询下载状态的处理函数
pub async fn get_status(req: Request<AppState>) -> tide::Result {
    let state_lock = req.state().download_state.lock().unwrap();

    let status_message = match *state_lock {
        DownloadState::NotStarted => "Download not started",
        DownloadState::InProgress => "Download in progress",
        DownloadState::Completed => "Download completed",
    };
    success_response!(status_message)
}

// 异步下载文件，并更新状态
#[async_recursion]
async fn download_file(state: AppState, url: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("start to down load file, url {}", url);

    // let url = "https://desktop.docker.com/win/stable/Docker Desktop Installer.exe";
    let output = "Docker Desktop Installer.exe";

    let mut res = surf::get(url).await?;

    // 检查状态码是否为重定向
    if res.status().is_redirection() {
        // 如果有 Location 头部，则进行重定向
        if let Some(new_url) = res.header("Location") {
            let target = new_url.last().as_str();
            info!("download_file Redirecting to: {}", target);
            return download_file(state, target.to_string()).await;
        }
    }

    if res.status().is_success() {
        let total_size = res
            .header("Content-Length")
            .and_then(|values| values.last().as_str().parse::<u64>().ok())
            .unwrap_or(0);
        info!("download_file total size: {}", total_size);

        let mut downloaded: u64 = 0;
        let mut stream = res.take_body();
        let mut buf = vec![0; 1024 * 1024]; // 1 MB buffer

        let mut last_reported_progress: f32 = 0.0; // 用于记录上次报告的进度
        let mut file = File::create(output).await?;
        while let Ok(n) = futures::io::AsyncReadExt::read(&mut stream, &mut buf).await {
            if n == 0 {
                info!("download_file download complete in AsyncReadExt");
                break;
            }
            file.write_all(&buf[..n]).await?;
            downloaded += n as u64;

            // Update progress here
            let progress = if total_size > 0 {
                downloaded as f32 / total_size as f32 * 100.0
            } else {
                0.0 // If total size is unknown, we can't calculate progress
            };
            if progress - last_reported_progress >= 5.0 {
                info!("Download progress: {:.2}%", progress);
                last_reported_progress = progress; // 更新最后报告的进度
            }
        }

        // let body = res.body_bytes().await?;
        // file.write_all(&body).await?;
    } else {
        error!("Failed to download file: {}", res.status());
    }

    let mut state_lock = state.download_state.lock().unwrap();
    *state_lock = DownloadState::Completed;
    info!("complete to down load file");

    Ok(())
}
