use clap::Parser;

// Effortlessly manage and execute command-line tasks with this powerful system command tool. Create and organize aliases, configurations, and namespaces for enhanced command-line productivity. Streamline your workflow and make complex commands a breeze to remember.
// The command-line tool should enable the creation of aliases to commands, execute the commands and list them.
// Usage: cmd [OPTIONS] [SUBCOMMAND]
//
// Options:
//   -h, --help       Print help information
//   -V, --version    Print version information
//
// Subcommands:
//   new      Create and manage commands
//   config   Create and manage configurations
//   exec     Execute commands
//   list     List commands
//   ns       Create and manage namespaces
//   rm       Remove commands
//   run      Run commands
//   show     Show commands
//   update   Update commands
//   which    Show which commands will be run
//
// See 'cmd help <command>' for more information on a specific command.

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(name = "FILE")]
    pub file: String,

    #[arg(short, long)]
    pub overrides: Option<Vec<String>>,
    // #[command(subcommand)]
    // subcmd: Option<SubCommand>,
}

// #[derive(Parser, Debug)]
// pub enum SubCommand {
//     #[command(about = "Create and manage commands", alias = "n")]
//     New {
//         #[arg(short, long)]
//         name: String,
//         #[arg(short, long)]
//         command: String,
//     },

//     #[command(about = "Execute commands", alias = "e")]
//     Exec { name: String },

//     #[command(about = "List commands", alias = "l")]
//     List,

//     #[command(about = "Remove commands", alias = "r")]
//     Remove { name: String },
// }

// impl SubCommand {
//     fn run(&self) -> Result<(), ShellDeckErrorKind> {
//         match self {
//             SubCommand::New { name, command } => {
//                 // Create a file containing the command
//                 let mut file = fs::File::create(format!(".sd/{name}.sdf"))?;
//                 file.write_all(command.as_bytes())?;
//             }
//             SubCommand::Exec { name } => {
//                 // Read the file containing the command
//                 let cmd = fs::read_to_string(format!(".sd/{name}.sdf")).map_err(|err| {
//                     warn!("Failed to read file: {}", err);
//                     ShellDeckErrorKind::CommandNotFound
//                 })?;
//                 // Execute the command
//                 let mut cmd = Command::new(cmd);
//                 let status = cmd.status()?;
//                 if !status.success() {
//                     return Err(ShellDeckErrorKind::FailedToExecute);
//                 }
//             }
//             SubCommand::List => {
//                 // List all files in the .sd directory
//                 println!("Commands:");
//                 for file in fs::read_dir(".sd/files")? {
//                     println!("\t{}", file?.path().display());
//                 }
//             }
//             SubCommand::Remove { name } => {
//                 // Remove the file containing the command
//                 fs::remove_file(format!(".sd/{name}"))?;
//             }
//         }
//         Ok(())
//     }
// }