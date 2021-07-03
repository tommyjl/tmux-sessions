use anyhow::Result;
use std::process::Command;

pub fn list_sessions() -> Result<Vec<String>> {
    let output = Command::new("tmux")
        .arg("list-sessions")
        .arg("-F")
        .arg("#S")
        .output()?;
    let output = String::from_utf8(output.stdout)?;
    Ok(output.split("\n").map(|s| s.to_string()).collect())
}

pub struct Session {
    name: String,
}

pub struct Window {
    pub name: String,
    pub working_dir: String,
    pub cmd: String,
}

impl Session {
    pub fn new(name: &str) -> Result<Session> {
        Command::new("tmux")
            .arg("new-session")
            .arg("-s")
            .arg(name)
            .arg("-d")
            .output()?;

        Ok(Session {
            name: String::from(name),
        })
    }

    pub fn kill(self) -> Result<Session> {
        Command::new("tmux")
            .arg("kill-session")
            .arg("-t")
            .arg(&self.name)
            .spawn()?
            .wait()?;
        Ok(self)
    }

    pub fn new_window(self, window: Window) -> Result<Session> {
        Command::new("tmux")
            .arg("new-window")
            .arg("-t")
            .arg(&self.name)
            .arg("-n")
            .arg(window.name)
            .arg("-c")
            .arg(window.working_dir)
            .arg(window.cmd)
            .spawn()?
            .wait()?;
        Ok(self)
    }
}
