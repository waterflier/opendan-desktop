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

pub async fn docker_install(_: Request<AppState>) -> tide::Result {
    let os = std::env::consts::OS;
    match os {
        "windows" => {
            // let output = "Docker Desktop Installer.exe";
            info!("win install Docker Desktop Installer");
            let install_cmd = r#"
            Start-Process -Wait "Docker Desktop Installer.exe"
            "#;
            Command::new("powershell")
                .args(&["-Command", install_cmd])
                .output()
                .await?;
            success_response!(json!({
                "message": "install success",
            }))
        }
        "macos" => {
            // let url = "https://desktop.docker.com/mac/stable/Docker.dmg";
            info!("download Docker Desktop Installer");
            // Command::new("curl").args(&["-LO", url]).output().await?;
            info!("install Docker Desktop Installer");
            let install_cmd = r#"
            hdiutil attach Docker.dmg
            cp -R /Volumes/Docker/Docker.app /Applications/
            hdiutil detach /Volumes/Docker/
            "#;
            Command::new("sh")
                .arg("-c")
                .arg(install_cmd)
                .output()
                .await?;
            success_response!(json!({
                "message": "install success",
            }))
        }
        e => {
            eprintln!("Unsupported OS for automatic Docker installation");
            error_response!(e, ErrorCode::CommandError)
        }
    }
}
