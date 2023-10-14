
mod long_cmd;

use std::arch::x86_64::_CMP_ORD_Q;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use std::{io, env};

use async_std::sync::Mutex;
use async_std::{process::Command, task};
use async_std::prelude::*;

use tide::{prelude::*};
use tide_serve_dir_macro::auto_serve_dir;
use tide::{Request, Response, StatusCode};

use serde::{Deserialize, Serialize};
use tray_item::{TrayItem, IconSource};

use long_cmd::*;

#[derive(Serialize, Deserialize, Debug)]
struct RpcRequest {
    function_name: String,
    params: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RpcResponse {
    result: serde_json::Value,
    error: Option<String>,
}


async fn check_docker_service() -> tide::Result<u32> {
    let output = Command::new("docker")
        .arg("--version")
        .output().await?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let version: Vec<&str> = output_str.split_whitespace().collect();
        if version.len() > 2 {
            return Ok(version[2].to_string().parse::<u32>().unwrap());
        }
    }
    
    Ok(0)    
}

async fn try_install_docker() -> tide::Result<i32> {
    let os = env::consts::OS;
    match os {
        "windows" => {
            let download_cmd = r#"
            $url = "https://desktop.docker.com/win/stable/Docker Desktop Installer.exe"
            $output = "Docker Desktop Installer.exe"
            Invoke-WebRequest -Uri $url -OutFile $output
            "#;
            Command::new("powershell")
                .args(&["-Command", download_cmd])
                .output().await?;

            let install_cmd = r#"
            Start-Process -Wait "Docker Desktop Installer.exe"
            "#;
            Command::new("powershell")
                .args(&["-Command", install_cmd])
                .output().await?;
        },
        "macos" => {
            let url = "https://desktop.docker.com/mac/stable/Docker.dmg";

            Command::new("curl")
                .args(&["-LO", url])
                .output().await?;


            let install_cmd = r#"
            hdiutil attach Docker.dmg
            cp -R /Volumes/Docker/Docker.app /Applications/
            hdiutil detach /Volumes/Docker/
            "#;
            Command::new("sh")
                .arg("-c")
                .arg(install_cmd)
                .output().await?;
        },
        _ => {
            eprintln!("Unsupported OS for automatic Docker installation");
            return Ok(1);
        }
    }

    Ok(0)
}

async fn container_exists(container_name: &str) -> tide::Result<bool> {
    let output = Command::new("docker")
        .args(&["ps", "-a", "--format", "{{.Names}}"])
        .output()
        .await
        .expect("Failed to run docker command");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return Ok(output_str.lines().any(|line| line == container_name))
    }

    Ok(false)
}

