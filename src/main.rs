// Adding needed creates
use std::env;
use crate::cli::Cli;

// Adding other rust modules to the program
mod cli;
mod package_manager;
mod api;
mod database;
mod utils;

const OWL_DATABASE: &str = ".owl.db";
const PACKAGES_PATH: &str = "~/.rect/packages";

fn main() {
    // Fetching command line arguments and showing the help message if only "owl" is used.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        Cli::help();
        return;
    }

    // Now we convert the arguments to a Cli struct
    // owl get package1 package2 package3
    //     ^^^ ^^^^^^^^^^^^^^^^^^^^^^^^^^
    //    Flag         Arguments
    let cmd = Cli::new(
        args[1].clone(),
        args[2..].to_owned()
    );

    // Now we process the flags, this checks the flag and then calls the
    // corresponding functions in package_manager.rs
    cmd.process_flags();  // See cli.rs for more details
}


