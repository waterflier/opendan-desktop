mod api;
// mod long_cmd;

use crate::api::download::*;
// use crate::api::{api_handler, shell_open};
use async_std::task;
use env_logger::Builder;
use log::{info, LevelFilter};
use tide_serve_dir_macro::auto_serve_dir;

async fn open_home_page() -> tide::Result<()> {
    task::sleep(std::time::Duration::from_millis(300)).await;
    // shell_open("http://localhost:9812/").await?;
    Ok(())
}

async fn async_main() -> tide::Result<()> {
    let app_state = AppState::new();
    let mut app = tide::with_state(app_state);

    auto_serve_dir!(app, "/", "./static/");
    app.at("/").serve_file("./static/index.html")?;
    // app.at("/api").post(api_handler);
    app.at("/api/dockerdesktop/start_download")
        .post(start_download);
    app.at("/api/dockerdesktop/get_status").get(get_status);

    task::spawn(open_home_page());
    app.listen("127.0.0.1:9812").await.unwrap();
    Ok(())
}

fn main() {
    let mut builder = Builder::from_default_env();
    builder.filter(None, LevelFilter::Info);
    builder.init();
    info!("paios booter start!");

    #[cfg(target_os = "macos")]
    {
        std::thread::spawn(|| {
            let _ = task::block_on(async_main());
        });

        let mut inner = tray.inner_mut();
        inner.add_quit_item("Quit");
        inner.display();
    }

    #[cfg(target_os = "windows")]
    {
        let _ = task::block_on(async_main());
    }

    println!("aios boot loader down!");
}
