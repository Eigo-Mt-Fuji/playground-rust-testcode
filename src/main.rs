#![allow(unused)]
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    // https://doc.rust-lang.org/std/path/struct.PathBuf.html
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };
    // use awesome crate "clap"
    let args = Cli::parse();
    let is_exists: bool =  args.path.as_path().exists();

    println!("args={:?}, the path exists?: {}", args, is_exists);
}
