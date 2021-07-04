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
    Start(CommandOpts),
    Stop(CommandOpts),
    Restart(CommandOpts),
}

#[derive(Clap)]
struct CommandOpts {
    #[clap(short, long, default_value = "~/.config/tsesh/config.toml")]
    config: String,
    name: String,
}

fn start(opts: &CommandOpts) -> Result<()> {
    if list_sessions()?.contains(&opts.name) {
        Err(anyhow!("Session '{}' already exists", &opts.name))
    } else {
        let session_config = get_config(&opts.config, &opts.name)?;
        let mut session = Session::new(&opts.name)?;
        for window in session_config.windows {
            session = session.new_window(window)?;
        }
        Ok(())
    }
}

fn stop(opts: &CommandOpts) -> Result<()> {
    if !crate::tmux::list_sessions()?.contains(&opts.name) {
        Err(anyhow!("Session '{}' does not exist", &opts.name))
    } else {
        let session = Session::new(&opts.name)?;
        session.kill()?;
        Ok(())
    }
}

fn restart(opts: &CommandOpts) -> Result<()> {
    stop(opts)?;
    start(opts)?;
    Ok(())
}

fn main() -> Result<()> {
    match TmuxSessionsOpts::parse().subcmd {
        Command::Start(opts) => start(&opts),
        Command::Stop(opts) => stop(&opts),
        Command::Restart(opts) => restart(&opts),
    }
}
