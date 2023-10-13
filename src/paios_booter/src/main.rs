
use std::{io, env};

use async_std::{process::Command, task};
use async_std::prelude::*;

use tide::{prelude::*};
use tide_serve_dir_macro::auto_serve_dir;
use tide::{Request, Response, StatusCode};

use serde::{Deserialize, Serialize};
use tray_item::{TrayItem, IconSource};

#[derive(Debug, Deserialize)]
struct InputData {
    message: String,
}

#[derive(Serialize)]
struct OutputData {
    reply: String,
}


async fn get_task_status(task_id:u32) -> tide::Result<()> {
    Ok(())
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



async fn api_handler(mut req: tide::Request<()>) -> tide::Result {
    let input: InputData = req.body_json().await?;
    let output = OutputData {
        reply: format!("Hello, {}!", input.message),
    };
    let json_string = serde_json::to_string(&output).unwrap();
    let mut resp = Response::new(tide::StatusCode::Ok);
    resp.set_body(json_string);
    Ok(resp)
  }

async fn open_home_page() -> tide::Result<()> {
    task::sleep(std::time::Duration::from_millis(300)).await;
    shell_open("http://localhost:9812/index.html").await?;
    Ok(())
}

async fn async_main() -> tide::Result<()> {
    let mut app = tide::new();
    
    auto_serve_dir!(app, "/", "./boot_web/");
    app.at("/api").post(api_handler);
    let open_home_page_task = task::spawn(open_home_page());
    app.listen("127.0.0.1:9812").await.unwrap(); 

    Ok(())
}


fn main() {
    std::thread::spawn(|| {
        0
    });

    let mut tray = TrayItem::new("Tray Example", IconSource::Resource("name-of-icon-in-rc-file")).unwrap();

    tray.add_label("Tray Label").unwrap();

    tray.add_menu_item("Open", || {
        let _ = async_std::task::block_on(shell_open("http://localhost:9812/index.html")); 
    }).unwrap();


    let _ = async_std::task::block_on(async_main()); 
    println!("server down!");
  }