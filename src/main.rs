use std::env;
use crate::cli::Cli;

mod cli;
mod package_manager;
mod api;


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        Cli::help();
        return;
    }

    let cmd = Cli::new(
        args[1].clone(),
        args[2..].to_owned()
    );

    cmd.process_flags();
}


