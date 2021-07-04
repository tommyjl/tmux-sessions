mod config;
mod tmux;

use anyhow::{anyhow, Result};
use clap::{crate_authors, crate_version, Clap};
use config::get_config;
use tmux::{list_sessions, Session};

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct TmuxSessionsOpts {
    #[clap(subcommand)]
    subcmd: Command,
}

#[derive(Clap)]
enum Command {
    Start {
        #[clap(short, long, default_value = "~/.config/tsesh/config.toml")]
        config: String,
        name: String,
    },
    Stop {
        name: String,
    },
    Restart {
        #[clap(short, long, default_value = "~/.config/tsesh/config.toml")]
        config: String,
        name: String,
    },
}

fn start(config: String, name: String) -> Result<()> {
    if list_sessions()?.contains(&name) {
        Err(anyhow!("Session '{}' already exists", name))
    } else {
        let session_config = get_config(&config, &name)?;
        let mut session = Session::new(&name)?;
        for window in session_config.windows {
            session = session.new_window(window)?;
        }
        Ok(())
    }
}

fn stop(name: String) -> Result<()> {
    if !crate::tmux::list_sessions()?.contains(&name) {
        Err(anyhow!("Session '{}' does not exist", &name))
    } else {
        let session = Session::new(&name)?;
        session.kill()?;
        Ok(())
    }
}

fn restart(config: String, name: String) -> Result<()> {
    stop(name.clone())?;
    start(config, name)?;
    Ok(())
}

fn main() -> Result<()> {
    match TmuxSessionsOpts::parse().subcmd {
        Command::Start { config, name } => start(config, name),
        Command::Stop { name } => stop(name),
        Command::Restart { config, name } => restart(config, name),
    }
}
