#[derive(Debug, clap::Parser)]
#[clap(version, author)]
pub struct Args {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    Transform,
    Grep,
    Count,
    Compile,
    Clean,
}
