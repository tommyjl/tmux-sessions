use clap::Clap;

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

#[derive(Clap)]
struct StopOpts {
    name: String,
}

#[derive(Clap)]
struct RestartOpts {
    name: String,
}

fn main() {
    let opts: TmuxSessionsOpts = TmuxSessionsOpts::parse();

    match opts.subcmd {
        Command::Start(opts) => println!("Starting {}", opts.name),
        Command::Stop(opts) => println!("Stopping {}", opts.name),
        Command::Restart(opts) => println!("Restarting {}", opts.name),
    };
}
