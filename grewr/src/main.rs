use clap::Parser;

use crate::command::Command;

mod command;
mod options;

fn main() {
    let args = options::Args::parse();
    dbg!(&args);
    match args.cmd {
        options::Command::Clean(cmd) => cmd.execute(),
        options::Command::Compile(cmd) => cmd.execute(),
        options::Command::Count(cmd) => cmd.execute(),
        options::Command::Grep(cmd) => cmd.execute(),
        options::Command::Transform(cmd) => cmd.execute(),
    }
}
