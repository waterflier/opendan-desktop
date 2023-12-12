use async_std::process::Command;
use log::{error, info};
use serde::{Deserialize, Serialize, Serializer};
use std::env;
use std::error::Error;
use tide::prelude::*;
use tide::Response;
// use tokio::fs::File;
// use tokio::io::AsyncWriteExt;

use crate::api::def::*;

macro_rules! error_response {
    ($e:expr, $code:expr) => {{
        RpcResponse {
            result: json!(""),
            error: Some($e.to_string()),
            code: $code,
        }
    }};
}

macro_rules! success_response {
    ($msg:expr) => {{
        RpcResponse {
            result: json!($msg),
            error: None,
            code: ErrorCode::Ok,
        }
    }};
}

async fn check_docker_service() -> tide::Result<RpcResponse> {
    match Command::new("docker").arg("--version").output().await {
        Ok(output) => {
            if !output.status.success() {
                return Ok(error_response!("output failed", ErrorCode::InternalError));
            }
            let output_str = String::from_utf8_lossy(&output.stdout);
            let version: Vec<&str> = output_str.split_whitespace().collect();
            if version.len() > 2 {
                return Ok(success_response!(version[2].to_string()));
            } else {
                error!("docker parse version failed");

                Ok(error_response!(
                    "version parse failed",
                    ErrorCode::InternalError
                ))
            }
        }
        Err(e) => {
            error!("docker check failed {}", e.to_string());
            let rpc_resp = error_response!(e, ErrorCode::CommandError);
            Ok(rpc_resp)
        }
    }
}

async fn try_install_docker() -> tide::Result<RpcResponse> {
    let os = env::consts::OS;
    match os {
        "windows" => {
            let url = "https://desktop.docker.com/win/stable/Docker Desktop Installer.exe";
            let output = "Docker Desktop Installer.exe";
            info!("win download Docker Desktop Installer");
            // info!("win install Docker Desktop Installer");
            // let install_cmd = r#"
            // Start-Process -Wait "Docker Desktop Installer.exe"
            // "#;
            // Command::new("powershell")
            //     .args(&["-Command", install_cmd])
            //     .output()
            //     .await?;
            Ok(success_response!("downloading"))
        }
        "macos" => {
            let url = "https://desktop.docker.com/mac/stable/Docker.dmg";
            info!("download Docker Desktop Installer");
            Command::new("curl").args(&["-LO", url]).output().await?;

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
            Ok(success_response!(""))
        }
        e => {
            eprintln!("Unsupported OS for automatic Docker installation");
            let rpc_resp = error_response!(e, ErrorCode::CommandError);
            Ok(rpc_resp)
        }
    }
}

async fn container_exists(container_name: &str) -> tide::Result<bool> {
    let output = Command::new("docker")
        .args(&["ps", "-a", "--format", "{{.Names}}"])
        .output()
        .await
        .expect("Failed to run docker command");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return Ok(output_str.lines().any(|line| line == container_name));
    }

    Ok(false)
}

async fn update_paios() -> tide::Result<i32> {
    let status = Command::new("docker")
        .args(&["pull", "paios/aios:latest"])
        .status()
        .await?;

    if !status.success() {
        return Ok(1);
    }

    Ok(0)
}

async fn start_paios() -> tide::Result<bool> {
    let is_container_exists = container_exists("aios").await?;

    if is_container_exists {
        let status = Command::new("docker")
            .args(&["start", "aios"])
            .status()
            .await?;
    } else {
        let status = Command::new("docker")
            .args(&[
                "run",
                "-v",
                "~/myai:/root/myai",
                "--name",
                "aios",
                "paios/aios:latest",
            ])
            .status()
            .await?;
    }
    Ok(true)
}

async fn stop_paios() -> tide::Result<bool> {
    let status = Command::new("docker")
        .arg("stop")
        .arg("aios")
        .status()
        .await?;

    if !status.success() {
        return Ok(false);
    }

    return Ok(true);
}

pub async fn shell_open(url: &str) -> tide::Result<()> {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer").arg(url).status().await?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(url).status().await?;
    }

    Ok(())
}

//---------------------------------------------------------------------------------------

