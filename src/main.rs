use std::env;
mod cli;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Help menu [WIP]");
        return;
    }

    let cmd = cli::Cli {
        flag: args[1].clone(),
        arguments: args[2..].to_owned(),
    };

    println!("Flag: {:?}", cmd.flag);
    println!("Args: {:?}", cmd.arguments);

    cli::process_flags(&cmd);
}