async fn update_paios() -> tide::Result<i32> {
    let status = Command::new("docker")
        .args(&["pull", "paios/aios:latest"])
        .status()
        .await?;

    if !status.success() {
        return Ok(1)
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
            .args(&["run", "-v","~/myai:/root/myai", "--name", "aios","paios/aios:latest"])
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

async fn shell_open(url:&str) -> tide::Result<()> {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(url)
            .status().await?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(url)
            .status().await?;
    }

    Ok(())
}

//---------------------------------------------------------------------------------------

async fn api_handler(mut req: tide::Request<()>) -> tide::Result {
    let rpc_req: RpcRequest = req.body_json().await?;
    let mut resp = Response::new(tide::StatusCode::Ok);
    match rpc_req.function_name.as_str() {
        "check_docker_version" => {
            let version = check_docker_service().await?;
            let rpc_resp = RpcResponse {
                result: json!(version),
                error: None
            };
            let json_string = serde_json::to_string(&rpc_resp).unwrap();
            resp.set_body(json_string);

        },
        "try_install_docker" => {

        },
        "start_aios"=> {

        },
        "stop_aios" => {

        },
        "update_aios" => {

        },
        "exec_cmd" => {
            let params = rpc_req.params.unwrap();
            let cmd = params["cmd"].as_str().unwrap();
            let args = params["args"].as_array().unwrap();
            let mut args_vec = Vec::new();
            for arg in args {
                args_vec.push(arg.as_str().unwrap());
            }
            let output = Command::new(cmd)
                .args(&args_vec)
                .output().await?;
            let output_str = String::from_utf8_lossy(&output.stdout);
            let rpc_resp = RpcResponse {
                result: json!(output_str),
                error: None
            };
            let json_string = serde_json::to_string(&rpc_resp).unwrap();
            resp.set_body(json_string);        
        },
        "exec_long_cmd" => {
            let params = rpc_req.params.unwrap();
            let cmd = params["cmd"].as_str().unwrap();
            let cmd_id: &str;
            if params["cmd_id"].is_null() {
                cmd_id = cmd
            } else {
                cmd_id = params["cmd_id"].as_str().unwrap();
            }

            if get_long_command_manager().get_command(cmd_id).await.is_some() {
                let rpc_resp = RpcResponse {
                    result: json!(1),
                    error: Some(String::from("command already exists"))
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
            get_long_command_manager().add_command(String::from(cmd_id), long_cmd).await;
            let rpc_resp = RpcResponse {
                result: json!(0),
                error: None
            };
            let json_string = serde_json::to_string(&rpc_resp).unwrap();
            resp.set_body(json_string);
        },
        "get_cmd_new_output" => {
            let params = rpc_req.params.unwrap();
            let cmd_id = params["cmd_id"].as_str().unwrap();
            let long_cmd = get_long_command_manager().get_command(cmd_id).await;
            if long_cmd.is_none() {
                let rpc_resp = RpcResponse {
                    result: json!(""),
                    error: Some(String::from("error"))
                };
                let json_string = serde_json::to_string(&rpc_resp).unwrap();
                resp.set_body(json_string);
                return Ok(resp);
            }

            let long_cmd = long_cmd.unwrap();
            let new_output = long_cmd.get_new_output().await;
            let mut err_str= None;
            if long_cmd.is_completed().await {
                err_str = Some(String::from("done"));
                get_long_command_manager().remove_command(cmd_id).await;
            }
            let rpc_resp = RpcResponse {
                result: json!(new_output),
                error: err_str
            };
            let json_string = serde_json::to_string(&rpc_resp).unwrap();
            resp.set_body(json_string);
        },
        "remove_cmd" => {
            let params = rpc_req.params.unwrap();
            let cmd_id = params["cmd_id"].as_str().unwrap();
            get_long_command_manager().remove_command(cmd_id).await;
            let rpc_resp = RpcResponse {
                result: json!(0),
                error: None
            };
            let json_string = serde_json::to_string(&rpc_resp).unwrap();
            resp.set_body(json_string);
        },
        _ => {
            let rpc_resp = RpcResponse {
                result: json!(1),
                error: Some(String::from("Unknown function name"))
            };
            let json_string = serde_json::to_string(&rpc_resp).unwrap();
            resp.set_body(json_string);
        }
    }

    Ok(resp)
  }

async fn open_home_page() -> tide::Result<()> {
    task::sleep(std::time::Duration::from_millis(300)).await;
    shell_open("http://localhost:9812/index.html").await?;
    Ok(())
}

async fn async_main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/api").post(api_handler);
    auto_serve_dir!(app, "/", "./boot_web/");

    let open_home_page_task = task::spawn(open_home_page());
    app.listen("127.0.0.1:9812").await.unwrap(); 

    Ok(())
}

fn main() {
    log::info!("paios booter start!");
    let mut tray = TrayItem::new("Tray Example", IconSource::Resource("name-of-icon-in-rc-file")).unwrap();
    tray.add_label("Tray Label").unwrap();
    tray.add_menu_item("Open", || {
        let _ = async_std::task::block_on(shell_open("http://localhost:9812/index.html")); 
    }).unwrap();

    #[cfg(target_os = "macos")]
    {
        std::thread::spawn(|| {
        
            let _ = async_std::task::block_on(async_main()); 
        });

        let mut inner = tray.inner_mut();
        inner.add_quit_item("Quit");
        inner.display();
    }

    #[cfg(target_os = "windows")]
    {
        let _ = async_std::task::block_on(async_main()); 
    }

    println!("aios boot loader down!");
  }