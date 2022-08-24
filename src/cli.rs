
pub struct Cli {
    pub flag: String,
    pub arguments: Vec<String>,
}

pub fn process_flags(cli: &Cli) {

    // Check for -Syu
    match cli.flag.as_str() {
        "-Syu" => {
            if cli.arguments.len() < 1 {
                println!("Updating packages");
            } else {
                println!("Unexpected number of arguments for -Syu!")
            }
        }
        "-h" | "--help" => {
            if cli.arguments.len() < 1 {
                println!("Help menu [WIP]");
            } else {
                println!("Unexpected number of arguments for --h!")
            }
        }
        "-S" => {
            println!("Install package!");
        }
        "-R" => {
            println!("Remove package!");
        }
        _ => println!("Could not find pattern for {}", cli.flag)
    }

}