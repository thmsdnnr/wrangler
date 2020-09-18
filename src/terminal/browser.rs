use std::process::{Command, Stdio};

pub fn open_browser(url: &str) -> Result<(), failure::Error> {
    if cfg!(target_os = "windows") {
        let url_escaped = url.replace("&", "^&");
        let windows_cmd = format!("start {}", url_escaped);
        Command::new("cmd")
            .args(&["/C", &windows_cmd])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    } else if cfg!(target_os = "linux") {
        let linux_cmd = format!(r#"xdg-open "{}""#, url);
        Command::new("sh")
            .arg("-c")
            .arg(&linux_cmd)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    } else {
        let mac_cmd = format!(r#"open "{}""#, url);
        Command::new("sh")
            .arg("-c")
            .arg(&mac_cmd)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    };

    Ok(())
}
