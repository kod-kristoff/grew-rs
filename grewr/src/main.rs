use clap::Parser;

mod options;

fn main() {
    let args = options::Args::parse();
    dbg!(&args);
    println!("Hello, world!");
}