pub async fn api_handler(mut req: tide::Request<()>) -> tide::Result {
    let rpc_req: RpcRequest = req.body_json().await?;
    let mut resp = Response::new(tide::StatusCode::Ok);
    match rpc_req.function_name.as_str() {
        "check_docker_version" => {
            let rpc_resp = check_docker_service().await?;
            let json_string = serde_json::to_string(&rpc_resp).unwrap();
            resp.set_body(json_string);
        }
        "try_install_docker" => {
            let rpc_resp = try_install_docker().await?;
            let json_string = serde_json::to_string(&rpc_resp).unwrap();
            resp.set_body(json_string);
        }
        "start_aios" => {}
        "stop_aios" => {}
        "update_aios" => {}
        "exec_cmd" => {
            let params = rpc_req.params.unwrap();
            let cmd = params["cmd"].as_str().unwrap();
            let args = params["args"].as_array().unwrap();
            let mut args_vec = Vec::new();
            for arg in args {
                args_vec.push(arg.as_str().unwrap());
            }
            let output = Command::new(cmd).args(&args_vec).output().await?;
            let output_str = String::from_utf8_lossy(&output.stdout);
            let rpc_resp = RpcResponse {
                result: json!(output_str),
                error: None,
                code: ErrorCode::Ok,
            };
            let json_string = serde_json::to_string(&rpc_resp).unwrap();
            resp.set_body(json_string);
        }
        "exec_long_cmd" => {
            let params = rpc_req.params.unwrap();
            let cmd = params["cmd"].as_str().unwrap();
            let cmd_id: &str;
            if params["cmd_id"].is_null() {
                cmd_id = cmd
            } else {
                cmd_id = params["cmd_id"].as_str().unwrap();
            }

            if get_long_command_manager()
                .get_command(cmd_id)
                .await
                .is_some()
            {
                let rpc_resp = RpcResponse {
                    result: json!(1),
                    error: Some(String::from("command already exists")),
                    code: ErrorCode::Ok,
                };
                let json_string = serde_json::to_string(&rpc_resp).unwrap();
                resp.set_body(json_string);
                return Ok(resp);
            }

            let args = params["args"].as_array().unwrap();
            let mut args_vec = Vec::new();
            for arg in args {
                args_vec.push(arg.as_str().unwrap());
            }
            let long_cmd = LongCommand::new(cmd, &args_vec);
            long_cmd.run().await?;
            get_long_command_manager()
                .add_command(String::from(cmd_id), long_cmd)
                .await;
            let rpc_resp = RpcResponse {
                result: json!(0),
                error: None,
                code: ErrorCode::Ok,
            };
            let json_string = serde_json::to_string(&rpc_resp).unwrap();
            resp.set_body(json_string);
        }
        "get_cmd_new_output" => {
            let params = rpc_req.params.unwrap();
            let cmd_id = params["cmd_id"].as_str().unwrap();
            let long_cmd = get_long_command_manager().get_command(cmd_id).await;
            if long_cmd.is_none() {
                let rpc_resp = RpcResponse {
                    result: json!(""),
                    error: Some(String::from("error")),
                    code: ErrorCode::Ok,
                };
                let json_string = serde_json::to_string(&rpc_resp).unwrap();
                resp.set_body(json_string);
                return Ok(resp);
            }

            let long_cmd = long_cmd.unwrap();
            let new_output = long_cmd.get_new_output().await;
            let mut err_str = None;
            if long_cmd.is_completed().await {
                err_str = Some(String::from("done"));
                get_long_command_manager().remove_command(cmd_id).await;
            }
            let rpc_resp = RpcResponse {
                result: json!(new_output),
                error: err_str,
                code: ErrorCode::Ok,
            };
            let json_string = serde_json::to_string(&rpc_resp).unwrap();
            resp.set_body(json_string);
        }
        "remove_cmd" => {
            let params = rpc_req.params.unwrap();
            let cmd_id = params["cmd_id"].as_str().unwrap();
            get_long_command_manager().remove_command(cmd_id).await;
            let rpc_resp = RpcResponse {
                result: json!(0),
                error: None,
                code: ErrorCode::Ok,
            };
            let json_string = serde_json::to_string(&rpc_resp).unwrap();
            resp.set_body(json_string);
        }
        _ => {
            let rpc_resp = RpcResponse {
                result: json!(1),
                error: Some(String::from("Unknown function name")),
                code: ErrorCode::Ok,
            };
            let json_string = serde_json::to_string(&rpc_resp).unwrap();
            resp.set_body(json_string);
        }
    }

    Ok(resp)
}
