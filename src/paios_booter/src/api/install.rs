use crate::api::{AppState, ErrorCode};
use async_std::process::Command;
use log::{error, info};
use serde_json::json;
use tide::Request;

pub async fn check_docker_service(_: Request<AppState>) -> tide::Result {
    match Command::new("docker").arg("--version").output().await {
        Ok(output) => {
            if !output.status.success() {
                error_response!("output failed", ErrorCode::InternalError)
            }

            let output_str = String::from_utf8_lossy(&output.stdout);
            let version: Vec<&str> = output_str.split_whitespace().collect();
            if version.len() > 2 {
                info!("docker version: {}", version[2]);
                success_response!(json!({
                    "version": version[2],
                    "state": 2,
                    "message": "output failed",
                }))
            } else {
                error!("docker parse version failed");
                error_response!("version parse failed", ErrorCode::InternalError)
            }
        }
        Err(e) => {
            error!("docker check failed {}", e.to_string());
            error_response!("docker check failed: no command", ErrorCode::CommandError)
        }
    }
}
