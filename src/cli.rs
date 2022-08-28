use crate::package_manager;  // For calling package manager functions
use colored::Colorize;  // For showing colour in the console

// The Cli struct is a super easy way to compact the flags and arguments
pub struct Cli {
    pub flag: String,               // Flags: -S -Syu deps install ..etc..
    pub arguments: Vec<String>,     // Arguments: package1 package2 path/to/file ..etc..
}

impl Cli {
    // Syntax sugar for constructing our Cli
    pub fn new(flag: String, arguments: Vec<String>) -> Cli {
        Cli { flag, arguments }
    }

    // This function processes the command line flags and calls the corresponding package manager
    // function. Here we handle both pacman-like flags (-S, -R, -Syu, etc) and rps-cli-like flags
    // such as update, remove, get, search.
    pub fn process_flags(&self) {

        println!(":: Processing flags...");

        match self.flag.as_str() {
            // Basic necessity commands
            // [NOTE] For basic usage like updating, adding, and removing, packages.
            "-Syu"|"update" => package_manager::update(),
            "-S"|"get" => package_manager::sync(self.arguments.clone()),
            "-R"|"remove"|"rm" => package_manager::remove(self.arguments.clone()),

            // Advanced removal commands
            // [NOTE] For removing dependencies
            "-Rd"|"eliminate" => package_manager::eliminate(self.arguments.clone()),
            "-Rrd"|"clean" => package_manager::clean(),

            // Advanced searching/information commands
            // [NOTE] For searching and getting information on packages
            "-Ss"|"search"|"find" => package_manager::search(self.arguments.clone()),
            "-I"|"info" => package_manager::info(self.arguments.clone()),

            // Advanced viewing commands
            // [NOTE] for viewing packages and dependencies
            "-Q"|"packages"|"list" => package_manager::list_packages(),
            "-Qd" => package_manager::list_dependencies(),
            "-Qds" => package_manager::list_package_dependencies(self.arguments.clone()),
            "deps" => if self.arguments.len() < 1 {
                package_manager::list_dependencies();
            } else {
                package_manager::list_package_dependencies(self.arguments.clone());
            }

            // Install local packages (for packages not uploaded to rps).
            "-U"|"install" => package_manager::local_install(self.arguments.clone()),

            // Help commands
            "-h"|"--help"|"help" => Cli::help(),

            // This is for when the flag given doesn't exist.
            _ => Cli::abort(
                "CLI[process_flags]",
            format!("Could not process flag \"{}\" as it does not match any known pattern!",
                    self.flag.trim().bold())
            ),
        }
    }

    // Shows help message
    pub fn help() {
        let name = "<name>".bold();

        // This is a really hacky wacky way of doing it -- should probably clean this up
        println!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\
            {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\
            {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\
            {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\
            {}{}{}{}{}{}{}{}",
            ":: Owl is a RPS CLI App!\n".bold(),
            ":: You can add, remove, update, and search packages using Owl!\n\n",
            "==>".bright_green()," Usage: owl <flag> [option]\n\n".bold(),
            "[INFO]".yellow(), " Flags are always prefixed with a dash (-Syu for example)\n",
            "[INFO]".yellow(), " Options are always packages you wish to add/remove/search\n\n",
            ":: Example of possible flags\n",
            "  ","update".magenta().underline(),"    |"," -Syu".magenta(),"             = Update Installed Packages\n".bold(),
            "  ","get".blue().underline(),"       |"," -S".blue(),"    ",name,"     = Install a new package\n".bold(),
            "  ","remove".red().underline(),"    |"," -R".red(),"    ",name,"     = Remove a package\n".bold(),
            "  ","eliminate".red().underline()," |"," -Rd".red(),"   ",name,"     = Remove a package and its dependencies\n".bold(),
            "  ","clean".bright_cyan().underline(),"     |"," -Rrd".bright_cyan(),"   ",name,"    = Remove any redundant dependencies\n".bold(),
            "  ","search".white().underline(),"    |"," -Ss".white(),"   ",name,"     = Search for a package\n".bold(),
            "  ","deps".bright_red().underline(),"      |"," -Qds".bright_red(),"  ",name,"     = List dependencies of a package\n".bold(),
            "  ","deps".bright_red().underline(),"      |"," -Qd".bright_red(),"         ","     = List all dependencies\n".bold(),
            "  ","packages".cyan().underline(),"  |"," -Q".cyan(),"          ","     = List all packages\n".bold(),
            "  ","install".bright_green().underline(),"   |"," -U".bright_green(),"    ",name,"     = Install local package\n".bold(),
            "  ","info".green().underline(),"      |"," -I".green(),"    ",name,"     = Get information on a package\n\n".bold(),
            "[INFO]".yellow()," For more information visit the GitHub:   ","https://github.com/hrszpuk/Owl\n".bright_blue().underline(),
            "[INFO]".yellow()," Or join the ReCT Discord:   ","https://discord.gg/Ymm9xGxWZf".bright_blue().underline(),
        );
    }

    // For exiting the program and telling the user what went wrong
    pub fn abort(sector: &str, message: String) {
        println!(
            ":: Issue raised in {}!\n{}{}", sector.purple(),
            "[ABORT] ".red(), message.bright_red(),
        );
        std::process::exit(0);
    }

    // Make a note
    pub fn note(message: String) {
        println!("{} {}","[NOTE]".blue(), message);
    }

    // Send general information
    pub fn info(message: String) {
        println!("{} {}", "[INFO]".yellow(), message);
    }

    // Send an issue (that isn't serious)
    pub fn issue(message: String) {
        println!("{} {}", "[ISSUE]".bright_red(), message);
    }
}


