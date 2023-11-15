use std::path::PathBuf;
use sysinfo::{ProcessExt, SystemExt};
use tokio::{io::AsyncWriteExt, process::Command};

const PROFILE_NAME: &str = "bramlett";

pub fn get_profile_path() -> Option<PathBuf> {
    dirs::home_dir()
        .unwrap()
        .join(".mozilla")
        .join("firefox")
        .read_dir()
        .unwrap()
        .filter_map(Result::ok)
        .find(|e| e.file_name().to_string_lossy().contains(PROFILE_NAME))
        .map(|e| e.path())
}

pub async fn launch(create_new_profile: bool) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("launching firefox");

    if create_new_profile {
        Command::new("firefox")
            .arg("-CreateProfile")
            .arg(PROFILE_NAME)
            .arg("-no-remote")
            .spawn()?
            .wait()
            .await?;
    }

    let mut sys = sysinfo::System::new();

    let existing = if create_new_profile {
        sys.refresh_processes();
        sys.processes_by_name("firefox")
            .map(ProcessExt::pid)
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    Command::new("firefox")
        .arg("-P")
        .arg(PROFILE_NAME)
        .arg("-no-remote")
        .spawn()?;

    if create_new_profile {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        sys.refresh_processes();

        sys.processes_by_name("firefox")
            .find(|p| !existing.contains(&p.pid()))
            .ok_or("firefox process not found")?
            .kill();

        let profile_path = get_profile_path().unwrap();

        tracing::info!("profile path: {profile_path:#?}");

        let prefs = profile_path.join("prefs.js");

        let mut prefs = tokio::fs::OpenOptions::new()
            .read(true)
            .append(true)
            .open(prefs)
            .await
            .unwrap();

        // proxy SOCKS to 127.0.0.1:8636 w/ remote DNS
        let conf = br#"
            user_pref("network.proxy.socks", "127.0.0.1");
            user_pref("network.proxy.socks_port", 8636);
            user_pref("network.proxy.socks_remote_dns", true);
            user_pref("network.proxy.type", 1);
        "#;

        prefs.write_all(conf).await?;

        Command::new("firefox")
            .arg("-P")
            .arg(PROFILE_NAME)
            .arg("-no-remote")
            .spawn()?;
    }

    Ok(())
}
