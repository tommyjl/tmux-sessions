# tmux-sessions

A command-line tool for managing sessions in tmux

## Installation

1. With homebrew: `brew install tommyjl/formulae/tsesh`
2. With cargo: `cargo install tsesh`

## Usage

1. Create a config file at `~/.config/tsesh/config.toml`. See [example.toml](./example.toml) for details.
2. Start a session with `tsesh start <session name>`
3. Stop a session with `tsesh stop <session name>`
4. Restart a session with `tsesh restart <session name>`
