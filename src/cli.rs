use crate::package_manager;
use colored::Colorize;

pub struct Cli {
    pub flag: String,
    pub arguments: Vec<String>,
}

impl Cli {
    pub fn new(flag: String, arguments: Vec<String>) -> Cli {
        Cli { flag, arguments }
    }

    pub fn process_flags(&self) {

        println!(":: Processing flags...");

        // Check for -Syu
        match self.flag.as_str() {
            "-Syu"|"update" => package_manager::update(),
            "-S"|"get" => package_manager::sync(self.arguments.clone()),
            "-R"|"remove"|"rm" => package_manager::remove(self.arguments.clone()),
            "-Rd"|"eliminate" => package_manager::eliminate(self.arguments.clone()),
            "-Rrd"|"clean" => package_manager::clean(),
            "-Ss"|"search"|"find" => package_manager::search(self.arguments.clone()),
            "-Q"|"packages" => package_manager::list_packages(),
            "-Qd" => package_manager::list_dependencies(),
            "-Qds" => package_manager::list_package_dependencies(self.arguments.clone()),
            "deps" => if self.arguments.len() < 1 {
                package_manager::list_dependencies();
            } else {
                package_manager::list_package_dependencies(self.arguments.clone());
            }
            "-U"|"install" => package_manager::local_install(self.arguments.clone()),
            "-I"|"info" => package_manager::info(self.arguments.clone()),
            "-h"|"--help"|"help" => Cli::help(),
            _ => Cli::abort(
                "CLI[process_flags]",
            format!("Could not process flag \"{}\" as it does not exist!",
                    self.flag.trim().bold())
            ),
        }
    }

    pub fn help() {
        let name = "<name>".bold();
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
            "  ","clean".bright_cyan().underline()," |"," -Rrd".bright_cyan(),"   ",name,"     = Remove any redundant dependencies\n".bold(),
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

    pub fn abort(sector: &str, message: String) {
        println!(
            ":: Issue raised in {}!\n{}{}", sector.purple(),
            "[ABORT] ".red(), message.bright_red(),
        );
        std::process::exit(0);
    }
}


