mod config;
mod tmux;

use anyhow::{anyhow, Result};
use clap::Clap;
use config::get_config;
use tmux::{list_sessions, Session, Window};

#[derive(Clap)]
struct TmuxSessionsOpts {
    #[clap(subcommand)]
    subcmd: Command,
}

#[derive(Clap)]
enum Command {
    Start(StartOpts),
    Stop(StopOpts),
    Restart(RestartOpts),
}

#[derive(Clap)]
struct StartOpts {
    name: String,
}

fn start(opts: StartOpts) -> Result<()> {
    if list_sessions()?.contains(&opts.name) {
        Err(anyhow!("Session '{}' already exists", &opts.name))
    } else {
        let session_config = get_config(&opts.name)?;
        let mut session = Session::new(&opts.name)?;
        for window in session_config.windows {
            let window = Window {
                name: window.name,
                working_dir: window.working_dir,
                cmd: window.cmd,
            };
            session = session.new_window(window)?;
        }
        Ok(())
    }
}

#[derive(Clap)]
struct StopOpts {
    name: String,
}

fn stop(opts: StopOpts) -> Result<()> {
    if !crate::tmux::list_sessions()?.contains(&opts.name) {
        Err(anyhow!("Session '{}' does not exist", &opts.name))
    } else {
        let session = Session::new(&opts.name)?;
        session.kill()?;
        Ok(())
    }
}

#[derive(Clap)]
struct RestartOpts {
    name: String,
}

fn restart(opts: RestartOpts) -> Result<()> {
    if !crate::tmux::list_sessions()?.contains(&opts.name) {
        Err(anyhow!("Session '{}' does not exist", &opts.name))
    } else {
        Session::new(&opts.name)?.kill()?;
        Session::new(&opts.name)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    match TmuxSessionsOpts::parse().subcmd {
        Command::Start(opts) => start(opts),
        Command::Stop(opts) => stop(opts),
        Command::Restart(opts) => restart(opts),
    }
}
