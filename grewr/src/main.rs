use clap::Parser;

mod options;

fn main() {
    let args = options::Args::parse();
    dbg!(&args);
    match args.cmd {
        options::Command::Transform(cmd) => {
            dbg!(cmd.output_format());
        }
        options::Command::Count(cmd) => {
            dbg!(cmd.output_format());
        }
        _ => {}
    }
}
